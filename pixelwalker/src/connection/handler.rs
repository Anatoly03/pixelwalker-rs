use crate::connection::Channel;
use futures_util::future::BoxFuture;
use pixelwalker_api::packets::WorldPacket;

/// The trait which all event handlers of the PixelWalker connection implement.
pub trait Handler: Send + Sync {
    /// Invokes the event handler with the channel and event packet arguments.
    fn call<'a>(
        &'a self,
        packet: &'a WorldPacket,
        channel: &'a mut Channel,
    ) -> BoxFuture<'a, anyhow::Result<()>>;
}
