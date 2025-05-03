pub mod client;
pub mod listing;

use bon::Builder;

use crate::{
    heartbeat::Heartbeat,
    marketplace::{
        item::Item,
        marktplaats::client::{MarktplaatsClient, SearchRequest},
    },
    prelude::*,
};

#[must_use]
#[derive(Clone, Builder)]
pub struct Marktplaats {
    client: MarktplaatsClient,
    search_limit: u32,
    heartbeat: Heartbeat,
}

impl Marktplaats {
    /// Search Marktplaats.
    #[instrument(skip_all)]
    pub async fn search(&self, query: &str) -> Result<Vec<Item>> {
        info!(query, "Searching…");
        let listings =
            SearchRequest::standard(query, self.search_limit).call_on(&self.client).await?.inner;
        info!(query, n_listings = listings.len(), "Fetched");
        listings.into_iter().map(TryInto::try_into).collect()
    }

    pub async fn check_in(&self) {
        self.heartbeat.check_in().await;
    }
}
