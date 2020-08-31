//! Redis extensions.

use crate::prelude::*;
use redis::aio::ConnectionLike;
use redis::{Client, ConnectionAddr, ConnectionInfo, FromRedisValue, ToRedisArgs};

const SUBSCRIPTION_COUNTER_KEY: &str = "subscriptions::counter";
const ALL_SUBSCRIPTIONS_KEY: &str = "subscriptions::all";

/// Open the Redis connection.
pub async fn open(db: i64) -> Result<Client> {
    info!("Connecting to Redis #{}…", db);
    Ok(Client::open(ConnectionInfo {
        addr: ConnectionAddr::Tcp("localhost".into(), 6379).into(),
        db,
        username: None,
        passwd: None,
    })?)
}

/// Set the value if not exists with the expiry time.
pub async fn set_nx_ex<C, V, R>(
    connection: &mut C,
    key: &str,
    value: V,
    expiry_time: u64,
) -> Result<R>
where
    C: ConnectionLike,
    V: ToRedisArgs,
    R: FromRedisValue,
{
    Ok(redis::cmd("SET")
        .arg(key)
        .arg(value)
        .arg("NX")
        .arg("EX")
        .arg(expiry_time)
        .query_async(connection)
        .await?)
}

/// Store the subscription in the Redis database.
pub async fn subscribe_to<C: AsyncCommands>(
    connection: &mut C,
    chat_id: i64,
    query: &str,
) -> Result<(i64, i64)> {
    let subscription_id = new_subscription_id(connection).await?;
    info!("New subscription #{}.", subscription_id);
    let subscription_count = enable_subscription(connection, subscription_id).await?;
    Ok((subscription_id, subscription_count))
}

/// Reserve and return a new subscription ID.
async fn new_subscription_id<C: AsyncCommands>(connection: &mut C) -> Result<i64> {
    Ok(connection.incr(SUBSCRIPTION_COUNTER_KEY, 1).await?)
}

/// Enable the subscription so that it will be picked up by the search bot.
/// Returns the total number of active subscriptions.
async fn enable_subscription<C: AsyncCommands>(
    connection: &mut C,
    subscription_id: i64,
) -> Result<i64> {
    connection
        .sadd(ALL_SUBSCRIPTIONS_KEY, subscription_id)
        .await?;
    Ok(connection.scard(ALL_SUBSCRIPTIONS_KEY).await?)
}
