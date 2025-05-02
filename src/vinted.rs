use bon::Builder;
use prost::Message;
use reqwest::Method;
use secrecy::{ExposeSecret, SecretString};

use crate::{client::Client, prelude::*};

pub struct Vinted(pub Client);

impl Vinted {
    #[instrument(skip_all, err(level = Level::DEBUG))]
    pub async fn refresh_token(
        &self,
        refresh_token: &SecretString,
    ) -> Result<AuthenticationTokens> {
        let response = self
            .0
            .request(Method::POST, "https://www.vinted.nl/web/api/auth/refresh")
            .header("Cookie", format!("refresh_token_web={}", refresh_token.expose_secret()))
            .send(true)
            .await?;
        let mut access_token = None;
        let mut refresh_token = None;
        for cookie in response.cookies() {
            if cookie.name().eq_ignore_ascii_case("access_token_web") {
                access_token = Some(cookie.value().to_string());
            } else if cookie.name().eq_ignore_ascii_case("refresh_token_web") {
                refresh_token = Some(cookie.value().to_string());
            }
        }
        Ok(AuthenticationTokens::builder()
            .access(access_token.context("missing access token cookie")?)
            .refresh(refresh_token.context("missing refresh token cookie")?)
            .build())
    }
}

#[must_use]
#[derive(PartialEq, Eq, Builder, Message)]
pub struct AuthenticationTokens {
    #[builder(into)]
    #[prost(tag = "1", string)]
    pub access: String,

    #[builder(into)]
    #[prost(tag = "2", string)]
    pub refresh: String,
}

impl AuthenticationTokens {
    /// Database key-value store key.
    pub const KEY: &'static str = "vinted::auth";
}
