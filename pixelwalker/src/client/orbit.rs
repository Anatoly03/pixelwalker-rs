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
    /// Registers a resource available to all handlers via `Res<T>`.
    ///
    /// Registering two values of the same concrete type replaces the first.
    pub fn manage<T: Send + Sync + 'static>(mut self, value: T) -> Self {
        self.resources.insert(value);
        self
    }

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
        self.channel.listen(&self.handlers, &self.resources).await?;
        Ok(self)
    }
}
