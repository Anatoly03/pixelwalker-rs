//! The main entry point for the API of the  PixelWalker Rust SDK.
//!
//! This module re-exports all structs and functionalities related to [PocketBase]
//! for communicating with the API Server of PixelWalker.

mod collection;
mod models;

pub use collection::PWCollectionQuery;
pub use models::{User, World};
pub use pocketbase_sdk as pocketbase;
pub use pocketbase_sdk::client::Client as PocketBase;
pub use pocketbase_sdk::collections::Collection as PocketBaseCollection;

/// Protobuf packet definitions for PixelWalker.
/// 
/// These packets are used for communication with the world server and
/// are generated from the `world.proto` file.
pub mod packets {
    include!(concat!(env!("OUT_DIR"), "/world_packets.rs"));
}

pub trait PWCollection {
    const COLLECTION_NAME: &'static str;
}
