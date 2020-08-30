//! Redis extensions.

use crate::prelude::*;
use redis::aio::ConnectionLike;
use redis::{Client, ConnectionAddr, ConnectionInfo, FromRedisValue, ToRedisArgs};

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

pub async fn sadd_many<C, V>(connection: &mut C, key: &str, values: V) -> Result
where
    C: ConnectionLike,
    V: IntoIterator,
    V::Item: ToRedisArgs,
{
    let mut cmd = redis::cmd("SADD");
    cmd.arg(key);
    for value in values.into_iter() {
        cmd.arg(value);
    }
    cmd.query_async(connection).await?;
    Ok(())
}
