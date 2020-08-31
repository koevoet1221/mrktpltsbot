use crate::marktplaats::{SearchListing, SearchResponse};
use crate::prelude::*;
use crate::redis::{pick_random_subscription, set_nx_ex};

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
            if let Some((chat_id, query)) = pick_random_subscription(&mut self.redis).await? {
                log_result(self.perform_search(chat_id, query).await);
            }
            task::sleep(POLL_INTERVAL).await;
        }
    }

    async fn perform_search(&mut self, chat_id: i64, query: String) -> Result {
        info!("Performing the search for `{}`…", query);
        // TODO
        Ok(())
    }

    async fn handle_search_result(
        &mut self,
        subscription_id: i64,
        response: SearchResponse,
    ) -> Result {
        info!("{} results.", response.listings.len());

        for listing in response.listings.iter() {
            if set_nx_ex(
                &mut self.redis,
                &format!("seen::{}::{}", subscription_id, listing.item_id),
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
