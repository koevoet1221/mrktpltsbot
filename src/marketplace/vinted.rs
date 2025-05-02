use bon::Builder;
use prost::Message;
use reqwest::Method;
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use url::Url;

use crate::{client::Client, marketplace::amount::Amount, prelude::*};

pub struct Vinted(pub Client);

impl Vinted {
    #[instrument(skip_all, err(level = Level::DEBUG))]
    pub async fn refresh_token(
        &self,
        refresh_token: &SecretString,
    ) -> Result<AuthenticationTokens> {
        let response = self
            .0
            .request(Method::POST, "https://www.vinted.com/web/api/auth/refresh")
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

#[derive(Deserialize)]
pub struct SearchResults {
    pub items: Vec<Item>,
}

#[derive(Deserialize)]
pub struct Item {
    pub id: i64,
    pub title: String,
    pub price: Price,
    pub url: Url,
    pub photo: Photo,
    pub user: User,
}

#[derive(Deserialize)]
pub struct Price {
    #[serde(deserialize_with = "Amount::deserialize_from_string")]
    pub amount: Amount,
}

#[derive(Deserialize)]
pub struct Photo {
    pub full_size_url: Url,
}

#[derive(Deserialize)]
pub struct User {
    pub login: String,
    pub profile_url: Url,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::too_many_lines)]
    fn test_deserialize_item_ok() -> Result {
        // language=json
        let _: Item = serde_json::from_str(
            r##"
            {
              "id": 6245197443,
              "title": "Unifi u6 pro",
              "price": {
                "amount": "135.0",
                "currency_code": "EUR"
              },
              "is_visible": true,
              "discount": null,
              "brand_title": "Ubiquiti",
              "path": "/items/6245197443-unifi-u6-pro",
              "user": {
                "id": 258360251,
                "login": "hertokken",
                "profile_url": "https://www.vinted.nl/member/258360251-hertokken",
                "photo": null,
                "business": false
              },
              "conversion": null,
              "url": "https://www.vinted.nl/items/6245197443-unifi-u6-pro",
              "promoted": false,
              "photo": {
                "id": 25350626557,
                "image_no": 1,
                "width": 600,
                "height": 800,
                "dominant_color": "#B4AFB3",
                "dominant_color_opaque": "#E9E7E8",
                "url": "https://images1.vinted.net/t/03_006f6_dTDx2rZxo5Ma5tTtWH8rW5v6/f800/1746107501.jpeg?s=a770eadce37dc98aa996920000668e0fbfcf180d",
                "is_main": true,
                "thumbnails": [
                  {
                    "type": "thumb70x100",
                    "url": "https://images1.vinted.net/t/03_006f6_dTDx2rZxo5Ma5tTtWH8rW5v6/70x100/1746107501.jpeg?s=47cb0126bc695fdfea2385d6733260dc9e9be634",
                    "width": 70,
                    "height": 100,
                    "original_size": null
                  },
                  {
                    "type": "thumb150x210",
                    "url": "https://images1.vinted.net/t/03_006f6_dTDx2rZxo5Ma5tTtWH8rW5v6/150x210/1746107501.jpeg?s=7aa83c5c7dfe9fe849c12b9964816f0bee8f94b2",
                    "width": 150,
                    "height": 210,
                    "original_size": null
                  },
                  {
                    "type": "thumb310x430",
                    "url": "https://images1.vinted.net/t/03_006f6_dTDx2rZxo5Ma5tTtWH8rW5v6/310x430/1746107501.jpeg?s=a836240144793a273808d7bea7f6ea7844d52c36",
                    "width": 310,
                    "height": 430,
                    "original_size": null
                  },
                  {
                    "type": "thumb428x624",
                    "url": "https://images1.vinted.net/t/03_006f6_dTDx2rZxo5Ma5tTtWH8rW5v6/f800/1746107501.jpeg?s=a770eadce37dc98aa996920000668e0fbfcf180d",
                    "width": 321,
                    "height": 428,
                    "original_size": true
                  },
                  {
                    "type": "thumb624x428",
                    "url": "https://images1.vinted.net/t/03_006f6_dTDx2rZxo5Ma5tTtWH8rW5v6/f800/1746107501.jpeg?s=a770eadce37dc98aa996920000668e0fbfcf180d",
                    "width": 468,
                    "height": 624,
                    "original_size": true
                  },
                  {
                    "type": "thumb364x428",
                    "url": "https://images1.vinted.net/t/03_006f6_dTDx2rZxo5Ma5tTtWH8rW5v6/f800/1746107501.jpeg?s=a770eadce37dc98aa996920000668e0fbfcf180d",
                    "width": 273,
                    "height": 364,
                    "original_size": true
                  }
                ],
                "high_resolution": {
                  "id": "03_006f6_dTDx2rZxo5Ma5tTtWH8rW5v6",
                  "timestamp": 1746107501,
                  "orientation": null
                },
                "is_suspicious": false,
                "full_size_url": "https://images1.vinted.net/tc/03_006f6_dTDx2rZxo5Ma5tTtWH8rW5v6/1746107501.jpeg?s=c9944e399ea422797bdcb1731e09eeb0e4a62d75",
                "is_hidden": false,
                "extra": {}
              },
              "favourite_count": 3,
              "is_favourite": false,
              "badge": null,
              "service_fee": {
                "amount": "7.45",
                "currency_code": "EUR"
              },
              "total_item_price": {
                "amount": "142.45",
                "currency_code": "EUR"
              },
              "view_count": 0,
              "size_title": "",
              "content_source": "search",
              "status": "Heel goed",
              "icon_badges": [],
              "item_box": {
                "first_line": "Ubiquiti",
                "second_line": "Heel goed",
                "exposure": {
                  "test_id": "67037",
                  "test_name": "item_high_demand_signal",
                  "variant": "on",
                  "test_anon_id": "2ca03c9c-7204-4499-82f5-80b6af39af85",
                  "test_user_id": "246887296",
                  "country_code": "NL"
                },
                "accessibility_label": "Unifi u6 pro, merk: Ubiquiti, staat: Heel goed, 135,00 €, 142,45 € inclusief Kopersbescherming",
                "badge": {
                  "title": "Populair"
                }
              },
              "search_tracking_params": {
                "score": -4.771793842315674,
                "matched_queries": []
              }
            }"##,
        )?;
        Ok(())
    }
}
