//! Telegram bot [API].
//!
//! [API]: https://core.telegram.org/bots/api

use crate::prelude::*;

pub mod bot;

/// <https://core.telegram.org/bots/api>
pub struct Telegram {
    /// <https://core.telegram.org/bots#6-botfather>
    pub token: String,
}

/// <https://core.telegram.org/bots/api#setmycommands>
#[derive(Serialize)]
pub struct SetMyCommands {
    pub commands: Vec<BotCommand>,
}

/// <https://core.telegram.org/bots/api#botcommand>
#[derive(Serialize)]
pub struct BotCommand {
    pub command: String,
    pub description: String,
}

impl Telegram {
    pub async fn set_my_commands(&self, args: &SetMyCommands) -> Result {
        CLIENT
            .post(&format!(
                "https://api.telegram.org/bot{}/setMyCommands",
                self.token
            ))
            .json(&args)
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }
}
