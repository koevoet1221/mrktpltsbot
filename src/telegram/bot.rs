use std::{borrow::Cow, collections::HashSet};

use bon::bon;
use maud::{Render, html};

use crate::{
    db::{Db, SearchQueries, SearchQuery, Subscription, Subscriptions},
    heartbeat::Heartbeat,
    marketplace::{Marketplace, Marktplaats, Vinted},
    prelude::*,
    telegram::{
        Telegram,
        commands::{CommandBuilder, CommandPayload, SubscriptionAction},
        methods::{
            AllowedUpdate,
            GetUpdates,
            Method,
            SendMessage,
            SetMyCommands,
            SetMyDescription,
        },
        notification::Notification,
        objects::{
            BotCommand,
            ChatId,
            LinkPreviewOptions,
            ParseMode,
            ReplyParameters,
            Update,
            UpdatePayload,
        },
        render,
        render::{DELIMITER, ManageSearchQuery},
    },
};

/// Telegram [`Message`] bot.
///
/// It listens to Telegram [`Update`]'s and reacts on them.
#[derive(Clone)]
pub struct Bot {
    telegram: Telegram,
    authorized_chat_ids: HashSet<i64>,
    db: Db,
    marktplaats: Marktplaats,
    vinted: Vinted,
    poll_timeout_secs: u64,
    heartbeat: Heartbeat,
    command_builder: CommandBuilder,
}

#[bon]
impl Bot {
    #[builder(finish_fn = try_init)]
    pub async fn new(
        telegram: Telegram,
        command_builder: CommandBuilder,
        db: Db,
        marktplaats: Marktplaats,
        vinted: Vinted,
        heartbeat: Heartbeat,
        authorized_chat_ids: HashSet<i64>,
        poll_timeout_secs: u64,
    ) -> Result<Self> {
        SetMyDescription::builder()
            .description("👋 This is a private bot for Marktplaats\n\nFeel free to set up your own instance from https://github.com/eigenein/mrktpltsbot")
            .build()
            .call_on(&telegram)
            .await
            .context("failed to set the bot's description")?;
        SetMyCommands::builder()
            .commands(&[&BotCommand::builder()
                .command("manage")
                .description("List and manage your subscriptions")
                .build()])
            .build()
            .call_on(&telegram)
            .await
            .context("failed to set the bot's commands")?;
        Ok(Self {
            telegram,
            authorized_chat_ids,
            db,
            marktplaats,
            vinted,
            poll_timeout_secs,
            heartbeat,
            command_builder,
        })
    }
}

impl Bot {
    /// Run the bot indefinitely.
    pub async fn run(mut self) {
        info!(me = self.command_builder.url().as_str(), "🔄 Running Telegram bot…");
        let mut offset = 0;
        loop {
            offset = self.handle_updates(offset).await;
        }
    }

    /// Handle a single batch of updates.
    ///
    /// # Returns
    ///
    /// New offset.
    #[instrument(skip_all)]
    async fn handle_updates(&mut self, offset: u64) -> u64 {
        let get_updates = GetUpdates::builder()
            .offset(offset)
            .timeout_secs(self.poll_timeout_secs)
            .allowed_updates(&[AllowedUpdate::Message])
            .build();

        let updates: Vec<Update> = match self.telegram.call(&get_updates).await {
            Ok(updates) => {
                self.heartbeat.check_in().await;
                updates
            }
            Err(error) => {
                error!("‼️ Failed to fetch Telegram updates: {error:#}");
                return offset;
            }
        };

        let new_offset = updates.last().map_or(offset, |last_update| last_update.id + 1);
        if updates.is_empty() {
            debug!("📭 Received no Telegram updates");
        } else {
            info!(n = updates.len(), new_offset, "📬 Received Telegram updates");
        }

        for update in updates {
            let UpdatePayload::Message(message) = update.payload else { continue };
            let (Some(chat), Some(text)) = (message.chat, message.text) else {
                warn!(message.id, "⚠️ Message without an associated chat or text");
                continue;
            };
            let ChatId::Integer(chat_id) = chat.id else {
                warn!(message.id, "⚠️ Username chat IDs are not supported");
                continue;
            };
            if let Err(error) = self.on_message(chat_id, message.id, text.trim()).await {
                error!(%chat_id, message.id, "‼️ Failed to handle the message: {error:#}");
                let _ = SendMessage::builder()
                    .chat_id(Cow::Owned(ChatId::Integer(chat_id)))
                    .text("💥 An internal error occurred and has been logged")
                    .build()
                    .call_and_discard_on(&self.telegram)
                    .await;
            }
        }

        new_offset
    }

    #[instrument(skip_all)]
    async fn on_message(&mut self, chat_id: i64, message_id: u64, text: &str) -> Result {
        if !self.authorized_chat_ids.contains(&chat_id) {
            warn!(chat_id, message_id, text, "⚠️ Received message from an unauthorized chat");
            let chat_id = ChatId::Integer(chat_id);
            let text = render::unauthorized(&chat_id).render().into_string();
            let _ =
                SendMessage::quick_html(Cow::Owned(chat_id), text).call_on(&self.telegram).await?;
            return Ok(());
        }

        let reply_parameters = ReplyParameters::builder()
            .message_id(message_id)
            .allow_sending_without_reply(true)
            .build();

        if text.starts_with('/') {
            self.on_command(text, chat_id, reply_parameters).await?;
        } else {
            self.on_search(text, chat_id, reply_parameters).await?;
        }
        Ok(())
    }
    /// Handle the search request from Telegram.
    ///
    /// A search request is just a message that is not a command.
    #[instrument(skip_all)]
    async fn on_search(
        &mut self,
        query: &str,
        chat_id: i64,
        reply_parameters: ReplyParameters,
    ) -> Result {
        let query = SearchQuery::from(query);

        let mut items = Vec::new();
        self.marktplaats.search_and_extend_infallible(&query, Some(1), &mut items).await;
        self.vinted.search_and_extend_infallible(&query, Some(1), &mut items).await;
        info!(query.hash, n_items = items.len(), query.text, "🛍️");

        SearchQueries(&mut *self.db.connection().await).upsert(&query).await?;

        // We need the subscribe command anyway, even if no listings were found.
        let subscribe_link = self.command_builder.subscribe_link(query.hash);

        if items.is_empty() {
            let markup = html! {
                "There are no items matching the search query. Try a different query or subscribe anyway to wait for them to appear"
                (DELIMITER)
                (ManageSearchQuery::new(&query.text, &[&subscribe_link]))
            };
            let _ = SendMessage::builder()
                .chat_id(Cow::Owned(chat_id.into()))
                .text(markup.render().into_string())
                .parse_mode(ParseMode::Html)
                .reply_parameters(reply_parameters)
                .link_preview_options(LinkPreviewOptions::DISABLED)
                .build()
                .call_on(&self.telegram)
                .await?;
        } else {
            for item in items {
                let description = render::item_description(
                    &item,
                    &ManageSearchQuery::new(&query.text, &[&subscribe_link]),
                );
                Notification::builder()
                    .chat_id(Cow::Owned(chat_id.into()))
                    .text(description.into())
                    .maybe_picture_url(item.picture_url.as_ref())
                    .reply_parameters(reply_parameters)
                    .parse_mode(ParseMode::Html)
                    .build()
                    .react_to(&self.telegram)
                    .await?;
            }
        }

        Ok(())
    }

    #[instrument(skip_all)]
    async fn on_command(
        &self,
        text: &str,
        chat_id: i64,
        reply_parameters: ReplyParameters,
    ) -> Result {
        if text == "/start" {
            // Just an initial greeting.
            let chat_id: Cow<'_, ChatId> = Cow::Owned(ChatId::Integer(chat_id));
            let _ = SendMessage::builder()
                .chat_id(chat_id.clone())
                .text("👋")
                .build()
                .call_on(&self.telegram)
                .await?;
            let _ = SendMessage::builder()
                .chat_id(chat_id)
                .text("Just send me a search query to start")
                .build()
                .call_on(&self.telegram)
                .await?;
        } else if text == "/manage" {
            self.on_manage_subscriptions(chat_id).await?;
        } else if let Some(payload) = text.strip_prefix("/start ") {
            // Command with a payload.
            let command = CommandPayload::from_base64(payload)?;
            debug!(?command, "❕ Received command");

            if command.manage.is_some() {
                self.on_manage_subscriptions(chat_id).await?;
            }

            if let Some(subscription_command) = command.subscription {
                let query_hash = subscription_command.query_hash;
                let subscription = Subscription { query_hash, chat_id };
                let connection = &mut *self.db.connection().await;
                let query_text = SearchQueries(connection).fetch_text(query_hash).await?;
                let mut subscriptions = Subscriptions(connection);

                match SubscriptionAction::try_from(subscription_command.action) {
                    Ok(SubscriptionAction::Subscribe) => {
                        info!(subscription.query_hash, "➕ Subscribing");
                        subscriptions.upsert(subscription).await?;
                        let unsubscribe_link =
                            self.command_builder.unsubscribe_link(subscription.query_hash);
                        let markup = html! {
                            "You are now subscribed"
                            (DELIMITER)
                            (ManageSearchQuery::new(&query_text, &[&unsubscribe_link, &self.command_builder.manage_link()]))
                        };
                        let send_message = SendMessage::quick_html(
                            Cow::Owned(chat_id.into()),
                            markup.render().into_string(),
                        );
                        let _ = send_message.call_on(&self.telegram).await?;
                    }

                    Ok(SubscriptionAction::Unsubscribe) => {
                        info!(subscription.query_hash, "➖ Unsubscribing");
                        subscriptions.delete(subscription).await?;
                        let resubscribe_link =
                            self.command_builder.resubscribe_link(subscription.query_hash);
                        let markup = html! {
                            "You are now unsubscribed"
                            (DELIMITER)
                            (ManageSearchQuery::new(&query_text, &[&resubscribe_link, &self.command_builder.manage_link()]))
                        };
                        let send_message = SendMessage::quick_html(
                            Cow::Owned(chat_id.into()),
                            markup.render().into_string(),
                        );
                        let _ = send_message.call_on(&self.telegram).await?;
                    }

                    _ => {} // TODO: technically, I should return a message that the action is no longer supported
                }
            }
        } else {
            // Unknown command.
            let _ = SendMessage::builder()
                .chat_id(Cow::Owned(chat_id.into()))
                .text("I am sorry, but I do not know this command")
                .reply_parameters(reply_parameters)
                .build()
                .call_on(&self.telegram)
                .await?;
        }
        Ok(())
    }

    /// List the user's subscriptions.
    #[instrument(skip_all)]
    async fn on_manage_subscriptions(&self, chat_id: i64) -> Result {
        let subscriptions = self.db.subscriptions_of(chat_id).await?;
        let markup = html! {
            @if subscriptions.is_empty() {
                "You do not have any subscriptions at the moment"
            } @else {
                "Here are your subscriptions:\n"
                @for (subscription, search_query) in subscriptions {
                    @let unsubscribe_link = self.command_builder.unsubscribe_link(subscription.query_hash);;
                    "\n"
                    (ManageSearchQuery::new(&search_query.text, &[&unsubscribe_link]))
                }
            }
        };
        let _ = SendMessage::builder()
            .chat_id(Cow::Owned(chat_id.into()))
            .text(markup.render().into_string())
            .parse_mode(ParseMode::Html)
            .link_preview_options(LinkPreviewOptions::DISABLED)
            .build()
            .call_on(&self.telegram)
            .await?;
        Ok(())
    }
}
