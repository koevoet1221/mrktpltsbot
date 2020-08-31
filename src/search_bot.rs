use crate::marktplaats;
use crate::marktplaats::{SearchListing, SearchResponse};
use crate::prelude::*;
use crate::redis::{check_seen, pick_random_subscription};

const POLL_INTERVAL: Duration = Duration::from_secs(60);
const SEARCH_LIMIT: &str = "10";

pub struct Bot {
    redis: RedisConnection,
}

impl Bot {
    pub fn new(redis: RedisConnection) -> Self {
        Self { redis }
    }

    /// Spawn the bot.
    pub async fn spawn(mut self) -> Result {
        info!("Running the search bot…");
        loop {
            if let Some((chat_id, query)) = pick_random_subscription(&mut self.redis).await? {
                log_result(self.check_subscription(chat_id, query).await);
            }
            task::sleep(POLL_INTERVAL).await;
        }
    }

    async fn check_subscription(&mut self, chat_id: i64, query: String) -> Result {
        info!("Checking `{}`…", query);
        self.handle_search_result(chat_id, marktplaats::search(&query, SEARCH_LIMIT).await?)
            .await?;
        Ok(())
    }

    async fn handle_search_result(&mut self, chat_id: i64, response: SearchResponse) -> Result {
        info!("{} search results.", response.listings.len());

        for listing in response.listings.iter() {
            if check_seen(&mut self.redis, chat_id, &listing.item_id).await? {
                self.handle_unseen_item(listing).await?;
            }
        }

        Ok(())
    }

    async fn handle_unseen_item(&mut self, listing: &SearchListing) -> Result {
        info!("New item: {}.", listing.item_id);
        // TODO: notify.
        Ok(())
    }
}
