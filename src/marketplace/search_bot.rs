use std::time::Duration;

use bon::Builder;
use tokio::time::sleep;
use tracing::{error, info};

use crate::{
    db::{Db, search_query::SearchQuery, subscription::Subscription},
    prelude::{instrument, *},
};

/// Generic marketplace search bot.
#[derive(Builder)]
pub struct SearchBot {
    db: Db,
    search_interval: Duration,
}

impl SearchBot {
    /// Run the bot indefinitely.
    pub async fn run(self) {
        info!(?self.search_interval, "Running the search bot…");
        let mut previous = None;
        loop {
            sleep(self.search_interval).await;
            match self.advance_and_handle(previous.as_ref()).await {
                Ok(handled) => {
                    previous = handled;
                }
                Err(error) => {
                    error!("Failed to handle the next subscription: {error:#}");
                }
            }
        }
    }

    /// Advance in the subscription list and handle the subscription.
    ///
    /// # Returns
    ///
    /// Handled subscription entry as a next pointer.
    async fn advance_and_handle(
        &self,
        previous: Option<&(Subscription, SearchQuery)>,
    ) -> Result<Option<(Subscription, SearchQuery)>> {
        let current = match previous {
            Some((previous, _)) => match self.db.next_subscription(previous).await? {
                Some(next) => Some(next),
                None => self.db.first_subscription().await?, // reached the end, restart
            },
            None => self.db.first_subscription().await?, // fresh start or no subscriptions
        };
        if let Some((subscription, search_query)) = &current {
            self.handle(subscription, search_query).await?;
            Ok(current)
        } else {
            info!("No active subscriptions");
            Ok(None)
        }
    }

    /// Handle the specified subscription.
    #[instrument(skip_all)]
    async fn handle(&self, _subscription: &Subscription, search_query: &SearchQuery) -> Result {
        info!(search_query.text, "Handling…");
        // TODO
        info!(search_query.text, "Done.");
        Ok(())
    }
}
