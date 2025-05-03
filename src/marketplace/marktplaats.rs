pub mod client;
pub mod listing;

use std::{borrow::Cow, time::Duration};

use bon::Builder;
use chrono::Utc;
use tokio::time::sleep;

use crate::{
    db::{
        Db,
        item::{Item, Items},
        notification::{Notification, Notifications},
        search_query::SearchQuery,
        subscription::Subscription,
    },
    heartbeat::Heartbeat,
    marketplace::marktplaats::client::{MarktplaatsClient, SearchRequest},
    prelude::*,
    telegram::{
        Telegram,
        commands::CommandBuilder,
        objects::ParseMode,
        reaction::ReactionMethod,
        render,
        render::ManageSearchQuery,
    },
};

/// Marktplaats reactor.
///
/// It crawls Marktplaats and produces reactions on the items.
#[derive(Builder)]
pub struct Marktplaats {
    db: Db,
    client: MarktplaatsClient,
    telegram: Telegram,
    command_builder: CommandBuilder,
    crawl_interval: Duration,
    search_limit: u32,
    heartbeat: Heartbeat,
}

impl Marktplaats {
    /// Run the bot indefinitely.
    pub async fn run(self) {
        info!(?self.crawl_interval, "Running Marktplaats bot…");
        let mut entry = None;
        loop {
            sleep(self.crawl_interval).await;
            match self.handle_subscription(entry.as_ref()).await {
                Ok(next_entry) => {
                    entry = next_entry;
                }
                Err(error) => {
                    error!("Failed to handle the next subscription: {error:#}");
                }
            }
        }
    }

    /// Handle a single subscription.
    ///
    /// # Returns
    ///
    /// Handled subscription entry.
    #[instrument(skip_all)]
    async fn handle_subscription(
        &self,
        previous: Option<&(Subscription, SearchQuery)>,
    ) -> Result<Option<(Subscription, SearchQuery)>> {
        let entry = match previous {
            None => self.db.first_subscription().await?,
            Some((previous, _)) => match self.db.next_subscription(previous).await? {
                Some(next) => Some(next),
                None => self.db.first_subscription().await?, // reached the end, restart
            },
        };
        if let Some((subscription, search_query)) = entry {
            self.on_subscription(&subscription, &search_query).await?;
            self.heartbeat.check_in().await;
            Ok(Some((subscription, search_query)))
        } else {
            info!("No active subscriptions");
            self.heartbeat.check_in().await;
            Ok(None)
        }
    }

    /// Handle the [`Subscription`] and return [`Reaction`]'s to it.
    #[instrument(skip_all)]
    async fn on_subscription(
        &self,
        subscription: &Subscription,
        search_query: &SearchQuery,
    ) -> Result {
        info!(subscription.chat_id, search_query.text, "Crawling…");
        let text = &search_query.text;
        let unsubscribe_link = self.command_builder.unsubscribe_link(search_query.hash);

        let listings = SearchRequest::standard(&search_query.text, self.search_limit)
            .call_on(&self.client)
            .await?
            .inner;
        info!(subscription.chat_id, search_query.text, n_listings = listings.len(), "Fetched");

        for listing in listings {
            let mut connection = self.db.connection().await;
            let item = Item { id: &listing.item_id, updated_at: Utc::now() };
            Items(&mut connection).upsert(item).await?;
            let notification =
                Notification { item_id: listing.item_id.clone(), chat_id: subscription.chat_id };
            if Notifications(&mut connection).exists(&notification).await? {
                trace!(subscription.chat_id, listing.item_id, "Notification was already sent");
                continue;
            }
            info!(subscription.chat_id, notification.item_id, "Reacting");
            let description = render::listing_description(
                &listing,
                &ManageSearchQuery::new(text, &[&unsubscribe_link]),
            );
            ReactionMethod::builder()
                .chat_id(Cow::Owned(subscription.chat_id.into()))
                .text(description.into())
                .maybe_picture(listing.pictures.first())
                .parse_mode(ParseMode::Html)
                .build()
                .react_to(&self.telegram)
                .await?;
            Notifications(&mut connection).upsert(&notification).await?;
        }

        info!(subscription.chat_id, search_query.text, "Done");
        Ok(())
    }
}
