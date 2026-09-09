//! Represents PocketBase records used in PixelWalker collections.

use serde::Deserialize;
use std::fmt::Debug;

/// Retrievable token for joining a PixelWalker world. The API server
/// provides this token and the Game server uses it to verify the player
/// authentication and world id.
///
/// ```https
/// GET https://api.pixelwalker.net/api/joinkey/ROOM_TYPE/ROOM_ID
/// ```
///
/// The variable `ROOM_TYPE` is the constant `pixelwalker`, and `ROOM_ID` is
/// the PocketBase world record ID.
#[derive(Default, Eq, PartialEq, Deserialize, Clone)]
pub struct JoinKey {
    pub token: String,
}

impl Debug for JoinKey {
    /// Formats the value using the given formatter. This is a non-exhaustive print
    /// to prevent underlying access tokens leaking to the console.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JoinKey").finish_non_exhaustive()
    }
}
