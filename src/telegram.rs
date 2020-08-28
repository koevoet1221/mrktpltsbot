//! Telegram bot [API].
//!
//! [API]: https://core.telegram.org/bots/api

use crate::prelude::*;

pub mod ui_bot;

const GET_UPDATES_TIMEOUT: u64 = 60;
const GET_UPDATES_REQUEST_TIMEOUT: Duration = Duration::from_secs(GET_UPDATES_TIMEOUT + 1);

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
    pub id: i64,

    #[serde(default)]
    pub message: Option<Message>,
}

#[derive(Deserialize)]
pub struct Message {
    pub from: Option<User>,

    pub text: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub id: i64,
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

    pub async fn get_updates(
        &self,
        offset: i64,
        allowed_updates: Vec<&'static str>,
    ) -> Result<Vec<Update>> {
        Ok(CLIENT
            .get(&format!("{}/getUpdates", self.base_url))
            .json(&json!({ "offset": offset, "allowed_updates": allowed_updates, "timeout": GET_UPDATES_TIMEOUT }))
            .timeout(GET_UPDATES_REQUEST_TIMEOUT)
            .send()
            .await?
            .error_for_status()?
            .json::<TelegramResult<Vec<Update>>>()
            .await?
            .result)
    }
}
