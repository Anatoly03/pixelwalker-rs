
use crate::connection::{Channel, Handler};
use crate::{Client, state::State};
use anyhow::Result;
use pixelwalker_api::packets::WorldPacket;
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
