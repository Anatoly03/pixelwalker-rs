use crate::{PWCollection, PocketBase};
use anyhow::Result;
use pocketbase_sdk::{client::Auth, records::RecordsListRequestBuilder};
use serde::de::DeserializeOwned;
use std::marker::PhantomData;

/// The magic number `1000` which is used in the first list request. This tries
/// to find the maximal amount of entries per page.
pub const PER_PAGE_START: i32 = 1000;

/// A collection query created from a [PocketBase] client.
pub struct PWCollectionQuery<'a, T: PWCollection> {
    pub(crate) client: &'a PocketBase<Auth>,
    pub(crate) phantom: PhantomData<T>,
    pub(crate) sort_options: Vec<String>,
}

impl<'a, T> PWCollectionQuery<'a, T>
where
    T: PWCollection + Default + DeserializeOwned,
{
    /// Create a new collection query from a [PocketBase] client.
    ///
    /// # Example
    ///
    /// ```no_run,no_test
    /// let client = Client::new().auth_with_email_password()?;
    /// let collection = client.collection::<World>();
    /// ```
    pub fn new(client: &'a PocketBase<Auth>) -> Self {
        Self {
            client,
            phantom: PhantomData,
            sort_options: vec![],
        }
    }

    /// Pushes a new sort index to the query builder. The sort direction can be
    /// optionally specified with a leading `+` (ascending, default) or a `-`
    /// (descending).
    ///
    /// # Example
    ///
    /// ```no_run,no_test
    /// let client = Client::new().auth_with_email_password()?;
    /// // This collection query will sort by `woots` in descending order
    /// // and sort ties by their `plays` count.
    /// let query = client.collection::<World>().sort("-woots").sort("-plays");
    /// ```
    pub fn sort<K: ToString>(&mut self, sort_option: K) -> &mut Self {
        self.sort_options.push(sort_option.to_string());
        self
    }

    /// Creates an internal list request builder, with the corresponding
    /// sort options and filters.
    fn list(&self) -> RecordsListRequestBuilder<'_> {
        let mut list = self.client.records(T::COLLECTION_NAME).list();
        if self.sort_options.len() > 0 {
            list = list.sort(&self.sort_options.join(","));
        }
        list
    }

    /// Returns the length of the collection (= amount of total items).
    ///
    /// # Example
    ///
    /// ```no_run,no_test
    /// let client = Client::new().auth_with_email_password()?;
    /// // Print statistics.
    /// println!("- Total Worlds: {}", client.collection::<World>().len()?);
    /// println!("- Total Users: {}", client.collection::<User>().len()?);
    /// ```
    pub fn len(&self) -> Result<usize> {
        let response = self.list().per_page(0).call::<T>()?;
        Ok(response.total_items as usize)
    }

    /// Fetches all elements of the query.
    ///
    /// This is an expensive operation and not recommended to be called.
    /// **Calling this function too often can be seen as Denial-Of-Service by
    /// the server and could get you banned.**
    pub fn take(&self, amount: usize) -> Result<Vec<T>> {
        let mut items = Vec::with_capacity(amount);

        let mut response = self.list().per_page(PER_PAGE_START).call::<T>()?;
        items.extend(response.items.into_iter().take(amount));
        let mut finish = false;

        while !finish {
            response = self
                .list()
                .per_page(response.per_page)
                .page(response.page + 1)
                .call::<T>()?;
            finish = response.items.len() != response.per_page as usize;

            if amount - items.len() == 0 {
                break;
            }

            items.extend(response.items.into_iter().take(amount - items.len()));
        }

        Ok(items)
    }

    /// Fetches all elements of the query.
    ///
    /// This is an expensive operation and not recommended to be called.
    /// **Calling this function too often can be seen as Denial-Of-Service by
    /// the server and could get you banned.**
    pub fn collect(&self) -> Result<Vec<T>> {
        let mut items = vec![];

        let mut response = self.list().per_page(PER_PAGE_START).call::<T>()?;
        items.extend(response.items);
        let mut finish = false;

        while !finish {
            response = self
                .list()
                .per_page(response.per_page)
                .page(response.page + 1)
                .call::<T>()?;
            finish = response.items.len() != response.per_page as usize;
            items.extend(response.items);
        }

        Ok(items)
    }
}
