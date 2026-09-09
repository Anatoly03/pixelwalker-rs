//! Contains the PixelWalker [Client].

mod guest;
mod lobby;
mod orbit;

use crate::state::State;
pub use lobby::Lobby;
pub use guest::Guest;
use pixelwalker_api::PocketBase;
use std::fmt::Debug;

/// A PixelWalker client instance.
pub struct Client<S: State> {
    /// The [PocketBase] client instance, which is used for communication with
    /// the API server. It handles authentication and querying the database.
    pub(crate) pocketbase: PocketBase<S::PocketBaseState>,
}

impl<S: State> Debug for Client<S> {
    /// Formats the value using the given formatter. This is a non-exhaustive print
    /// to prevent underlying access tokens leaking to the console.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Client").finish_non_exhaustive()
    }
}
