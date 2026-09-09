use crate::{Client, state::State};
use anyhow::Result;
use futures_util::{SinkExt, StreamExt as _};
use pixelwalker_api::packets::world_packet::Packet;
use pixelwalker_api::packets::{Ping, PlayerInitPacket, PlayerInitReceivedPacket, WorldPacket};
use pixelwalker_api::pocketbase::client::Auth;
use prost::Message as _;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message as WsMessage;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

/// The logged-in game state. The client has logged in and entered a world.
/// This starts the websocket connection and blocks the main thread.
pub struct Orbit;

impl State for Orbit {
    type PocketBaseState = Auth;
    type SocketStruct = WebSocketStream<MaybeTlsStream<TcpStream>>;
}

impl Client<Orbit> {
    /// Starts listening on the websocket channel. This will run indefinitely until
    /// the websocket closes or an error occurs.
    pub async fn listen(mut self) -> Result<Client<super::Orbit>> {
        loop {
            // Fetch the next message. Since this stream should run indefinitely, if it is
            // [None] or an error in the optional, we halt.
            let message = match self.websocket.next().await {
                Some(Ok(message)) => message,
                Some(Err(e)) => {
                    println!("Error: {e}");
                    break;
                }
                None => break,
            };

            // Detect message type and only pass binary messages to a special handler.
            match message {
                WsMessage::Text(text) => {
                    println!("Binary: {text}");
                }
                WsMessage::Binary(message) => {
                    let world_packet = WorldPacket::decode(&message[..])?;
                    println!("World Packet: {world_packet:?}");

                    // temporary event handler
                    // TODO refactor this into an event system.
                    match world_packet.packet {
                        Some(Packet::Ping(Ping {})) => {
                            let mut buf = Vec::new();
                            if (WorldPacket {
                                packet: Some(Packet::Ping(Ping {})),
                            })
                            .encode(&mut buf)
                            .is_ok()
                            {
                                // let mut channel = channel.lock().await;
                                let _ = self
                                    .websocket
                                    .send(tokio_tungstenite::tungstenite::Message::Binary(
                                        buf.into(),
                                    ))
                                    .await;
                            }
                        }
                        Some(Packet::PlayerInitPacket(PlayerInitPacket { .. })) => {
                            let mut buf: Vec<u8> = Vec::new();
                            if (WorldPacket {
                                packet: Some(Packet::PlayerInitReceived(
                                    PlayerInitReceivedPacket {},
                                )),
                            })
                            .encode(&mut buf)
                            .is_ok()
                            {
                                // let mut channel = channel.lock().await;
                                let _ = self
                                    .websocket
                                    .send(tokio_tungstenite::tungstenite::Message::Binary(
                                        buf.into(),
                                    ))
                                    .await;
                            }
                        }
                        _ => {}
                    }
                }
                WsMessage::Close(Some(frame)) => {
                    println!("Close Frame: {frame:?}");
                }
                _ => {}
            }
        }

        return Ok(Client {
            pocketbase: self.pocketbase,
            websocket: self.websocket,
        });
    }
}
