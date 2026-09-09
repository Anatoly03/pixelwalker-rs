use pixelwalker::{Client, api::User};

fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();
    let client = Client::new().auth_with_email_password()?;
    let auth_id = client.auth_id();
    let bot = client.collection::<User>().view(&auth_id)?;
    let world_id = std::env::var("WORLD_ID").unwrap();
    let client = client.connect(world_id)?;

    Ok(())
}
