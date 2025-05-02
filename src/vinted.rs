use reqwest::Method;
use secrecy::{ExposeSecret, SecretString};

use crate::{client::Client, prelude::*};

pub struct Vinted(pub Client);

impl Vinted {
    #[instrument(skip_all, err(level = Level::DEBUG))]
    pub async fn refresh_token(&self, refresh_token: &SecretString) -> Result<TokenPair> {
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
                access_token = Some(SecretString::from(cookie.value()));
            } else if cookie.name().eq_ignore_ascii_case("refresh_token_web") {
                refresh_token = Some(SecretString::from(cookie.value()));
            }
        }
        Ok(TokenPair {
            access: access_token.context("missing access token cookie")?,
            refresh: refresh_token.context("missing refresh token cookie")?,
        })
    }
}

#[must_use]
pub struct TokenPair {
    pub access: SecretString,
    pub refresh: SecretString,
}
