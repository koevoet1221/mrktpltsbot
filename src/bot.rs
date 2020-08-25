use crate::prelude::*;
use crate::telegram::*;

pub async fn init(telegram: &Telegram) -> Result {
    set_my_commands(telegram).await?;
    Ok(())
}

pub async fn spawn(telegram: Telegram) -> Result {
    Ok(())
}

async fn set_my_commands(telegram: &Telegram) -> Result {
    info!("Setting the bot commands…");
    telegram
        .set_my_commands(&SetMyCommands {
            commands: vec![BotCommand {
                command: "/list".into(),
                description: "List the saved searches".into(),
            }],
        })
        .await
}
