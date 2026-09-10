//! The main entry point for the PixelWalker Rust SDK.
//!
//! This module re-exports all structs and functionalities.

pub mod client;
pub mod connection;
mod join_key;
pub mod players;
pub mod state;
pub(crate) mod vars;

pub use client::Client;
pub use join_key::JoinKey;
pub use pixelwalker_api as api;
pub use pixelwalker_macros as macros;

/// The prelude is a module packaging all useful imports. By simply
/// adding the following line at the top of the Rust file you have
/// access to all requires resources:
///
/// ```
/// pub use pixelwalker::prelude::*;
/// ```
pub mod prelude {
    pub use crate::Client;
    pub use crate::client::{FromResources, Res, Resources};
    pub use crate::connection::Channel;
    pub use pixelwalker_api::packets::*;
    pub use pixelwalker_macros::*;
}
