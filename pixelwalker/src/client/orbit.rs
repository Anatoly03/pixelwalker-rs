use crate::connection::Channel;
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
}

impl Client<Orbit> {
    /// Starts listening on the websocket channel. This will run indefinitely until
    /// the websocket closes or an error occurs.
    pub async fn listen(mut self) -> Result<Client<super::Orbit>> {
        let () = self.channel.listen(async |channel: &mut Channel, world_packet: WorldPacket| {
            println!("World Packet: {world_packet:?}");

            match world_packet.packet {
                Some(Packet::Ping(Ping {})) => {
                    let _ = channel.send(Ping::default()).await;
                }
                Some(Packet::PlayerInitPacket(PlayerInitPacket { .. })) => {
                    let a = channel.send(PlayerInitReceivedPacket::default()).await;
                    println!("Response: {a:?}");
                }
                _ => {}
            }

            return ();
        }).await?;

        return Ok(Client {
            pocketbase: self.pocketbase,
            channel: self.channel,
        });
    }
}
