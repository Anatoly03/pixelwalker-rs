//! Contains the PixelWalker [Client].

use crate::{
    state::{Guest, Lobby, State},
    vars::{AUTH_EMAIL, AUTH_PASSWORD, PIXELWALKER_API_HOST},
};
use anyhow::Result;
use pixelwalker_api::{PWCollection, PWCollectionQuery, PocketBase};
use serde::de::DeserializeOwned;
use std::{env::VarError, fmt::Debug};

/// A PixelWalker client instance.
pub struct Client<S: State> {
    /// The [PocketBase] client instance, which is used for communication with
    /// the API server. It handles authentication and querying the database.
    pub(crate) pocketbase: PocketBase<S::PocketBaseState>,
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

impl Client<Lobby> {
    /// Creates a new collection query builder.
    ///
    /// ### Example
    ///
    /// ```no_run,no_test
    /// let _ = dotenvy::dotenv();
    /// let client = Client::new().auth_with_email_password()?;
    /// let total_worlds = client.collection::<World>().len()?;
    /// println!("Total Public PixelWalker Worlds: {total_worlds}");
    /// ```
    ///
    /// ### Explanation
    ///
    /// The source code is a bit unreadable due to the flood of generics, so this
    /// section will attempt at giving an explanation:
    ///
    /// The [PWCollectionQuery] struct takes the [PocketBase] client as a reference
    /// and extracts the name of the PocketBase collection from the generic. The
    /// latter can be done because the [PWCollection] trait defines the collection
    /// name as a constant.
    ///
    /// The lifetime is used to indicate that the application state should outlive
    /// the query.
    pub fn collection<'a, T>(&'a self) -> PWCollectionQuery<'a, T>
    where
        T: PWCollection + Default + DeserializeOwned,
    {
        PWCollectionQuery::<'a, T>::new(&self.pocketbase)
    }
}

impl<S: State> Debug for Client<S> {
    /// Formats the value using the given formatter. This is a non-exhaustive print
    /// to prevent underlying access tokens leaking to the console.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Client").finish_non_exhaustive()
    }
}
