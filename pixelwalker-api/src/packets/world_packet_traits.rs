use crate::packets::{PlayerChatPacket, PlayerInitReceivedPacket};

use super::{Ping, WorldPacket, world_packet::Packet};

/// Protocol messages which can be derived from a world message.
///
/// Any type that implements [FromWorldPacket] must also implement
/// [IntoWorldPacket], but a type that implements [IntoWorldPacket]
/// must not necessarily implement [FromWorldPacket], like [String].
pub trait FromWorldPacket<'p>: From<&'p WorldPacket> {
    fn from_world_packet(packet: &'p WorldPacket) -> Option<&'p Self>;
}

/// Types which can be converted into a protocol message.
pub trait IntoWorldPacket: Into<WorldPacket> {
    fn into_world_packet(&self) -> WorldPacket;
}

impl IntoWorldPacket for Ping {
    fn into_world_packet(&self) -> WorldPacket {
        WorldPacket {
            packet: Some(Packet::Ping(self.clone())),
        }
    }
}

impl Into<WorldPacket> for Ping {
    fn into(self) -> WorldPacket {
        WorldPacket {
            packet: Some(Packet::Ping(self)),
        }
    }
}

impl IntoWorldPacket for PlayerInitReceivedPacket {
    fn into_world_packet(&self) -> WorldPacket {
        WorldPacket {
            packet: Some(Packet::PlayerInitReceived(self.clone())),
        }
    }
}

impl Into<WorldPacket> for PlayerInitReceivedPacket {
    fn into(self) -> WorldPacket {
        WorldPacket {
            packet: Some(Packet::PlayerInitReceived(self)),
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

impl Into<WorldPacket> for &str {
    fn into(self) -> WorldPacket {
        WorldPacket {
            packet: Some(Packet::PlayerChatPacket(PlayerChatPacket {
                player_id: None,
                message: self.to_string(),
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

impl Into<WorldPacket> for String {
    fn into(self) -> WorldPacket {
        WorldPacket {
            packet: Some(Packet::PlayerChatPacket(PlayerChatPacket {
                player_id: None,
                message: self,
            })),
        }
    }
}
