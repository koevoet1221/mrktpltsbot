//! Generic and shared stuff for different marketplace.

pub mod item;
mod marktplaats;
mod search_bot;
mod vinted;

use async_trait::async_trait;

pub use self::{
    marktplaats::{Marktplaats, MarktplaatsClient},
    search_bot::SearchBot,
    vinted::{AuthenticationTokens as VintedAuthenticationTokens, Vinted, VintedClient},
};
use crate::{marketplace::item::Item, prelude::*};

#[async_trait]
pub trait Marketplace {
    async fn check_in(&self);

    async fn search_one(&mut self, query: &str) -> Result<Option<Item>>;

    async fn search_many(&mut self, query: &str) -> Result<Vec<Item>>;
}
