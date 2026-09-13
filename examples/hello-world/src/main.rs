use anyhow::Result;
use pixelwalker::players::PlayerManager;
use pixelwalker::prelude::*;

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
pub async fn handle_init(channel: &mut Channel) -> Result<()> {
    channel.send(PlayerInitReceivedPacket::default()).await?;
    channel.send("Hello, World!").await?;
    Ok(())
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
pub async fn handle_ping(ping: &Ping, channel: &mut Channel) -> Result<()> {
    channel.send(*ping).await?;
    Ok(())
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
#[handler(PlayerChatPacket)]
pub async fn handle_chat(chat: &PlayerChatPacket, players: Res<PlayerManager>) -> Result<()> {
    let Some(player_id) = chat.player_id else {
        return Ok(());
    };

    // let player = &players[player_id as usize];
    // println!("{}: {}", player.username(), chat.message);
    println!("{}: {}", player_id, chat.message);
    Ok(())
}

#[handler(WorldBlockPlacedPacket)]
pub async fn handle_block_placed(
    channel: &mut Channel,
    block: &WorldBlockPlacedPacket,
) -> Result<()> {
    // channel
    //     .send(WorldBlockPlacedPacket {
    //         player_id: None,
    //         positions: vec![block.positions[0]],
    //         layer: block.layer,
    //         block_id: block.block_id + 1,
    //         fields: HashMap::new(),
    //     })
    //     .await?;
    Ok(())
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
    let client = client.connect(joinkey).await?.mount([
        handle_init(),
        handle_ping(),
        handle_chat(),
        handle_block_placed(),
    ]);
    let _ = client.listen().await?;

    Ok(())
}
