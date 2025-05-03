use bon::Builder;
use url::Url;

use self::{
    condition::Condition,
    delivery::Delivery,
    location::Location,
    price::Price,
    seller::Seller,
};

pub mod amount;
pub mod condition;
pub mod delivery;
pub mod location;
pub mod price;
pub mod seller;

/// Marketplace item.
#[derive(Builder)]
pub struct Item {
    pub id: String,
    pub url: Url,
    pub title: String,
    pub description: String,
    pub picture_url: Option<Url>,
    pub condition: Option<Condition>,
    pub delivery: Option<Delivery>,
    pub price: Price,
    pub seller: Seller,
    pub location: Option<Location>,
}
