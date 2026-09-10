//! The main entry point for the API of the  PixelWalker Rust SDK.
//!
//! This module re-exports all structs and functionalities related to [PocketBase]
//! for communicating with the API Server of PixelWalker.

pub mod packets;
mod collection;
mod models;

pub use collection::PWCollectionQuery;
pub use models::{User, World};
pub use pocketbase_sdk as pocketbase;
pub use pocketbase_sdk::client::Client as PocketBase;
pub use pocketbase_sdk::collections::Collection as PocketBaseCollection;

pub trait PWCollection {
    const COLLECTION_NAME: &'static str;
}
