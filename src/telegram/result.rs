use monostate::MustBe;
use serde::Deserialize;

use crate::prelude::*;

/// Telegram bot API [response][1].
///
/// [1]: https://core.telegram.org/bots/api#making-requests
#[derive(Deserialize)]
#[must_use]
#[serde(untagged)]
pub enum TelegramResult<T> {
    Ok { ok: MustBe!(true), result: T },
    Err { ok: MustBe!(false), description: String, error_code: i32 },
}

impl<T> From<TelegramResult<T>> for Result<T> {
    fn from(result: TelegramResult<T>) -> Self {
        match result {
            TelegramResult::Ok { result, .. } => Ok(result),
            TelegramResult::Err { error_code, description, .. } => {
                Err(anyhow!("API error {error_code}: {description}"))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_ok() -> Result {
        // language=json
        let response: TelegramResult<u32> = serde_json::from_str(r#"{"ok": true, "result": 42}"#)?;
        match response {
            TelegramResult::Ok { result, .. } => {
                assert_eq!(result, 42);
            }
            TelegramResult::Err { .. } => unreachable!(),
        }
        Ok(())
    }
}
