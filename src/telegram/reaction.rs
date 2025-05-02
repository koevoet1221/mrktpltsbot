use std::borrow::Cow;

use bon::bon;
use serde::Serialize;

use crate::{
    marktplaats::listing::Picture,
    prelude::*,
    telegram::{
        Telegram,
        methods::{Method, SendMessage, SendPhoto},
        objects::{ChatId, LinkPreviewOptions, ParseMode, ReplyParameters},
    },
};

/// Reaction method on Telegram.
#[derive(Serialize)]
#[serde(untagged)]
#[must_use]
pub enum ReactionMethod<'a> {
    Message(SendMessage<'a>),
    Photo(SendPhoto<'a>),
}

#[bon]
impl<'a> ReactionMethod<'a> {
    /// Build a new reaction method from a listing contents.
    #[builder]
    pub fn new(
        chat_id: Cow<'a, ChatId>,
        text: Cow<'a, str>,
        parse_mode: ParseMode,
        picture: Option<&'a Picture>,
        reply_parameters: Option<ReplyParameters>,
    ) -> Self {
        // Specific representation depends on how many pictures there are.
        match picture.and_then(Picture::to_url) {
            None => Self::Message(
                SendMessage::builder()
                    .chat_id(chat_id)
                    .text(text)
                    .parse_mode(parse_mode)
                    .link_preview_options(LinkPreviewOptions::DISABLED)
                    .maybe_reply_parameters(reply_parameters)
                    .build(),
            ),

            Some(url) => Self::Photo(
                SendPhoto::builder()
                    .chat_id(chat_id)
                    .photo(url)
                    .caption(text)
                    .parse_mode(parse_mode)
                    .maybe_reply_parameters(reply_parameters)
                    .build(),
            ),
        }
    }
}

impl ReactionMethod<'_> {
    pub async fn react_to(&self, telegram: &Telegram) -> Result {
        match self {
            ReactionMethod::Message(inner) => inner.call_and_discard_on(telegram).await,
            ReactionMethod::Photo(inner) => inner.call_and_discard_on(telegram).await,
        }
    }
}
