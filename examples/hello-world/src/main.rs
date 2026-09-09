use pixelwalker::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();
    let client = Client::new().auth_with_email_password()?;
    let world_id = std::env::var("WORLD_ID").unwrap();
    let joinkey = client.get_join_key(world_id).await?;
    let client = client.connect(joinkey).await?;
    let _ = client.listen().await?;

    Ok(())
}
