//! Telegram bot [API].
//!
//! [API]: https://core.telegram.org/bots/api

use crate::prelude::*;
use std::borrow::Cow;

pub type ChatId = i64;
pub type UpdateId = i64;
pub type UserId = i64;

const GET_UPDATES_TIMEOUT: u64 = 60;
const GET_UPDATES_REQUEST_TIMEOUT: Duration = Duration::from_secs(GET_UPDATES_TIMEOUT + 1);

lazy_static! {
    static ref ESCAPE_MARKDOWN_V2_REGEX: regex::Regex =
        regex::Regex::new(r"[_\*\[\]\(\)\~`>\#\+\-=\|\{\}\.!]").unwrap();
}

/// <https://core.telegram.org/bots/api>
pub struct Telegram {
    /// <https://core.telegram.org/bots#6-botfather>
    base_url: String,
}

/// <https://core.telegram.org/bots/api#botcommand>
#[derive(Serialize)]
pub struct BotCommand {
    pub command: String,
    pub description: String,
}

#[derive(Deserialize)]
struct TelegramResult<T> {
    result: T,
}

#[derive(Deserialize)]
pub struct Update {
    #[serde(rename = "update_id")]
    pub id: UpdateId,

    #[serde(default)]
    pub message: Option<Message>,

    #[serde(default)]
    pub callback_query: Option<CallbackQuery>,
}

#[derive(Deserialize)]
pub struct CallbackQuery {
    pub id: String,
    pub from: User,

    #[serde(default)]
    pub message: Option<Message>,

    #[serde(default)]
    pub data: Option<String>,
}

#[derive(Deserialize)]
pub struct Message {
    #[serde(rename = "message_id")]
    pub id: i64,

    pub from: Option<User>,

    pub text: Option<String>,

    pub chat: Chat,
}

#[derive(Deserialize)]
pub struct User {
    pub id: UserId,
}

#[derive(Deserialize)]
pub struct Chat {
    pub id: ChatId,
}

#[derive(Serialize)]
pub enum ReplyMarkup {
    #[serde(rename = "inline_keyboard")]
    InlineKeyboard(Vec<Vec<InlineKeyboardButton>>),
}

#[derive(Serialize)]
pub struct InlineKeyboardButton {
    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_data: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl Telegram {
    pub fn new(token: &str) -> Self {
        Self {
            base_url: format!("https://api.telegram.org/bot{}", token),
        }
    }

    /// <https://core.telegram.org/bots/api#setmycommands>
    pub async fn set_my_commands(&self, commands: Vec<BotCommand>) -> Result {
        CLIENT
            .post(&format!("{}/setMyCommands", self.base_url))
            .json(&json!({ "commands": commands }))
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    /// <https://core.telegram.org/bots/api#getupdates>
    pub async fn get_updates(
        &self,
        offset: i64,
        allowed_updates: Vec<&'static str>,
    ) -> Result<Vec<Update>> {
        Ok(CLIENT
            .get(&format!("{}/getUpdates", self.base_url))
            .json(&json!({
                "offset": offset,
                "allowed_updates": allowed_updates,
                "timeout": GET_UPDATES_TIMEOUT,
            }))
            .timeout(GET_UPDATES_REQUEST_TIMEOUT)
            .send()
            .await?
            .error_for_status()?
            .json::<TelegramResult<Vec<Update>>>()
            .await?
            .result)
    }

    /// <https://core.telegram.org/bots/api#sendmessage>
    pub async fn send_message(
        &self,
        chat_id: ChatId,
        text: &str,
        parse_mode: Option<&str>,
        reply_markup: Option<ReplyMarkup>,
    ) -> Result<Message> {
        Ok(CLIENT
            .post(&format!("{}/sendMessage", self.base_url))
            .json(&json!({
                "chat_id": chat_id,
                "text": text,
                "parse_mode": parse_mode,
                "reply_markup": if let Some(reply_markup) = reply_markup {
                    serde_json::to_string(&reply_markup)?
                } else {
                    "{}".into()
                },
            }))
            .send()
            .await?
            .error_for_status()?
            .json::<TelegramResult<Message>>()
            .await?
            .result)
    }
}

pub fn escape_markdown_v2(text: &str) -> Cow<str> {
    ESCAPE_MARKDOWN_V2_REGEX.replace_all(text, r"\$0")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_markdown_v2_ok() {
        assert_eq!(escape_markdown_v2("Hello, world!"), r"Hello, world\!");
        assert_eq!(escape_markdown_v2("hello, world"), r"hello, world");
        assert_eq!(
            escape_markdown_v2("Philips Hue GU10 White and Color Ambiance Splinternieuw!"),
            r"Philips Hue GU10 White and Color Ambiance Splinternieuw\!",
        );
    }
}
