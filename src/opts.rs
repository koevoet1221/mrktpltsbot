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

    /// Sentry DSN.
    #[structopt(long = "sentry-dsn", env = "MRKTPLTS_BOT_SENTRY_DSN")]
    pub sentry_dsn: Option<String>,

    #[structopt(short = "c", long = "allowed-chat", env = "MRKTPLTS_BOT_ALLOWED_CHATS")]
    pub allowed_chats: Vec<crate::telegram::ChatId>,
}
