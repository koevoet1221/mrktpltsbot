use crate::prelude::*;
use crate::telegram::*;

pub struct Bot {
    pub telegram: Telegram,
}

impl Bot {
    pub async fn spawn(self) -> Result {
        self.set_my_commands().await?;
        Ok(())
    }

    /// Set the Telegram bot commands.
    async fn set_my_commands(&self) -> Result {
        info!("Setting the bot commands…");
        self.telegram
            .set_my_commands(&SetMyCommands {
                commands: vec![BotCommand {
                    command: "/list".into(),
                    description: "List the saved searches".into(),
                }],
            })
            .await?;
        Ok(())
    }
}
