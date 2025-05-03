use prost::Message;
use sqlx::SqliteConnection;

use crate::prelude::*;

pub struct KeyValues<'a>(pub &'a mut SqliteConnection);

impl KeyValues<'_> {
    #[instrument(skip_all, fields(key = key, value = ?value))]
    pub async fn upsert(&mut self, key: &str, value: &impl Message) -> Result {
        // language=sql
        const QUERY: &str = "
            INSERT INTO key_values (key, value) VALUES (?1, ?2)
            ON CONFLICT DO UPDATE SET value = ?2
        ";
        sqlx::query(QUERY)
            .bind(key)
            .bind(value.encode_to_vec())
            .execute(&mut *self.0)
            .await
            .context("failed to upsert the subscription")?;

        Ok(())
    }

    #[instrument(skip_all, fields(key = key), ret(level = Level::TRACE))]
    pub async fn fetch<V: Default + Message>(&mut self, key: &str) -> Result<Option<V>> {
        // language=sql
        const QUERY: &str = "SELECT value FROM key_values WHERE key = ?1";

        let value: Option<Vec<u8>> = sqlx::query_scalar(QUERY)
            .bind(key)
            .fetch_optional(&mut *self.0)
            .await
            .with_context(|| format!("failed to fetch the value for key `{key}`"))?;
        value.map_or_else(
            || Ok(None),
            |value| V::decode(value.as_slice()).context("failed to decode the value").map(Some),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;
    use crate::{db::Db, marketplace::vinted::client::AuthenticationTokens};

    #[tokio::test]
    async fn upsert_value_ok() -> Result {
        let db = Db::try_new(Path::new(":memory:")).await?;
        let mut connection = db.connection().await;
        let mut key_values = KeyValues(&mut connection);

        assert!(
            key_values.fetch::<AuthenticationTokens>(AuthenticationTokens::KEY).await?.is_none()
        );

        let tokens = AuthenticationTokens::builder().access("access").refresh("refresh").build();
        key_values.upsert(AuthenticationTokens::KEY, &tokens).await?;
        assert_eq!(key_values.fetch(AuthenticationTokens::KEY).await?, Some(tokens));

        Ok(())
    }
}
