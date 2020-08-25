use crate::prelude::*;
use lazy_static::lazy_static;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};

const USER_AGENT: &str = concat!(
    "mrktpltsbot / ",
    crate_version!(),
    " (Rust; https://github.com/eigenein/mrktpltsbot)",
);
const TIMEOUT: Duration = Duration::from_secs(30);
const POOL_IDLE_TIMEOUT: Duration = Duration::from_secs(600);

lazy_static! {
    pub static ref CLIENT: Client = Client::builder()
        .gzip(true)
        .use_rustls_tls()
        .default_headers({
            let mut headers = HeaderMap::new();
            headers.insert(
                reqwest::header::USER_AGENT,
                HeaderValue::from_static(USER_AGENT),
            );
            headers
        })
        .timeout(TIMEOUT)
        .pool_idle_timeout(Some(POOL_IDLE_TIMEOUT))
        .build()
        .unwrap();
}
