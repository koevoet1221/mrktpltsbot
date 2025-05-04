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
}

#[async_trait]
impl Marketplace for Marktplaats {
    async fn check_in(&self) {
        self.heartbeat.check_in().await;
    }

    #[instrument(skip_all)]
    async fn search_one(&mut self, query: &str) -> Result<Option<Item>> {
        SearchRequest::builder()
            .query(query)
            .limit(1)
            .build()
            .call_on(&self.client)
            .await?
            .inner
            .pop()
            .map(Item::try_from)
            .transpose()
    }

    /// Search Marktplaats.
    #[instrument(skip_all)]
    async fn search_many(&mut self, query: &str) -> Result<Vec<Item>> {
        let listings = SearchRequest::builder()
            .query(query)
            .limit(self.search_limit)
            .build()
            .call_on(&self.client)
            .await?
            .inner;
        info!(query, n_listings = listings.len(), "Fetched");
        listings.into_iter().map(TryInto::try_into).collect()
    }
}
