use super::Lobby;
use crate::vars::{AUTH_EMAIL, AUTH_PASSWORD, PIXELWALKER_API_HOST};
use crate::{Client, state::State};
use anyhow::Result;
use pixelwalker_api::PocketBase;
use pixelwalker_api::pocketbase::client::NoAuth;
use std::env::VarError;

/// The guest state, and the initial state. The client is not logged in and
/// not in the lobby. From this state you can either login or join the world
/// as guest.
pub struct Guest;

impl State for Guest {
    type PocketBaseState = NoAuth;
}

impl Client<Guest> {
    /// Creates a new PixelWalker instance, initially not logged in and with
    /// the powers of a guest user.
    pub fn new() -> Client<Guest> {
        Self {
            pocketbase: PocketBase::new(&PIXELWALKER_API_HOST),
        }
    }

    /// Authenticate with email and password. Requires environment variables
    /// `AUTH_EMAIL` and `AUTH_PASSWORD` to be set, which serve as identifier
    /// and secret.
    ///
    /// **Note: You need to include an environment variable scanner yourself,
    /// we recommend using [dotenvy](https://docs.rs/dotenvy/latest/dotenvy/)**
    ///
    /// # Example
    ///
    /// ```no_run,no_test
    /// let _ = dotenvy::dotenv();
    /// let client = Client::new().auth_with_email_password()?;
    /// ```
    ///
    /// An example of a `.env` file can be seen below. This needs to be placed
    /// in the same directory where you start the program.
    ///
    /// ```toml
    /// # Authentication for the user `HELLO`
    /// AUTH_EMAIL = user@example.org
    /// AUTH_PASSWORD = 12345678
    /// ```
    pub fn auth_with_email_password(&self) -> Result<Client<Lobby>> {
        let email: &Result<String, VarError> = &AUTH_EMAIL;
        let pass: &Result<String, VarError> = &AUTH_PASSWORD;

        // Unwrap the environment variable results.
        let (identifier, secret) = match (email, pass) {
            (Ok(e), Ok(p)) => (e, p),
            _ => panic!(
                "calling `auth_with_email_password` requires the following environment variables: `AUTH_EMAIL`, `AUTH_PASSWORD`"
            ),
        };

        // Logs in (This sends a POST request to the server).
        let pocketbase = self
            .pocketbase
            .auth_with_password("users", identifier, secret)?;

        return Ok(Client { pocketbase });
    }
}
