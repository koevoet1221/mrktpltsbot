use bon::Builder;

use crate::{
    db::{Db, key_values::KeyValues},
    marketplace::{
        item::Item,
        vinted::{
            client::{AuthenticationTokens, VintedClient},
            search::SearchRequest,
        },
    },
    prelude::*,
};

pub mod client;
pub mod error;
pub mod search;

pub use self::error::Error as VintedError;

#[derive(Clone, Builder)]
pub struct Vinted {
    client: VintedClient,
    search_limit: u32,
    db: Db,
}

impl Vinted {
    pub async fn search_one(&mut self, query: &str) -> Result<Option<Item>> {
        Ok(self.search(query, 1).await?.pop())
    }

    pub async fn search_many(&mut self, query: &str) -> Result<Vec<Item>> {
        self.search(query, self.search_limit).await
    }

    #[instrument(skip_all)]
    pub async fn search(&mut self, query: &str, limit: u32) -> Result<Vec<Item>> {
        let Some(auth_tokens) =
            KeyValues(&mut *self.db.connection().await).fetch::<AuthenticationTokens>().await?
        else {
            warn!("Run `mrktpltsbot vinted authenticate` to use Vinted search");
            return Ok(vec![]);
        };
        let result = SearchRequest::builder()
            .search_text(query)
            .per_page(limit)
            .access_token(&auth_tokens.access)
            .build()
            .call_on(&self.client)
            .await;
        let search_results = match result {
            Ok(search_results) => search_results,
            Err(VintedError::Unauthorized) => {
                let auth_tokens = self.refresh_tokens(&auth_tokens.refresh).await?;
                SearchRequest::builder()
                    .search_text(query)
                    .per_page(limit)
                    .access_token(&auth_tokens.access)
                    .build()
                    .call_on(&self.client)
                    .await?
            }
            Err(error) => {
                bail!("failed to search: {error:#}");
            }
        };
        Ok(search_results.items.into_iter().map(Item::from).collect())
    }

    #[instrument(skip_all)]
    async fn refresh_tokens(&mut self, refresh_token: &str) -> Result<AuthenticationTokens> {
        let mut db = self.db.connection().await;
        let mut key_values = KeyValues(&mut db);
        match self.client.refresh_token(refresh_token).await {
            Ok(auth_tokens) => {
                key_values.upsert(&auth_tokens).await?;
                Ok(auth_tokens)
            }
            Err(error) => {
                key_values.delete::<AuthenticationTokens>().await?;
                bail!("failed to refresh the authentication token, disabling Vinted: {error:#}");
            }
        }
    }
}
