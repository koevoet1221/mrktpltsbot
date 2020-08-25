use futures::future::try_join;
use structopt::StructOpt;

mod bot;
mod client;
mod logging;
mod marktplaats;
mod opts;
mod prelude;
mod redis;
mod telegram;

use crate::prelude::*;

#[async_std::main]
async fn main() -> Result {
    let opts = opts::Opts::from_args();

    logging::init()?;
    redis::open(opts.redis_db).await?;
    let telegram = telegram::Telegram {
        token: opts.telegram_token,
    };
    bot::init(&telegram).await?;

    info!("Running…");
    try_join(bot::spawn(telegram), futures::future::ready(Ok(()))).await?;
    Ok(())
}
