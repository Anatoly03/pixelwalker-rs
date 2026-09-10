//! Contains the PixelWalker [Client].

mod guest;
mod lobby;
mod orbit;
mod resource;

use crate::state::State;
use base64::{Engine, engine::general_purpose::STANDARD_NO_PAD};
pub use guest::Guest;
pub use lobby::Lobby;
pub use orbit::Orbit;
use pixelwalker_api::PocketBase;
use pixelwalker_api::pocketbase::client::Auth;
pub use resource::*;
use serde_json::Value;
use std::fmt::Debug;

/// A PixelWalker client instance.
pub struct Client<S: State> {
    /// The [PocketBase] client instance, which is used for communication with
    /// the API server. It handles authentication and querying the database.
    pub(crate) pocketbase: PocketBase<S::PocketBaseState>,

    /// The websocket instance, which is used for communication with the game
    /// server.
    pub(crate) channel: S::SocketStruct,

    /// The registered event handlers.
    pub(crate) handlers: S::Handlers,

    /// Static resources within the application state.
    pub(crate) resources: Resources,
}

impl<S: State> Debug for Client<S> {
    /// Formats the value using the given formatter. This is a non-exhaustive print
    /// to prevent underlying access tokens leaking to the console.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Client").finish_non_exhaustive()
    }
}

/// Utility function get the id of the authenticated record by decoding
/// the JWT data and getting the `id` field.
pub fn get_pocketbase_auth_id(pocketbase: &PocketBase<Auth>) -> String {
    let token = pocketbase
        .auth_token
        .as_ref()
        .expect("auth token should be set");
    let payload = token.split('.').nth(1).unwrap();
    let decoded = STANDARD_NO_PAD.decode(payload).unwrap();
    let data: Value = serde_json::from_slice(&decoded).expect("auth token should not be corrupted");
    data["id"].as_str().unwrap().to_owned()
}
