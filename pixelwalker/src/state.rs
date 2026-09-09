#[cfg(doc)]
use pixelwalker_api::pocketbase::client::{Auth, NoAuth};

/// A state of the PixelWalker client instance.
pub trait State {
    /// The PocketBase state: Either [Auth] or [NoAuth].
    type PocketBaseState;
}
