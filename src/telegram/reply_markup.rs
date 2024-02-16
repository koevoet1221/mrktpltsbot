use crate::telegram::*;

/// Converts a button into a single-button inline keyboard.
impl From<InlineKeyboardButton> for ReplyMarkup {
    fn from(button: InlineKeyboardButton) -> Self {
        vec![button].into()
    }
}

/// Converts a single row into a single-row inline keyboard.
impl From<Vec<InlineKeyboardButton>> for ReplyMarkup {
    fn from(row: Vec<InlineKeyboardButton>) -> Self {
        vec![row].into()
    }
}

impl From<Vec<Vec<InlineKeyboardButton>>> for ReplyMarkup {
    fn from(buttons: Vec<Vec<InlineKeyboardButton>>) -> Self {
        Self::InlineKeyboard(buttons)
    }
}

impl InlineKeyboardButton {
    pub fn new_search_preview_button(query: &str) -> Self {
        Self {
            text: "🔎 Preview".into(),
            callback_data: Some(format!("/search {query}")),
            url: None,
        }
    }

    pub fn new_subscribe_button(query: &str) -> Self {
        Self {
            text: "✅ Subscribe".into(),
            callback_data: Some(format!("/subscribe {query}")),
            url: None,
        }
    }

    pub fn new_unsubscribe_button(subscription_id: i64, text: Option<String>) -> Self {
        Self {
            text: text.unwrap_or_else(|| "❌ Unsubscribe".into()),
            callback_data: Some(format!("/unsubscribe {subscription_id}")),
            url: None,
        }
    }

    pub fn new_unsubscribe_list_button() -> Self {
        Self {
            text: "❌ Show unsubscribe list".into(),
            callback_data: Some("/unsubscribe".to_string()),
            url: None,
        }
    }

    pub fn new_url_button(url: String) -> Self {
        Self {
            text: "🔗 View".into(),
            url: Some(url),
            callback_data: None,
        }
    }
}
