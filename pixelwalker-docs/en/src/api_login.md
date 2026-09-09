# Logging In

When you open PixelWalker, assuming you haven't logged in yet, you see an authentication screen. In this software kit we refer to this as the `Guest` state. From here, you would first have to "login" into the `Lobby` state. The code snippet for logging in can be seen below, with the recommended dependencies `anyhow` and `dotenvy`.

```rust
use pixelwalker::Client;

fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();
    let client = Client::new().auth_with_email_password()?;
    Ok(())
}
```

The decision was made to move all authentication-related secrets to environments. With the [`dotenvy`](https://docs.rs/dotenvy/latest/dotenvy/) crate you can read out the special `.env` file. An example of how such a file is structed can be seen below.

```toml
# Authentication for the user `HELLO`
AUTH_EMAIL = user@example.org
AUTH_PASSWORD = 12345678
```

And this pretty much summarizes how to log in. Currently you can only log in with email and password. 
