use bon::Builder;
use url::Url;

#[derive(Builder)]
pub struct Seller {
    pub username: String,
    pub profile_url: Url,
}
