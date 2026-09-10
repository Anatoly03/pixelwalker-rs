#[cfg(doc)]
use pixelwalker_api::pocketbase::client::{Auth, NoAuth};

/// A state of the PixelWalker client instance.
pub trait State {
    /// The PocketBase state: Either [Auth] or [NoAuth].
    type PocketBaseState;

    /// The Socket struct, either `()` or `WebSocketStream<MaybeTlsStream<TcpStream>>`
    type SocketStruct;

    /// The type of event handlers. This is a vector of handlers on an orbitting client,
    /// but a Lobby client can have handlers as PocketBase subscriptions.
    type Handlers;
}
