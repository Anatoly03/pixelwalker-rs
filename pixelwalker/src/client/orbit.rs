use std::any::TypeId;

use crate::connection::{Channel, Handler};
use crate::{Client, state::State};
use anyhow::Result;
use pixelwalker_api::packets::world_packet::Packet;
use pixelwalker_api::packets::{Ping, PlayerInitPacket, PlayerInitReceivedPacket, WorldPacket};
use pixelwalker_api::pocketbase::client::Auth;

/// The logged-in game state. The client has logged in and entered a world.
/// This starts the websocket connection and blocks the main thread.
pub struct Orbit;

impl State for Orbit {
    type PocketBaseState = Auth;
    type SocketStruct = Channel;
    type Handlers = Vec<Box<dyn Handler + Send + Sync>>;
}

impl Client<Orbit> {
    /// Registers event handlers.
    pub fn mount<K>(mut self, handlers: K) -> Self
    where
        K: IntoIterator<Item = Box<dyn Handler + Send + Sync>>,
    {
        self.handlers.extend(handlers);
        self
    }

    /// Starts listening on the websocket channel. This will run indefinitely until
    /// the websocket closes or an error occurs.
    pub async fn listen(mut self) -> Result<Client<super::Orbit>> {
        let () = self
            .channel
            .listen(async |channel: &mut Channel, world_packet: WorldPacket| {
                for handler in self.handlers.iter() {
                    if let Err(e) = handler.call(&world_packet, channel).await {
                        println!("{e:?}");
                    }
                }
            })
            .await?;

        return Ok(Client {
            pocketbase: self.pocketbase,
            channel: self.channel,
            handlers: self.handlers,
        });
    }
}

fn packet_type_id(world_packet: &WorldPacket) -> Option<TypeId> {
    if let Some(packet) = &world_packet.packet {
        let type_id = match packet {
            Packet::Ping(_) => TypeId::of::<Ping>(),
            Packet::PlayerInitPacket(_) => TypeId::of::<PlayerInitPacket>(),
            _ => return None,
        };
        Some(type_id)
    } else {
        None
    }
}
