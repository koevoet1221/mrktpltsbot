//! Generic and shared stuff for different marketplace.

pub mod item;
mod marktplaats;
mod search_bot;
mod vinted;

use std::any::type_name;

use async_trait::async_trait;

#[cfg(test)]
pub use self::vinted::AuthenticationTokens as VintedAuthenticationTokens;
pub use self::{
    marktplaats::{Marktplaats, MarktplaatsClient},
    search_bot::SearchBot,
    vinted::{Vinted, VintedClient},
};
use crate::{marketplace::item::Item, prelude::*};

#[async_trait]
pub trait Marketplace {
    async fn check_in(&self);

    async fn search_one(&mut self, query: &str) -> Result<Option<Item>>;

    async fn search_many(&mut self, query: &str) -> Result<Vec<Item>>;

    async fn search_many_and_extend_infallible(&mut self, query: &str, into: &mut Vec<Item>) {
        match self.search_many(query).await {
            Ok(marktplaats_items) => {
                into.extend(marktplaats_items);
                self.check_in().await;
            }
            Err(error) => {
                error!("‼️ Failed to search on {}: {error:#}", type_name::<Self>());
            }
        }
    }
}
