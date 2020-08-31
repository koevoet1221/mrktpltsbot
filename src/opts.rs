use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(author, about)]
pub struct Opts {
    /// Telegram bot token, get a one from the BotFather
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

    /// Optional Sentry DSN for monitoring of the bot
    #[structopt(long = "sentry-dsn", env = "MRKTPLTS_BOT_SENTRY_DSN")]
    pub sentry_dsn: Option<String>,

    /// Chat IDs that are allowed to interact with the bot
    #[structopt(
        short = "c",
        long = "allowed-chat",
        env = "MRKTPLTS_BOT_ALLOWED_CHAT_IDS"
    )]
    pub allowed_chat_ids: Vec<i64>,
}
