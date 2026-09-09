use pixelwalker_api::pocketbase::client::{Auth, NoAuth};

/// A state of the PixelWalker client instance.
pub trait State {
    /// The PocketBase state: Either [Auth] or [NoAuth].
    type PocketBaseState;
}

/// The guest state, and the initial state. The client is not logged in and
/// not in the lobby. From this state you can either login or join the world
/// as guest.
pub struct Guest;

impl State for Guest {
    type PocketBaseState = NoAuth;
}

/// The logged-in lobby state. The client has logged in and sees the lobby
/// now. It can either join the world as a logged-in user, accept or reject
/// friend requests, and see online worlds.
pub struct Lobby;

impl State for Lobby {
    type PocketBaseState = Auth;
}
