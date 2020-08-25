use structopt::StructOpt;

mod logging;
mod opts;
mod prelude;
mod redis;

use crate::prelude::*;

#[async_std::main]
async fn main() -> Result {
    let opts = opts::Opts::from_args();
    logging::init()?;
    redis::open(opts.redis_db).await?;

    Ok(())
}
