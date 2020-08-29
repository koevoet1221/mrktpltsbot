//! Implements the Telegram chat bot.

use crate::prelude::*;
use crate::telegram::*;

pub struct ChatBot {
    telegram: Arc<Telegram>,
    redis: RedisConnection,
}

impl ChatBot {
    pub fn new(telegram: Telegram, redis: RedisConnection) -> Self {
        Self {
            telegram: Arc::new(telegram),
            redis,
        }
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

/// Handles Telegram `Update`s.
impl ChatBot {
    /// Handle a single `Update`.
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
        let telegram = self.telegram.clone();
        match text.trim() {
            "/start" => {
                task::spawn(async move {
                    telegram
                        .send_message(
                            chat_id,
                            "✏️ Start by sending me a search query",
                            Some("MarkdownV2"),
                        )
                        .await
                        .log_result();
                });
            }
            _ => {
                // Treat the text as a search query.
                task::spawn(async move {
                    telegram
                        .send_message(
                            chat_id,
                            "☑️ Click the button to confirm subscribing to the query",
                            Some("MarkdownV2"),
                        )
                        .await
                        .log_result();
                });
            }
        }
        Ok(())
    }
}

impl ChatBot {
    /// Set the bot commands.
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
