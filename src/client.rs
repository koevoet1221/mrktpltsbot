//! Provides the global `Client` instance.

use std::time::Duration;

use clap::crate_version;
use reqwest::{
    Client,
    header,
    header::{HeaderMap, HeaderValue},
};

use crate::prelude::*;

pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(10);
const USER_AGENT: &str = concat!(
    "mrktpltsbot / ",
    crate_version!(),
    " (Rust; https://github.com/eigenein/mrktpltsbot)",
);

pub fn try_new() -> Result<Client> {
    let mut headers = HeaderMap::new();
    headers.insert(header::USER_AGENT, HeaderValue::from_static(USER_AGENT));
    Client::builder()
        .gzip(true)
        .use_rustls_tls()
        .default_headers(headers)
        .timeout(DEFAULT_TIMEOUT)
        .connect_timeout(DEFAULT_TIMEOUT)
        .pool_idle_timeout(Some(Duration::from_secs(300)))
        .build()
        .context("failed to build an HTTP client")
}
