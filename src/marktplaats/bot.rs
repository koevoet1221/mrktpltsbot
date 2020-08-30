use rand::prelude::*;

use crate::marktplaats;
use crate::prelude::*;
use crate::redis::set_nx_ex;

/// Minimal sleep duration between Marktplaats searches.
const MIN_SLEEP_MILLIS: u64 = 20000;

/// Maximal sleep duration between Marktplaats searches.
const MAX_SLEEP_MILLIS: u64 = 40000;

/// Marktplaats search charset.
const SEARCH_CHARSET: &[char] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

/// Ad "seen" flag expiration time.
const SEEN_TTL_SECS: u64 = 30 * 24 * 60 * 60;

pub struct Bot {
    redis: RedisConnection,
}

impl Bot {
    pub fn new(redis: RedisConnection) -> Self {
        Self { redis }
    }

    /// Spawn the bot.
    pub async fn spawn(mut self) -> Result {
        let mut rng = thread_rng();

        loop {
            log_result(self.loop_(&mut rng).await);

            // Sleep for a while before the next request to Marktplaats.
            sleep(&mut rng).await;
        }
    }

    async fn loop_(&mut self, rng: &mut ThreadRng) -> Result {
        // Pick a random query so that we get an equal chance to pull new ads with other words.
        let query = random_query(rng);
        let search_response = marktplaats::search(&query).await?;
        self.redis.hincr("searches::count", &query, 1).await?;
        self.handle_search_result(&query, search_response).await?;
        Ok(())
    }

    async fn handle_search_result(
        &mut self,
        query: &str,
        response: marktplaats::SearchResponse,
    ) -> Result {
        info!("Got {} results.", response.listings.len());
        let mut counter = 0usize;

        for listing in response.listings.iter() {
            if set_nx_ex(
                &mut self.redis,
                &format!("ads::{}::seen", listing.item_id),
                1,
                SEEN_TTL_SECS,
            )
            .await?
            {
                self.handle_new_ad(listing).await?;
                counter += 1;
            }
        }

        info!("{} new ads.", counter);
        self.redis
            .hincr("searches::ads::count", query, counter)
            .await?;
        Ok(())
    }

    async fn handle_new_ad(&mut self, listing: &marktplaats::SearchListing) -> Result {
        info!(
            "New: {:<12} | {} | {}",
            listing.item_id,
            listing.timestamp.format("%m-%d %H:%M"),
            listing.title,
        );
        Ok(())
    }
}

/// Sleeps for a random amount of time.
async fn sleep(rng: &mut ThreadRng) {
    let duration = Duration::from_millis(rng.gen_range(MIN_SLEEP_MILLIS, MAX_SLEEP_MILLIS));
    info!("Next iteration in {:?}.", duration);
    task::sleep(duration).await;
}

/// Generate random query like `"ab*"`.
fn random_query(rng: &mut ThreadRng) -> String {
    format!("{}{}*", random_char(rng), random_char(rng))
}

/// Generates random char from `SEARCH_CHARSET`.
fn random_char(rng: &mut ThreadRng) -> char {
    SEARCH_CHARSET[rng.gen_range(0, SEARCH_CHARSET.len())]
}
