use once_cell::sync::Lazy;
use std::env::VarError;

/// The PixelWalker API hostname. It can be set with the environment variable
/// `PIXELWALKER_API_HOST`, otherwise it will fall back to a default. The host
/// can optionally end with a slash.
///
/// ### Default
///
/// ```rs
/// "https://api.pixelwalker.net/"
/// ```
pub static PIXELWALKER_API_HOST: Lazy<String> = Lazy::new(|| {
    std::env::var("PIXELWALKER_API_HOST")
        .map(|s| {
            if s.ends_with("/") {
                return s[..s.len() - 1].to_string();
            }
            s
        })
        .unwrap_or("https://api.pixelwalker.net".into())
});

/// The PixelWalker game server hostname. It can be set with the environment
/// variable `PIXELWALKER_GAME_HOST`, otherwise it will fall back to a default.
/// The host can optionally end with a slash.
///
/// ### Default
///
/// ```rs
/// "wss://game.pixelwalker.net"
/// ```
pub static PIXELWALKER_GAME_HOST: Lazy<String> = Lazy::new(|| {
    std::env::var("PIXELWALKER_GAME_HOST")
        .map(|s| {
            if s.ends_with("/") {
                return s[..s.len() - 1].to_string();
            }
            s
        })
        .unwrap_or("wss://server.pixelwalker.net".into())
});

/// The environment variable `AUTH_EMAIL`.
pub static AUTH_EMAIL: Lazy<Result<String, VarError>> = Lazy::new(|| std::env::var("AUTH_EMAIL"));

/// The environment variable `AUTH_PASSWORD`.
pub static AUTH_PASSWORD: Lazy<Result<String, VarError>> =
    Lazy::new(|| std::env::var("AUTH_PASSWORD"));
