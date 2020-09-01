//! Telegram bot [API].
//!
//! [API]: https://core.telegram.org/bots/api

use crate::prelude::*;
use crate::telegram::types::*;

pub mod format;
pub mod notifier;
pub mod reply_markup;
pub mod types;

pub const MARKDOWN_V2: Option<&str> = Some("MarkdownV2");

const GET_UPDATES_TIMEOUT: u64 = 60;
const GET_UPDATES_REQUEST_TIMEOUT: Duration = Duration::from_secs(GET_UPDATES_TIMEOUT + 1);

/// <https://core.telegram.org/bots/api>
pub struct Telegram {
    /// <https://core.telegram.org/bots#6-botfather>
    base_url: String,
}

impl Telegram {
    pub fn new(token: &str) -> Self {
        Self {
            base_url: format!("https://api.telegram.org/bot{}", token),
        }
    }

    /// <https://core.telegram.org/bots/api#setmycommands>
    pub async fn set_my_commands(&self, commands: Vec<BotCommand>) -> Result {
        error_for_status(
            CLIENT
                .post(&format!("{}/setMyCommands", self.base_url))
                .json(&json!({ "commands": commands }))
                .send()
                .await?,
        )
        .await?;
        Ok(())
    }

    /// <https://core.telegram.org/bots/api#getupdates>
    pub async fn get_updates(&self, offset: i64, allowed_updates: &[&str]) -> Result<Vec<Update>> {
        Ok(error_for_status(
            CLIENT
                .get(&format!("{}/getUpdates", self.base_url))
                .json(&json!({
                    "offset": offset,
                    "allowed_updates": allowed_updates,
                    "timeout": GET_UPDATES_TIMEOUT,
                }))
                .timeout(GET_UPDATES_REQUEST_TIMEOUT)
                .send()
                .await?,
        )
        .await?
        .json::<TelegramResult<Vec<Update>>>()
        .await?
        .result)
    }

    /// <https://core.telegram.org/bots/api#sendmessage>
    pub async fn send_message<RM: Into<Option<ReplyMarkup>>>(
        &self,
        chat_id: i64,
        text: &str,
        parse_mode: Option<&str>,
        reply_markup: RM,
    ) -> Result<Message> {
        Ok(error_for_status(
            CLIENT
                .post(&format!("{}/sendMessage", self.base_url))
                .json(&json!({
                    "chat_id": chat_id,
                    "text": text,
                    "parse_mode": parse_mode,
                    "reply_markup": serialize_reply_markup(&reply_markup.into())?,
                }))
                .send()
                .await?,
        )
        .await?
        .json::<TelegramResult<Message>>()
        .await?
        .result)
    }

    /// <https://core.telegram.org/bots/api#sendphoto>
    pub async fn send_photo<RM: Into<Option<ReplyMarkup>>>(
        &self,
        chat_id: i64,
        photo: &str,
        caption: Option<&str>,
        parse_mode: Option<&str>,
        reply_markup: RM,
    ) -> Result<Message> {
        Ok(error_for_status(
            CLIENT
                .post(&format!("{}/sendPhoto", self.base_url))
                .json(&json!({
                    "chat_id": chat_id,
                    "photo": photo,
                    "caption": caption,
                    "parse_mode": parse_mode,
                    "reply_markup": serialize_reply_markup(&reply_markup.into())?,
                }))
                .send()
                .await?,
        )
        .await?
        .json::<TelegramResult<Message>>()
        .await?
        .result)
    }

    pub async fn answer_callback_query(&self, callback_query_id: &str) -> Result<bool> {
        Ok(error_for_status(
            CLIENT
                .get(&format!("{}/answerCallbackQuery", self.base_url))
                .json(&json!({ "callback_query_id": callback_query_id }))
                .send()
                .await?,
        )
        .await?
        .json::<TelegramResult<bool>>()
        .await?
        .result)
    }
}

async fn error_for_status(response: reqwest::Response) -> Result<reqwest::Response> {
    if response.status().is_client_error() || response.status().is_server_error() {
        Err(anyhow!("{}: {}", response.status(), response.text().await?,))
    } else {
        Ok(response)
    }
}

fn serialize_reply_markup(reply_markup: &Option<ReplyMarkup>) -> Result<String> {
    Ok(if let Some(reply_markup) = reply_markup {
        serde_json::to_string(reply_markup)?
    } else {
        "{}".into()
    })
}
