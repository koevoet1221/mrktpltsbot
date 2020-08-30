//! Implements the Telegram chat bot.

use crate::marktplaats::{search, SearchListing};
use crate::prelude::*;
use crate::telegram::*;

const OFFSET_KEY: &str = "telegram::offset";

pub struct ChatBot {
    telegram: Telegram,
    redis: RedisConnection,
    allowed_chats: HashSet<ChatId>,
}

impl ChatBot {
    pub fn new(telegram: Telegram, redis: RedisConnection, allowed_chats: HashSet<i64>) -> Self {
        Self {
            redis,
            telegram,
            allowed_chats,
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
            .get_updates(offset, vec!["message", "callback_query"])
            .await?
            .into_iter()
        {
            offset = offset.max(update.id);
            self.redis.set(OFFSET_KEY, offset + 1).await?;
            self.handle_update(update).await.log_result();
        }
        Ok(())
    }
}

/// Handles Telegram `Update`s.
impl ChatBot {
    /// Handle a single `Update`.
    async fn handle_update(&self, update: Update) -> Result {
        info!("Update #{}.", update.id);

        if let Some(message) = update.message {
            info!("Message #{}.", message.id);
            self.handle_message(message.chat.id, message.text).await?;
        } else if let Some(callback_query) = update.callback_query {
            info!("Callback query #{}.", callback_query.id);
            // TODO: https://core.telegram.org/bots/api#answercallbackquery
            if let Some(message) = callback_query.message {
                self.handle_message(message.chat.id, callback_query.data)
                    .await?;
            } else {
                warn!("No message in the callback query.");
            }
        } else {
            warn!("Unhandled update #{}.", update.id);
        }

        Ok(())
    }

    async fn handle_message(&self, chat_id: ChatId, text: Option<String>) -> Result {
        info!("Message from the chat #{}.", chat_id);

        if self.allowed_chats.contains(&chat_id) {
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

    async fn handle_command(&self, chat_id: ChatId, text: String) -> Result {
        let text = text.trim();
        if let Some(query) = text.strip_prefix("/subscribe ") {
            // TODO
        } else if let Some(search_id) = text.strip_prefix("/unsubscribe ") {
            // TODO
        } else if let Some(query) = text.strip_prefix("/search ") {
            // TODO: refactor, it will be used in `search_bot` too.
            let search_response = search(query, "1").await?;
            for listing in search_response.listings.iter() {
                self.telegram
                    .send_message(chat_id, &format_listing(listing), Some("MarkdownV2"), None)
                    .await?;
            }
        } else if text == "/list" {
            // TODO
        } else {
            // Search query.
            self.telegram
                .send_message(
                    chat_id,
                    &format!("🎲 Search *{}*?", escape_markdown_v2(&text)),
                    Some("MarkdownV2"),
                    Some(ReplyMarkup::InlineKeyboard(vec![vec![
                        // TODO: refactor:
                        InlineKeyboardButton {
                            text: "🔎 Preview".into(),
                            callback_data: Some(format!("/search {}", text)),
                            url: None,
                        },
                        InlineKeyboardButton {
                            text: "✅ Subscribe".into(),
                            callback_data: Some(format!("/subscribe {}", text)),
                            url: None,
                        },
                    ]])),
                )
                .await
                .log_result();
        }
        Ok(())
    }
}

fn format_listing(listing: &SearchListing) -> String {
    // FIXME:
    let euros = listing.price.cents / 100;
    let cents = listing.price.cents % 100;

    // TODO: check the price type.
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
