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
        ReplyMarkup::InlineKeyboard(buttons)
    }
}
