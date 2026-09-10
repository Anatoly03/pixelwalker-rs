use pixelwalker::{Client, api::packets::*, macros::*};

/// Handles the initialization handshake. This occurs once when the
/// [PlayerInitPacket] is sent by the server. The PixelWalker protocol
/// requires the connection to respond with [PlayerInitReceivedPacket].
///
/// # Example
///
/// ```txt
/// SERVER -> CLIENT:   PlayerInitPacket
/// CLIENT -> SERVER:   PlayerInitReceivedPacket
/// ```
#[handler(PlayerInitPacket)]
pub fn handle_init(channel: Channel) {
    channel.send(PlayerInitReceivedPacket::default());
}

/// Handles the high-level ping pong. The PixelWalker protocol requires
/// a connection to respond with [PlayerInitReceivedPacket].
///
/// # Example
///
/// ```txt
/// SERVER -> CLIENT:   Ping
/// CLIENT -> SERVER:   Ping
/// ```
#[handler(Ping)]
pub fn handle_ping(ping: &Ping, channel: Channel) {
    channel.send(ping);
}

/// The entry point of the bot application. It runs through all steps -
/// authentication, connection and the player init handshake. Afterwards
/// it just keeps sending pinging to keep the connection alive.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();
    let client = Client::new().auth_with_email_password()?;
    let world_id = std::env::var("WORLD_ID").unwrap();
    let joinkey = client.get_join_key(world_id).await?;
    let client = client
        .connect(joinkey)
        .await?;
        // .mount(vec![handle_init, handle_ping]);
    let _ = client.listen().await?;

    Ok(())
}
