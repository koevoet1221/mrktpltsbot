use crate::prelude::*;

#[derive(Deserialize)]
#[must_use]
pub struct TelegramResult<T> {
    pub result: T,
}

/// <https://core.telegram.org/bots/api#botcommand>
#[derive(Serialize)]
#[must_use]
pub struct BotCommand {
    pub command: String,
    pub description: String,
}

#[derive(Deserialize)]
#[must_use]
pub struct Update {
    #[serde(rename = "update_id")]
    pub id: i64,

    #[serde(default)]
    pub message: Option<Message>,

    #[serde(default)]
    pub callback_query: Option<CallbackQuery>,
}

#[derive(Deserialize)]
#[must_use]
pub struct CallbackQuery {
    pub id: String,
    pub from: User,

    #[serde(default)]
    pub message: Option<Message>,

    #[serde(default)]
    pub data: Option<String>,
}

#[derive(Deserialize)]
pub struct Message {
    #[serde(rename = "message_id")]
    pub id: i64,

    pub from: Option<User>,

    pub text: Option<String>,

    pub chat: Chat,
}

#[derive(Deserialize)]
#[must_use]
pub struct User {
    pub id: i64,
}

#[derive(Deserialize)]
#[must_use]
pub struct Chat {
    pub id: i64,
}

#[derive(Serialize, Deserialize)]
#[must_use]
pub enum ReplyMarkup {
    #[serde(rename = "inline_keyboard")]
    InlineKeyboard(Vec<Vec<InlineKeyboardButton>>),
}

#[derive(Serialize, Deserialize)]
#[must_use]
pub struct InlineKeyboardButton {
    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_data: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
