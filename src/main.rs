use std::{iter::FromIterator, str::FromStr};

use structopt::StructOpt;

use crate::{
    chat_bot::ChatBot,
    prelude::*,
    telegram::{notifier::Notifier, Telegram},
};

pub mod chat_bot;
pub mod client;
pub mod logging;
pub mod marktplaats;
pub mod math;
pub mod opts;
pub mod prelude;
pub mod redis;
pub mod result;
pub mod search_bot;
pub mod telegram;

#[async_std::main]
async fn main() -> Result {
    let opts = opts::Opts::from_args();
    let _sentry_guard = init_sentry(opts.sentry_dsn);

    logging::init()?;
    let redis = redis::open(opts.redis_db)?;

    futures::future::try_join3(
        search_bot::Bot::new(redis.get_async_std_connection().await?, opts.polling_interval_secs)
            .run(),
        ChatBot::new(
            Telegram::new(&opts.telegram_token),
            redis.get_async_std_connection().await?,
            HashSet::from_iter(opts.allowed_chat_ids),
        )
        .run(),
        Notifier::new(redis.get_async_std_connection().await?, Telegram::new(&opts.telegram_token))
            .run(),
    )
    .await?;

    Ok(())
}

fn init_sentry(dsn: Option<String>) -> Option<sentry::ClientInitGuard> {
    dsn.map(|dsn| {
        sentry::init(sentry::ClientOptions {
            dsn: Some(sentry::types::Dsn::from_str(&dsn).unwrap()),
            release: sentry::release_name!(),
            attach_stacktrace: true,
            ..Default::default()
        })
    })
}
