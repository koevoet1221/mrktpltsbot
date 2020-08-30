use crate::marktplaats::{SearchListing, SearchResponse};
use crate::prelude::*;
use crate::redis::set_nx_ex;

const SAVED_SEARCHES_KEY: &str = "searches::saved";

/// Ad "seen" flag expiration time.
const SEEN_TTL_SECS: u64 = 30 * 24 * 60 * 60;

const POLL_INTERVAL: Duration = Duration::from_secs(60);

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
            if let Some(search_id) = self.redis.srandmember(SAVED_SEARCHES_KEY).await? {
                log_result(self.perform_search(search_id).await);
            } else {
                info!("No saved searches.");
            }
            task::sleep(POLL_INTERVAL).await;
        }
    }

    async fn perform_search(&mut self, search_id: i64) -> Result {
        Ok(())
    }

    async fn handle_search_result(&mut self, search_id: i64, response: SearchResponse) -> Result {
        info!("{} results.", response.listings.len());

        for listing in response.listings.iter() {
            if set_nx_ex(
                &mut self.redis,
                &format!("seen::{}::{}", search_id, listing.item_id),
                1,
                SEEN_TTL_SECS,
            )
            .await?
            {
                self.handle_new_ad(listing).await?;
            }
        }

        Ok(())
    }

    async fn handle_new_ad(&mut self, listing: &SearchListing) -> Result {
        info!("New: {}", listing.item_id,);
        Ok(())
    }
}
