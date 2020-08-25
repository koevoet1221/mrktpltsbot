use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(author, about)]
pub struct Opts {
    /// Telegram bot token, get one at BotFather
    #[structopt(env = "MRKTPLTS_BOT_TELEGRAM_TOKEN")]
    pub telegram_token: String,

    /// Redis database
    #[structopt(
        default_value = "0",
        short = "d",
        long = "redis-db",
        env = "MRKTPLTS_BOT_REDIS_DB"
    )]
    pub redis_db: i64,
}
