use bon::Builder;
use url::Url;

pub use self::{
    amount::Amount,
    condition::{Condition, New, Used},
    delivery::Delivery,
    location::{GeoLocation, Location},
    price::Price,
    seller::Seller,
};

mod amount;
mod condition;
mod delivery;
mod location;
mod price;
mod seller;

/// Marketplace item.
#[derive(Builder)]
pub struct Item {
    pub id: String,
    pub url: Url,
    pub title: String,
    pub description: Option<String>,
    pub picture_url: Option<Url>,
    pub condition: Option<Condition>,
    pub delivery: Option<Delivery>,
    pub price: Price,
    pub seller: Seller,
    pub location: Option<Location>,
}
