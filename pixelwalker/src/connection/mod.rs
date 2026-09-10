mod handler;

use anyhow::Result;
use futures_util::{SinkExt, StreamExt as _};
pub use handler::Handler;
use pixelwalker_api::packets::{IntoWorldPacket, WorldPacket};
use prost::Message;
use tokio::net::TcpStream;
use tokio::signal;
use tokio_tungstenite::tungstenite::Message as WsMessage;
use tokio_tungstenite::tungstenite::Message::Binary;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

pub struct Channel {
    /// The websocket instance.
    websocket: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl Channel {
    /// Starts an indefinite loop listening to game server messages and filtering
    /// for the binary [WorldPacket].
    pub(crate) async fn listen<P>(&mut self, mut handler: P) -> Result<()>
    where
        P: AsyncFnMut(&mut Channel, WorldPacket),
    {
        loop {
            tokio::select! {
                // Stop the websocket on interrupt.
                _ = signal::ctrl_c() => {
                    self.websocket.close(None).await?;
                    #[cfg(feature = "logs")]
                    {
                        use colored::Colorize;

                        println!("{}","Interrupted".red().bold());
                        println!(" {}", "└ Connection closed by our side".bright_black());
                    }
                    return Ok(());
                }
                // Accept next message.
                incoming_message = self.websocket.next() => {
                    // Fetch next message, if error or none, return early.
                    let message = match incoming_message {
                        Some(result) => result?,
                        None => return Ok(()),
                    };

                    // Determine websocket message type.
                    match message {
                        WsMessage::Text(_text) => {
                            #[cfg(feature = "logs")]
                            {
                                use colored::Colorize;

                                println!("{}","Received Text Message".cyan().bold());
                                println!(" {} {}", "└ Reason:   ".bright_black(), text.bright_black());
                            }
                        }
                        WsMessage::Binary(message) => {
                            let world_packet = WorldPacket::decode(&message[..])?;
                            handler(self, world_packet).await;
                        }
                        WsMessage::Close(Some(_frame)) => {
                            #[cfg(feature = "logs")]
                            {
                                use colored::Colorize;

                                println!("{}","Connection Closed".red().bold());
                                println!(" {} {}", "└ Reason:   ".bright_black(), frame.reason.red());
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    /// Send an event to the game server. Anything that implements [IntoWorldPacket]
    /// can be sent to the game server.
    ///
    /// This method will first convert any type into [WorldPacket], which will be
    /// encoded into a binary with [prost]. If successful, this binary is then sent
    /// to the server.
    ///
    /// # Example
    ///
    /// ```no_run,no_test
    /// channel.send(Ping::default());
    /// ```
    ///
    /// All protocol messages which can be sent to the server implement [IntoWorldPacket].
    /// This type safety avoids sending packets which are not understood by the
    /// server.
    ///
    /// # Example: Chat Messages
    ///
    /// ```no_run,no_test
    /// channel.send("/title Hello, World!");
    /// ```
    ///
    /// String types implement [IntoWorldPacket] and will be converted into a
    /// chat packet.
    pub async fn send<K: IntoWorldPacket>(&mut self, message: K) -> Result<()> {
        let mut buf: Vec<u8> = Vec::new();
        let world_packet: WorldPacket = message.into_world_packet();
        // println!("SENDING TO SERVER: {world_packet:?}");
        world_packet.encode(&mut buf)?;
        self.websocket.send(Binary(buf.into())).await?;
        Ok(())
    }
}

impl From<WebSocketStream<MaybeTlsStream<TcpStream>>> for Channel {
    /// Converts a [WebSocketStream] into a [Channel].
    fn from(websocket: WebSocketStream<MaybeTlsStream<TcpStream>>) -> Self {
        Self { websocket }
    }
}
