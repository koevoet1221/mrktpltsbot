pub mod client;
pub mod listing;

use bon::Builder;

use crate::{
    heartbeat::Heartbeat,
    marketplace::marktplaats::{
        client::{MarktplaatsClient, SearchRequest},
        listing::Listing,
    },
    prelude::*,
};

#[derive(Builder)]
pub struct Marktplaats {
    client: MarktplaatsClient,
    search_limit: u32,
    heartbeat: Heartbeat,
}

impl Marktplaats {
    /// Search Marktplaats.
    #[instrument(skip_all)]
    pub async fn search(&self, query: &str) -> Result<Vec<Listing>> {
        info!(query, "Searching…");
        let listings =
            SearchRequest::standard(query, self.search_limit).call_on(&self.client).await?.inner;
        info!(query, n_listings = listings.len(), "Fetched");
        Ok(listings)
    }

    pub async fn check_in(&self) {
        self.heartbeat.check_in().await;
    }
}
