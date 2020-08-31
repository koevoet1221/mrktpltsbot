//! Implements the Telegram chat bot.

use crate::marktplaats::{search, SearchListing};
use crate::math::div_rem;
use crate::prelude::*;
use crate::telegram::{types::*, *};

const OFFSET_KEY: &str = "telegram::offset";
const ALLOWED_UPDATES: &[&str] = &["message", "callback_query"];
const MARKDOWN_V2: Option<&str> = Some("MarkdownV2");

pub struct ChatBot {
    telegram: Telegram,
    redis: RedisConnection,

    /// For the sake of security we only allow the specified chats to interact with the bot.
    allowed_chat_ids: HashSet<i64>,
}

impl ChatBot {
    pub fn new(telegram: Telegram, redis: RedisConnection, allowed_chat_ids: HashSet<i64>) -> Self {
        Self {
            redis,
            telegram,
            allowed_chat_ids,
        }
    }

    pub async fn spawn(mut self) -> Result {
        self.set_my_commands().await?;
        info!("Running the chat bot…");
        loop {
            self.handle_updates().await.log_result();
        }
    }

    async fn handle_updates(&mut self) -> Result {
        let mut offset = self
            .redis
            .get::<_, Option<i64>>(OFFSET_KEY)
            .await?
            .unwrap_or_default();
        for update in self
            .telegram
            .get_updates(offset, ALLOWED_UPDATES)
            .await?
            .into_iter()
        {
            offset = offset.max(update.id);
            self.redis.set(OFFSET_KEY, offset + 1).await?;
            self.handle_update(update).await.log_result();
        }
        Ok(())
    }

    /// Handle a single `Update`.
    async fn handle_update(&mut self, update: Update) -> Result {
        info!("Update #{}.", update.id);

        if let Some(message) = update.message {
            info!("Message #{}.", message.id);
            self.handle_text_message(message.chat.id, message.text)
                .await?;
        } else if let Some(callback_query) = update.callback_query {
            info!("Callback query #{}.", callback_query.id);
            // TODO: https://core.telegram.org/bots/api#answercallbackquery
            if let Some(message) = callback_query.message {
                self.handle_text_message(message.chat.id, callback_query.data)
                    .await?;
            } else {
                warn!("No message in the callback query.");
            }
        } else {
            warn!("Unhandled update #{}.", update.id);
        }

        Ok(())
    }

    async fn handle_text_message(&mut self, chat_id: i64, text: Option<String>) -> Result {
        info!("Message from the chat #{}.", chat_id);

        if self.allowed_chat_ids.contains(&chat_id) {
            if let Some(text) = text {
                self.handle_command(chat_id, text).await?;
            } else {
                warn!("Empty message text.");
            }
        } else {
            warn!("Forbidden chat: {}.", chat_id);
            self.telegram
                .send_message(chat_id, &format!("⚠️ *Forbidden*\n\nAsk the administrator to add the chat ID `{}` to the allowed list\\.", chat_id), Some("MarkdownV2"), None)
                .await?;
        }

        Ok(())
    }
}

impl ChatBot {
    async fn handle_command(&mut self, chat_id: i64, text: String) -> Result {
        let text = text.trim();

        if let Some(query) = text.strip_prefix("/subscribe ") {
            self.handle_subscribe_command(chat_id, query).await?;
        } else if let Some(subscription_id) = text.strip_prefix("/unsubscribe ") {
            self.handle_unsubscribe_command(chat_id, subscription_id)
                .await?;
        } else if let Some(query) = text.strip_prefix("/search ") {
            self.handle_search_preview_command(chat_id, query).await?;
        } else if text == "/list" {
            self.handle_list_command(chat_id).await?;
        } else {
            self.handle_search_query(chat_id, text).await?;
        }

        Ok(())
    }

    async fn handle_subscribe_command(&mut self, chat_id: i64, query: &str) -> Result {
        let (subscription_id, subscription_count) =
            crate::redis::subscribe_to(&mut self.redis, chat_id, &query).await?;
        self.telegram
            .send_message(
                chat_id,
                &format!(
                    "✅ Subscribed to *{}*\n\nThere\\'re *{}* active subscriptions now.",
                    escape_markdown_v2(query),
                    subscription_count,
                ),
                MARKDOWN_V2,
                Into::<ReplyMarkup>::into(InlineKeyboardButton::new_unsubscribe_button(
                    subscription_id,
                )),
            )
            .await?;
        Ok(())
    }

    async fn handle_unsubscribe_command(&self, _chat_id: i64, _subscription_id: &str) -> Result {
        Ok(())
    }

    async fn handle_list_command(&self, _chat_id: i64) -> Result {
        Ok(())
    }

    async fn handle_search_preview_command(&self, chat_id: i64, query: &str) -> Result {
        let search_response = search(query, "1").await?;
        for listing in search_response.listings.iter() {
            self.telegram
                .send_message(chat_id, &format_listing(listing), MARKDOWN_V2, None)
                .await?;
        }
        Ok(())
    }

    async fn handle_search_query(&self, chat_id: i64, text: &str) -> Result {
        self.telegram
            .send_message(
                chat_id,
                &format!("🎲 Search *{}*?", escape_markdown_v2(text)),
                MARKDOWN_V2,
                Into::<ReplyMarkup>::into(vec![
                    InlineKeyboardButton::new_search_preview_button(text),
                    InlineKeyboardButton::new_subscribe_button(text),
                ]),
            )
            .await?;
        Ok(())
    }
}

impl InlineKeyboardButton {
    fn new_search_preview_button(query: &str) -> Self {
        Self {
            text: "🔎 Preview".into(),
            callback_data: Some(format!("/search {}", query)),
            url: None,
        }
    }

    fn new_subscribe_button(query: &str) -> Self {
        Self {
            text: "✅ Subscribe".into(),
            callback_data: Some(format!("/subscribe {}", query)),
            url: None,
        }
    }

    fn new_unsubscribe_button(subscription_id: i64) -> Self {
        Self {
            text: "❌ Unsubscribe".into(),
            callback_data: Some(format!("/unsubscribe {}", subscription_id)),
            url: None,
        }
    }
}

fn format_listing(listing: &SearchListing) -> String {
    let (euros, cents) = div_rem(listing.price.cents, 100);

    format!(
        "*{}*\n\n💰 {}\\.{:02} {:?}\n\n{}",
        escape_markdown_v2(&listing.title),
        euros,
        cents,
        listing.price.type_,
        escape_markdown_v2(&listing.description),
    )
}

impl ChatBot {
    /// Set the bot commands.
    async fn set_my_commands(&self) -> Result {
        info!("Setting the chat bot commands…");
        self.telegram
            .set_my_commands(vec![
                BotCommand {
                    command: "/list".into(),
                    description: "Show the saved searches".into(),
                },
                BotCommand {
                    command: "/subscribe".into(),
                    description: "Subscribe to the search query".into(),
                },
                BotCommand {
                    command: "/unsubscribe".into(),
                    description: "Unsubscribe from the search query".into(),
                },
                BotCommand {
                    command: "/search".into(),
                    description: "Make one-time search".into(),
                },
            ])
            .await?;
        Ok(())
    }
}
