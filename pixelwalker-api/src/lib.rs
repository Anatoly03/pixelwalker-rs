//! The main entry point for the API of the  PixelWalker Rust SDK.
//!
//! This module re-exports all structs and functionalities related to [PocketBase]
//! for communicating with the API Server of PixelWalker.

mod collection;
mod users;
mod worlds;

pub use collection::PWCollectionQuery;
pub use pocketbase_sdk as pocketbase;
pub use pocketbase_sdk::client::Client as PocketBase;
pub use pocketbase_sdk::collections::Collection as PocketBaseCollection;
pub use users::User;
pub use worlds::World;

pub trait PWCollection {
    const COLLECTION_NAME: &'static str;
}
