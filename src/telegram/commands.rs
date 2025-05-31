//! `/start` command.

use bon::Builder;
use prost::{Enumeration, Message};
use url::Url;

use crate::{prelude::*, telegram::render::CommandLink};

/// Builder of `/start` commands with [deep linking][1].
///
/// [1]: https://core.telegram.org/bots/features#deep-linking
#[derive(Clone)]
#[must_use]
pub struct CommandBuilder(Url);

impl CommandBuilder {
    pub fn new(me: &str) -> Result<Self> {
        let mut base_url = Url::parse("https://t.me/")?;
        base_url.set_path(me);
        Ok(Self(base_url))
    }

    /// Return the command builder base URL.
    pub const fn url(&self) -> &Url {
        &self.0
    }

    /// Build a new command link.
    pub fn command_link(&self, content: &'static str, payload: &CommandPayload) -> CommandLink {
        let mut url = self.0.clone();
        url.query_pairs_mut().append_pair("start", &payload.to_base64());
        CommandLink { content, url }
    }

    /// Produce «Manage subscriptions» link.
    pub fn manage_link(&self) -> CommandLink {
        self.command_link("Manage subscriptions", &CommandPayload::manage())
    }

    /// Produce a standard «Subscribe» link.
    pub fn subscribe_link(&self, to_query_hash: i64) -> CommandLink {
        self.command_link("Subscribe", &CommandPayload::subscribe_to(to_query_hash))
    }

    /// Produce a standard «Re-subscribe» link.
    pub fn resubscribe_link(&self, to_query_hash: i64) -> CommandLink {
        self.command_link("Re-subscribe", &CommandPayload::subscribe_to(to_query_hash))
    }

    /// Produce a standard «Unsubscribe» link.
    pub fn unsubscribe_link(&self, from_query_hash: i64) -> CommandLink {
        self.command_link("Unsubscribe", &CommandPayload::unsubscribe_from(from_query_hash))
    }
}

/// Payload for a `/start` command with a [deep link][1].
///
/// [1]: https://core.telegram.org/bots/features#deep-linking
#[derive(Builder, Message)]
pub struct CommandPayload {
    #[prost(tag = "3", message, optional)]
    pub subscription: Option<SubscriptionCommand>,

    #[prost(tag = "4", message, optional)]
    pub manage: Option<ManageCommand>,
}

impl CommandPayload {
    pub fn from_base64(text: &str) -> Result<Self> {
        let payload = base64_url::decode(text).context("failed to decode the payload")?;
        Self::decode(payload.as_slice()).context("failed to deserialize the payload")
    }

    pub fn to_base64(&self) -> String {
        base64_url::encode(&self.encode_to_vec())
    }

    pub const fn manage() -> Self {
        Self { subscription: None, manage: Some(ManageCommand {}) }
    }

    pub const fn subscribe_to(query_hash: i64) -> Self {
        Self { subscription: Some(SubscriptionCommand::subscribe_to(query_hash)), manage: None }
    }

    pub const fn unsubscribe_from(query_hash: i64) -> Self {
        Self { subscription: Some(SubscriptionCommand::unsubscribe_from(query_hash)), manage: None }
    }
}

/// List the user's subscriptions.
#[derive(Message)]
pub struct ManageCommand {}

#[derive(Eq, PartialEq, Message)]
pub struct SubscriptionCommand {
    #[prost(tag = "1", sfixed64)]
    pub query_hash: i64,

    #[prost(tag = "2", enumeration = "SubscriptionAction")]
    pub action: i32,
}

impl SubscriptionCommand {
    pub const fn subscribe_to(query_hash: i64) -> Self {
        Self { query_hash, action: SubscriptionAction::Subscribe as i32 }
    }

    pub const fn unsubscribe_from(query_hash: i64) -> Self {
        Self { query_hash, action: SubscriptionAction::Unsubscribe as i32 }
    }
}

#[derive(Debug, Enumeration)]
#[repr(i32)]
pub enum SubscriptionAction {
    None = 0,
    Subscribe = 1,
    Unsubscribe = 2,
}

#[cfg(test)]
mod tests {
    use maud::Render;

    use super::*;
    use crate::db::SearchQuery;

    #[test]
    fn test_build_subscribe_link_ok() -> Result {
        let search_query = SearchQuery::from("unifi");
        let link = CommandBuilder::new("mrktpltsbot")?.subscribe_link(search_query.hash);

        // language=html
        assert_eq!(
            link.render().into_string(),
            r#"<a href="https://t.me/mrktpltsbot?start=GgsJ_5xfEFkYbu0QAQ">Subscribe</a>"#,
        );

        Ok(())
    }

    #[test]
    fn test_deserialize_payload_ok() -> Result {
        let payload = CommandPayload::from_base64("GgsJ_5xfEFkYbu0QAQ")?;
        assert_eq!(
            payload.subscription,
            Some(SubscriptionCommand::subscribe_to(-1_338_105_268_476_601_089))
        );
        Ok(())
    }
}
