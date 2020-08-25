use crate::prelude::*;
use redis::aio::Connection;
use redis::{Client, ConnectionAddr, ConnectionInfo};

pub async fn open(db: i64) -> Result<Connection> {
    info!("Connecting to Redis #{}…", db);
    Ok(Client::open(ConnectionInfo {
        addr: ConnectionAddr::Tcp("localhost".into(), 6379).into(),
        db,
        username: None,
        passwd: None,
    })?
    .get_async_std_connection()
    .await?)
}
