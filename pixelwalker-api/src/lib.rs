//! The main entry point for the API of the  PixelWalker Rust SDK.
//! 
//! This module re-exports all structs and functionalities related to [PocketBase]
//! for communicating with the API Server of PixelWalker.

pub use pocketbase_sdk as pocketbase;
pub use pocketbase_sdk::client::Client as PocketBase;
pub use pocketbase_sdk::collections::Collection;
