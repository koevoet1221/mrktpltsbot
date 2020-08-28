use crate::marktplaats;
use crate::prelude::*;
use rand::prelude::*;

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
    pub redis: redis::aio::Connection,
}

impl Bot {
    /// Spawn the bot.
    pub async fn spawn(mut self) -> Result {
        let mut rng = thread_rng();

        loop {
            if let Err(error) = self.loop_(&mut rng).await {
                async_std::task::spawn(async move {
                    let uuid = capture_anyhow(&error);
                    error!("{}, Sentry ID: {}", error, uuid);
                });
            }

            // Sleep for a while before the next request to Marktplaats.
            sleep(&mut rng).await;
        }
    }

    async fn loop_(&mut self, rng: &mut ThreadRng) -> Result {
        // Pick a random query so that we get an equal chance to pull new ads with other words.
        let search_response = marktplaats::search(&random_query(rng)).await?;
        info!("Got {} results.", search_response.listings.len());
        self.handle_search_result(search_response).await?;
        Ok(())
    }

    async fn handle_search_result(&mut self, response: marktplaats::SearchResponse) -> Result {
        let mut counter = 0usize;

        for listing in response.listings.iter() {
            if redis::cmd("SET")
                .arg(format!("ads::{}::seen", listing.item_id))
                .arg(1)
                .arg("NX")
                .arg("EX")
                .arg(SEEN_TTL_SECS)
                .query_async(&mut self.redis)
                .await?
            {
                self.handle_new_ad(listing).await?;
                counter += 1;
            }
        }

        info!("{} new ads.", counter);
        Ok(())
    }

    async fn handle_new_ad(&mut self, listing: &marktplaats::SearchListing) -> Result {
        info!(
            "New: {:<10} | {} | {:.40}",
            listing.item_id,
            listing.timestamp.format("%m-%d %T%.3f"),
            listing.title,
        );
        Ok(())
    }
}

/// Sleeps for a random amount of time.
async fn sleep(rng: &mut ThreadRng) {
    let duration = Duration::from_millis(rng.gen_range(MIN_SLEEP_MILLIS, MAX_SLEEP_MILLIS));
    info!("Next iteration in {:?}.", duration);
    async_std::task::sleep(duration).await;
}

/// Generate random query like `"ab*"`.
fn random_query(rng: &mut ThreadRng) -> String {
    format!("{}{}*", random_char(rng), random_char(rng))
}

/// Generates random char from `SEARCH_CHARSET`.
fn random_char(rng: &mut ThreadRng) -> char {
    SEARCH_CHARSET[rng.gen_range(0, SEARCH_CHARSET.len())]
}
