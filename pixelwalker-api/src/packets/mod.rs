//! Protobuf packet definitions for PixelWalker.
//!
//! These packets are used for communication with the world server and
//! are generated from the `world.proto` file.

mod world_packet_traits;
pub use world_packet_traits::{FromWorldPacket, IntoWorldPacket};

include!(concat!(env!("OUT_DIR"), "/world_packets.rs"));

