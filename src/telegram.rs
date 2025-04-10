pub mod bot;
pub mod commands;
pub mod methods;
pub mod objects;
pub mod reaction;
pub mod render;
pub mod result;

use std::fmt::Debug;

use futures::{Stream, StreamExt, TryStreamExt, stream};
use secrecy::{ExposeSecret, SecretString};
use serde::de::DeserializeOwned;
use url::Url;

use crate::{
    client::Client,
    heartbeat::Heartbeat,
    prelude::*,
    telegram::{
        methods::{AllowedUpdate, GetUpdates, Method},
        objects::Update,
        result::TelegramResult,
    },
};

/// Telegram bot API connection.
#[must_use]
#[derive(Clone)]
pub struct Telegram {
    client: Client,
    token: SecretString,
    root_url: Url,
}

impl Telegram {
    pub fn new(client: Client, token: SecretString) -> Result<Self> {
        Ok(Self { client, token, root_url: Url::parse("https://api.telegram.org")? })
    }

    /// Call the Telegram Bot API method.
    #[instrument(skip_all)]
    pub async fn call<M, R>(&self, method: &M) -> Result<R>
    where
        M: Method + ?Sized,
        R: Debug + DeserializeOwned,
    {
        let mut url = self.root_url.clone();
        url.set_path(&format!("bot{}/{}", self.token.expose_secret(), method.name()));
        self.client
            .request(reqwest::Method::POST, url)
            .json(method)
            .timeout(method.timeout())
            .read_json::<TelegramResult<R>>(false)
            .await?
            .into()
    }

    /// Convert the client into a [`Stream`] of Telegram [`Update`]'s.
    pub fn into_updates<'a>(
        self,
        offset: u64,
        poll_timeout_secs: u64,
        heartbeat: &'a Heartbeat<'a>,
    ) -> impl Stream<Item = Result<Update>> + 'a {
        let advance = move |(this, offset)| async move {
            let updates = GetUpdates::builder()
                .offset(offset)
                .timeout_secs(poll_timeout_secs)
                .allowed_updates(&[AllowedUpdate::Message])
                .build()
                .call_on(&this)
                .await?;
            heartbeat.check_in().await;
            let next_offset = updates.last().map_or(offset, |last_update| last_update.id + 1);
            info!(n = updates.len(), next_offset, "Received Telegram updates");
            Ok::<_, Error>(Some((stream::iter(updates).map(Ok), (this, next_offset))))
        };
        stream::try_unfold((self, offset), advance).try_flatten()
    }
}
