use super::{Ping, WorldPacket, world_packet::Packet};
use crate::packets::*;

/// Protocol messages which can be derived from a world message.
///
/// Any type that implements [FromWorldPacket] must also implement
/// [IntoWorldPacket], but a type that implements [IntoWorldPacket]
/// must not necessarily implement [FromWorldPacket], like [String].
pub trait FromWorldPacket<'p> {
    fn from_world_packet(packet: &'p WorldPacket) -> Option<&'p Self>;
}

/// Types which can be converted into a protocol message.
pub trait IntoWorldPacket {
    fn into_world_packet(&self) -> WorldPacket;
}

impl IntoWorldPacket for Ping {
    fn into_world_packet(&self) -> WorldPacket {
        WorldPacket {
            packet: Some(Packet::Ping(*self)),
        }
    }
}

impl From<Ping> for WorldPacket {
    fn from(val: Ping) -> Self {
        WorldPacket {
            packet: Some(Packet::Ping(val)),
        }
    }
}

impl IntoWorldPacket for PlayerInitReceivedPacket {
    fn into_world_packet(&self) -> WorldPacket {
        WorldPacket {
            packet: Some(Packet::PlayerInitReceived(*self)),
        }
    }
}

impl IntoWorldPacket for WorldBlockPlacedPacket {
    fn into_world_packet(&self) -> WorldPacket {
        WorldPacket {
            packet: Some(Packet::WorldBlockPlacedPacket(self.clone())),
        }
    }
}

impl From<PlayerInitReceivedPacket> for WorldPacket {
    fn from(val: PlayerInitReceivedPacket) -> Self {
        WorldPacket {
            packet: Some(Packet::PlayerInitReceived(val)),
        }
    }
}

impl IntoWorldPacket for &str {
    fn into_world_packet(&self) -> WorldPacket {
        WorldPacket {
            packet: Some(Packet::PlayerChatPacket(PlayerChatPacket {
                player_id: None,
                message: self.to_string(),
            })),
        }
    }
}

impl From<&str> for WorldPacket {
    fn from(val: &str) -> Self {
        WorldPacket {
            packet: Some(Packet::PlayerChatPacket(PlayerChatPacket {
                player_id: None,
                message: val.to_string(),
            })),
        }
    }
}

impl IntoWorldPacket for String {
    fn into_world_packet(&self) -> WorldPacket {
        WorldPacket {
            packet: Some(Packet::PlayerChatPacket(PlayerChatPacket {
                player_id: None,
                message: self.to_string(),
            })),
        }
    }
}

impl From<String> for WorldPacket {
    fn from(val: String) -> Self {
        WorldPacket {
            packet: Some(Packet::PlayerChatPacket(PlayerChatPacket {
                player_id: None,
                message: val,
            })),
        }
    }
}

impl<'p> FromWorldPacket<'p> for Ping {
    fn from_world_packet(packet: &'p WorldPacket) -> Option<&'p Self> {
        packet.packet.as_ref().and_then(|p| match p {
            Packet::Ping(p) => Some(p),
            _ => None,
        })
    }
}

impl<'p> FromWorldPacket<'p> for PlayerInitPacket {
    fn from_world_packet(packet: &'p WorldPacket) -> Option<&'p Self> {
        packet.packet.as_ref().and_then(|p| match p {
            Packet::PlayerInitPacket(p) => Some(p),
            _ => None,
        })
    }
}

impl<'p> FromWorldPacket<'p> for WorldBlockPlacedPacket {
    fn from_world_packet(packet: &'p WorldPacket) -> Option<&'p Self> {
        packet.packet.as_ref().and_then(|p| match p {
            Packet::WorldBlockPlacedPacket(p) => Some(p),
            _ => None,
        })
    }
}
