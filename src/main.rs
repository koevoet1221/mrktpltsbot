use structopt::StructOpt;

pub mod client;
pub mod logging;
pub mod marktplaats;
pub mod opts;
pub mod prelude;
pub mod redis;
pub mod telegram;

use crate::prelude::*;

#[async_std::main]
async fn main() -> Result {
    let opts = opts::Opts::from_args();

    logging::init()?;
    let redis = redis::open(opts.redis_db).await?;

    info!("Running…");
    futures::future::try_join3(
        marktplaats::bot::Bot { redis }.spawn(),
        telegram::bot::Bot::new(telegram::Telegram::new(&opts.telegram_token)).spawn_ui(),
        telegram::bot::Bot::new(telegram::Telegram::new(&opts.telegram_token)).spawn_notifier(),
    )
    .await?;
    Ok(())
}
