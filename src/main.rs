use futures::future::try_join;
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
    try_join(
        marktplaats::bot::Bot { redis }.spawn(),
        telegram::bot::Bot {
            telegram: telegram::Telegram {
                token: opts.telegram_token,
            },
        }
        .spawn(),
    )
    .await?;
    Ok(())
}
