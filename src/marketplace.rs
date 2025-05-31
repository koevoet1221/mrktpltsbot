//! Generic and shared stuff for different marketplace.

pub mod item;
mod marktplaats;
mod search;
mod search_bot;
mod vinted;

use std::any::type_name;

use async_trait::async_trait;

pub use self::{
    marktplaats::{Marktplaats, MarktplaatsClient},
    search::NormalisedQuery,
    search_bot::SearchBot,
    vinted::{AuthenticationTokens as VintedAuthenticationTokens, Vinted, VintedClient},
};
use crate::{db::SearchQuery, marketplace::item::Item, prelude::*};

#[async_trait]
pub trait Marketplace {
    async fn check_in(&self);

    async fn search_and_extend_infallible(
        &mut self,
        query: &SearchQuery,
        limit: Option<usize>,
        into: &mut Vec<Item>,
    ) {
        match self.search(query).await {
            Ok(mut items) => {
                if let Some(limit) = limit {
                    items.truncate(limit);
                }
                into.extend(items);
            }
            Err(error) => {
                error!("‼️ Failed to search on {}: {error:#}", type_name::<Self>());
            }
        }
    }

    async fn search(&mut self, query: &SearchQuery) -> Result<Vec<Item>>;
}
