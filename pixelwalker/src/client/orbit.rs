use crate::state::State;
use pixelwalker_api::pocketbase::client::Auth;

/// The logged-in game state. The client has logged in and entered a world.
/// This starts the websocket connection and blocks the main thread.
pub struct Orbit;

impl State for Orbit {
    type PocketBaseState = Auth;
}
