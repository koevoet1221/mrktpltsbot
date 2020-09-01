use crate::prelude::*;
use crate::redis::pop_notification;
use crate::telegram::{Telegram, MARKDOWN_V2};

const ONE_SECOND: Duration = Duration::from_secs(1);

pub struct Notifier {
    redis: RedisConnection,
    telegram: Telegram,
}

impl Notifier {
    pub fn new(redis: RedisConnection, telegram: Telegram) -> Self {
        Self { redis, telegram }
    }

    pub async fn run(mut self) -> Result {
        info!("Running…");
        loop {
            let notification = pop_notification(&mut self.redis).await?;
            info!("Notification to the chat #{}.", notification.chat_id);

            if let Some(image_url) = notification.image_url {
                self.telegram
                    .send_photo(
                        notification.chat_id,
                        &image_url,
                        Some(&notification.text),
                        MARKDOWN_V2,
                        notification.reply_markup,
                    )
                    .await
                    .log_result();
            } else {
                self.telegram
                    .send_message(
                        notification.chat_id,
                        &notification.text,
                        MARKDOWN_V2,
                        notification.reply_markup,
                    )
                    .await
                    .log_result();
            }

            // https://core.telegram.org/bots/faq#my-bot-is-hitting-limits-how-do-i-avoid-this
            task::sleep(ONE_SECOND).await;
        }
    }
}
