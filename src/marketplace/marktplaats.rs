mod client;
mod listing;

use async_trait::async_trait;
use bon::Builder;

use self::client::SearchRequest;
pub use self::{client::MarktplaatsClient, listing::Listings};
use crate::{
    db::SearchQuery,
    heartbeat::Heartbeat,
    marketplace::{Marketplace, NormalisedQuery, item::Item},
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

    /// Search Marktplaats.
    async fn search(&mut self, query: &SearchQuery) -> Result<Vec<Item>> {
        let query = NormalisedQuery::parse(&query.text);
        let search_text = query.unparse();
        let listings = SearchRequest::builder()
            .query(&search_text)
            .limit(self.search_limit)
            .search_in_title_and_description(self.search_in_title_and_description)
            .build()
            .call_on(&self.client)
            .await?
            .inner;
        info!(search_text, n_listings = listings.len(), "🛍️ Fetched");
        listings.into_iter().map(TryInto::try_into).collect()
    }
}
