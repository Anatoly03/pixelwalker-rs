use crate::{Client, state::State};
use base64::{Engine, engine::general_purpose::STANDARD_NO_PAD};
use pixelwalker_api::pocketbase::client::Auth;
use pixelwalker_api::{PWCollection, PWCollectionQuery};
use serde::de::DeserializeOwned;
use serde_json::Value;

/// The logged-in lobby state. The client has logged in and sees the lobby
/// now. It can either join the world as a logged-in user, accept or reject
/// friend requests, and see online worlds.
pub struct Lobby;

impl State for Lobby {
    type PocketBaseState = Auth;
}

impl Client<Lobby> {
    /// Decodes the identifier of the logged in user record. In other words,
    /// when you log in into the user account `x`, this is the id of `x`.
    ///
    /// ### Example
    ///
    /// ```no_run,no_test
    /// let _ = dotenvy::dotenv();
    /// let client = Client::new().auth_with_email_password()?;
    /// let auth_id = client.auth_id();
    /// let bot = client.collection::<User>().view(auth_id)?;
    /// println!("Logged in as: {}", bot.username);
    /// ```
    pub fn auth_id(&self) -> String {
        let token = self
            .pocketbase
            .auth_token
            .as_ref()
            .expect("auth token should be set");
        let payload = token.split('.').skip(1).next().unwrap();
        let decoded = STANDARD_NO_PAD.decode(payload).unwrap();
        let data: Value =
            serde_json::from_slice(&decoded).expect("auth token should not be corrupted");
        data["id"].as_str().unwrap().to_owned()
    }

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
