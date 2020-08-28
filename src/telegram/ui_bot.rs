use crate::prelude::*;
use crate::telegram::*;

pub struct UiBot {
    telegram: Telegram,
    redis: RedisConnection,
}

impl UiBot {
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
        let updates = self
            .telegram
            .get_updates(
                self.redis
                    .get::<_, Option<i64>>("telegram::offset")
                    .await?
                    .unwrap_or_default(),
                vec!["message"],
            )
            .await?;
        for update in updates.iter() {
            self.handle_update(update).await?;
        }
        if let Some(offset) = updates.iter().map(|update| update.id).max() {
            self.redis.set("telegram::offset", offset + 1).await?;
        }
        Ok(())
    }

    async fn handle_update(&self, update: &Update) -> Result {
        info!("Message #{}.", update.id);
        Ok(())
    }

    /// Set the Telegram bot commands.
    async fn set_my_commands(&self) -> Result {
        info!("Setting the bot commands…");
        self.telegram
            .set_my_commands(vec![
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
