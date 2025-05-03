use reqwest::{Client, Response};
use url::Url;

use crate::prelude::*;

#[derive(Clone)]
pub struct Heartbeat(Option<HeartbeatInner>);

impl Heartbeat {
    pub fn new(client: Client, url: Option<Url>) -> Self {
        Self(url.map(|url| HeartbeatInner { client, url }))
    }

    pub async fn check_in(&self) {
        if let Some(inner) = &self.0 {
            if let Err(error) = inner
                .client
                .post(inner.url.clone())
                .send()
                .await
                .and_then(Response::error_for_status)
            {
                warn!("Failed to send the heartbeat: {error:#}");
            }
        }
    }
}

#[derive(Clone)]
struct HeartbeatInner {
    client: Client,
    url: Url,
}
