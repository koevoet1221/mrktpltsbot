#![doc = include_str!("../README.md")]

use std::time::Duration;

use clap::Parser;
use secrecy::ExposeSecret;

use crate::{
    cli::{Args, Command, RunArgs, VintedCommand},
    db::{Db, key_values::KeyValues},
    heartbeat::Heartbeat,
    marketplace::{
        marktplaats,
        marktplaats::{Marktplaats, client::MarktplaatsClient},
        search_bot::SearchBot,
        vinted::{Vinted, client::VintedClient},
    },
    prelude::*,
    telegram::Telegram,
};

mod cli;
mod client;
mod db;
mod heartbeat;
mod logging;
mod marketplace;
mod prelude;
mod serde;
mod telegram;

fn main() -> Result {
    let dotenv_result = dotenvy::dotenv();
    let cli = Args::parse();
    let _tracing_guards = logging::init(cli.sentry_dsn.as_deref())?;
    if let Err(error) = dotenv_result {
        warn!("Could not load `.env`: {error:#}");
    }
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async_main(cli))
        .inspect_err(|error| error!("Fatal error: {error:#}"))
}

async fn async_main(cli: Args) -> Result {
    let db = Db::try_new(&cli.db).await?;
    match cli.command {
        Command::Run(args) => run(db, *args).await,
        Command::Vinted { command } => manage_vinted(db, command).await,
    }
}

/// Run the bot indefinitely.
async fn run(db: Db, args: RunArgs) -> Result {
    let client = client::try_new()?;
    let telegram = Telegram::new(client.clone(), args.telegram.bot_token.into())?;
    let command_builder = telegram.command_builder().await?;

    // Marktplaats connection:
    let marktplaats = Marktplaats::builder()
        .client(MarktplaatsClient(client.clone()))
        .search_limit(args.marktplaats.marktplaats_search_limit)
        .heartbeat(Heartbeat::new(client.clone(), args.marktplaats.heartbeat_url))
        .build();

    // Vinted connection:
    let vinted = Vinted::builder()
        .client(VintedClient(client.clone()))
        .search_limit(args.vinted.vinted_search_limit)
        .db(db.clone())
        .build();

    // Telegram bot:
    let telegram_bot = telegram::bot::Bot::builder()
        .telegram(telegram.clone())
        .authorized_chat_ids(args.telegram.authorized_chat_ids.into_iter().collect())
        .db(db.clone())
        .marktplaats(marktplaats.clone())
        .vinted(vinted.clone())
        .poll_timeout_secs(args.telegram.poll_timeout_secs)
        .heartbeat(Heartbeat::new(client, args.telegram.heartbeat_url))
        .command_builder(command_builder.clone())
        .try_init()
        .await?;

    // Search bot:
    let search_bot = SearchBot::builder()
        .db(db)
        .search_interval(Duration::from_secs(args.search_interval_secs))
        .marktplaats(marktplaats)
        .telegram(telegram)
        .command_builder(command_builder)
        .build();

    // Run the bots:
    tokio::try_join!(tokio::spawn(telegram_bot.run()), tokio::spawn(search_bot.run()))?;
    Ok(())
}

/// Manage Vinted settings.
async fn manage_vinted(db: Db, command: VintedCommand) -> Result {
    let vinted = VintedClient(client::try_new()?);
    match command {
        VintedCommand::Authenticate { refresh_token } => {
            let tokens = vinted.refresh_token(refresh_token.expose_secret()).await?;
            info!(tokens.access, tokens.refresh);
            KeyValues(&mut *db.connection().await).upsert(&tokens).await?;
        }
    }
    Ok(())
}
