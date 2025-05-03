pub mod client;
pub mod listing;

use std::borrow::Cow;

use bon::Builder;
use chrono::Utc;

use crate::{
    db::{
        Db,
        item::{Item, Items},
        notification::{Notification, Notifications},
        search_query::SearchQuery,
        subscription::Subscription,
    },
    heartbeat::Heartbeat,
    marketplace::marktplaats::{
        client::{MarktplaatsClient, SearchRequest},
        listing::Listing,
    },
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

#[derive(Builder)]
pub struct Marktplaats {
    db: Db,
    client: MarktplaatsClient,
    telegram: Telegram,
    command_builder: CommandBuilder,
    search_limit: u32,
    heartbeat: Heartbeat,
}

impl Marktplaats {
    /// Handle the [`Subscription`] and return [`Reaction`]'s to it.
    #[instrument(skip_all)]
    pub async fn handle(&self, subscription: &Subscription, search_query: &SearchQuery) -> Result {
        debug!(subscription.chat_id, search_query.text, "Handling…");
        let text = &search_query.text;
        let unsubscribe_link = self.command_builder.unsubscribe_link(search_query.hash);

        for listing in self.search(&search_query.text).await? {
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

        debug!(subscription.chat_id, search_query.text, "Done");
        self.check_in().await;
        Ok(())
    }

    /// Search Marktplaats.
    #[instrument(skip_all)]
    pub async fn search(&self, query: &str) -> Result<Vec<Listing>> {
        info!(query, "Searching…");
        let listings =
            SearchRequest::standard(query, self.search_limit).call_on(&self.client).await?.inner;
        info!(query, n_listings = listings.len(), "Fetched");
        Ok(listings)
    }

    pub async fn check_in(&self) {
        self.heartbeat.check_in().await;
    }
}
