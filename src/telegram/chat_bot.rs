//! Implements the Telegram bot chat.

use crate::prelude::*;
use crate::telegram::*;

pub struct ChatBot {
    telegram: Telegram,
    redis: RedisConnection,
}

impl ChatBot {
    pub fn new(telegram: Telegram, redis: RedisConnection) -> Self {
        Self { telegram, redis }
    }

    pub async fn spawn(mut self) -> Result {
        self.set_my_commands().await?;
        loop {
            log_result(self.loop_().await);
        }
    }

    async fn loop_(&mut self) -> Result {
        let mut offset = self
            .redis
            .get::<_, Option<i64>>("telegram::offset")
            .await?
            .unwrap_or_default();
        let updates = self.telegram.get_updates(offset, vec!["message"]).await?;
        for update in updates.into_iter() {
            offset = offset.max(update.id);
            self.redis.set("telegram::offset", offset + 1).await?;
            self.handle_update(update).await?;
        }
        Ok(())
    }
}

/// Handles updates.
impl ChatBot {
    async fn handle_update(&self, update: Update) -> Result {
        info!("Message #{}.", update.id);

        if let Some(message) = update.message {
            if let Some(text) = message.text {
                self.handle_text_update(message.chat.id, text).await?;
            }
        }

        Ok(())
    }

    async fn handle_text_update(&self, chat_id: ChatId, text: String) -> Result {
        match text.trim() {
            "/start" => {
                self.telegram
                    .send_message(
                        chat_id,
                        "✏️ Start by sending me a search query",
                        Some("MarkdownV2"),
                    )
                    .await?;
            }
            _ => {
                // Treat the text as a search query.
                self.telegram
                    .send_message(
                        chat_id,
                        "☑️ Click the button to confirm subscribing to the query",
                        Some("MarkdownV2"),
                    )
                    .await?;
            }
        }
        Ok(())
    }
}

impl ChatBot {
    /// Set the Telegram bot commands.
    async fn set_my_commands(&self) -> Result {
        info!("Setting the bot commands…");
        self.telegram
            .set_my_commands(vec![
                BotCommand {
                    command: "/start".into(),
                    description: "Start the bot".into(),
                },
                BotCommand {
                    command: "/list".into(),
                    description: "List the saved searches".into(),
                },
                BotCommand {
                    command: "/subscribe".into(),
                    description: "Subscribe to the search query".into(),
                },
                BotCommand {
                    command: "/unsubscribe".into(),
                    description: "Unsubscribe from the search query".into(),
                },
            ])
            .await?;
        Ok(())
    }
}
