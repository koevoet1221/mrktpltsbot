pub mod bot;
pub mod commands;
pub mod methods;
pub mod objects;
pub mod reaction;
pub mod render;
pub mod result;

use std::fmt::Debug;

use secrecy::{ExposeSecret, SecretString};
use serde::de::DeserializeOwned;
use url::Url;

use crate::{
    client::Client,
    prelude::*,
    telegram::{
        commands::CommandBuilder,
        methods::{GetMe, Method},
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

    #[instrument(skip_all)]
    pub async fn command_builder(&self) -> Result<CommandBuilder> {
        let me = GetMe
            .call_on(self)
            .await
            .context("failed to get bot’s user")?
            .username
            .context("the bot has no username")?;
        CommandBuilder::new(&me)
    }
}
