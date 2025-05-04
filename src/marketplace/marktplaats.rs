mod client;
mod listing;

use async_trait::async_trait;
use bon::Builder;

use self::client::SearchRequest;
pub use self::{client::MarktplaatsClient, listing::Listings};
use crate::{
    heartbeat::Heartbeat,
    marketplace::{Marketplace, item::Item},
    prelude::*,
};

#[must_use]
#[derive(Clone, Builder)]
pub struct Marktplaats {
    client: MarktplaatsClient,
    search_limit: u32,
    heartbeat: Heartbeat,
    search_in_title_and_description: bool,
}

#[async_trait]
impl Marketplace for Marktplaats {
    async fn check_in(&self) {
        self.heartbeat.check_in().await;
    }

    async fn search_one(&mut self, query: &str) -> Result<Option<Item>> {
        SearchRequest::builder()
            .query(query)
            .limit(1)
            .search_in_title_and_description(self.search_in_title_and_description)
            .build()
            .call_on(&self.client)
            .await?
            .inner
            .pop()
            .map(Item::try_from)
            .transpose()
    }

    /// Search Marktplaats.
    async fn search_many(&mut self, query: &str) -> Result<Vec<Item>> {
        let listings = SearchRequest::builder()
            .query(query)
            .limit(self.search_limit)
            .search_in_title_and_description(self.search_in_title_and_description)
            .build()
            .call_on(&self.client)
            .await?
            .inner;
        info!(query, n_listings = listings.len(), "🛍️ Fetched");
        listings.into_iter().map(TryInto::try_into).collect()
    }
}
