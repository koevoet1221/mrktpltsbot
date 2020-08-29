use std::str::FromStr;
use structopt::StructOpt;

pub mod client;
pub mod logging;
pub mod marktplaats;
pub mod opts;
pub mod prelude;
pub mod redis;
pub mod telegram;
pub mod tokenize;

use crate::prelude::*;

#[async_std::main]
async fn main() -> Result {
    let opts = opts::Opts::from_args();
    let _sentry_guard = init_sentry(opts.sentry_dsn);

    logging::init()?;
    let redis = redis::open(opts.redis_db).await?;

    info!("Running…");
    futures::future::try_join(
        marktplaats::bot::Bot::new(redis.get_async_std_connection().await?).spawn(),
        telegram::chat_bot::ChatBot::new(
            telegram::Telegram::new(&opts.telegram_token),
            redis.get_async_std_connection().await?,
        )
        .spawn(),
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
