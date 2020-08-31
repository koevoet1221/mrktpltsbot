//! Telegram bot [API].
//!
//! [API]: https://core.telegram.org/bots/api

use crate::prelude::*;
use crate::telegram::types::*;
use std::borrow::Cow;

pub mod reply_markup;
pub mod types;

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
    pub async fn get_updates(&self, offset: i64, allowed_updates: &[&str]) -> Result<Vec<Update>> {
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
    pub async fn send_message<RM: Into<Option<ReplyMarkup>>>(
        &self,
        chat_id: i64,
        text: &str,
        parse_mode: Option<&str>,
        reply_markup: RM,
    ) -> Result<Message> {
        let reply_markup = reply_markup.into();
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
