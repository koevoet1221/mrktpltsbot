//! Listing rendering in Telegram.

use std::borrow::Cow;

use bon::Builder;
use maud::{Markup, PreEscaped, Render, html};
use url::Url;

use crate::{
    marketplace::item::{Amount, Condition, Delivery, GeoLocation, Item, Location, Price, Seller},
    telegram::objects::ChatId,
};

/// Just `<strong> • </strong>`.
pub const DELIMITER: PreEscaped<&'static str> = PreEscaped(
    // language=html
    "<strong> • </strong>",
);

pub fn unauthorized(chat_id: &ChatId) -> Markup {
    html! {
        "👋 Thank you for your interest"
        "\n\n"
        "This bot cannot handle many users, so it is private and only intended for authorized users."
        "\n\n"
        "However, " strong { "its " a href="https://github.com/eigenein/mrktpltsbot" { "source code" } " is open" } ","
        " and you are free to deploy your own instance."
        "\n\n"
        "If you are already setting it up for yourself, or someone is setting it up for you,"
        " "
        strong { "the following ID should be added to the list of authorized chat IDs:" }
        "\n\n"
        pre { code { (chat_id) } }
    }
}

/// Render the item description.
pub fn item_description<M: Render>(
    item: &Item,
    manage_search_query: &ManageSearchQuery<'_, M>,
) -> String {
    let markup = html! {
        strong { a href=(item.url) { (item.title) } }
        "\n"
        (manage_search_query)
        "\n\n"
        (item.price)
        @if let Some(condition) = item.condition {
            (DELIMITER)
            (condition)
        }
        @if let Some(delivery) = item.delivery {
            (DELIMITER)
            (delivery)
        }
        "\n\n"
        blockquote { (item.description) }
        "\n\n"
        (item.seller)
        @if let Some(location) = &item.location {
            (DELIMITER)
            (location)
        }
    };
    markup.render().into_string()
}

#[derive(Builder)]
pub struct Link<C> {
    content: C,
    url: Url,
}

impl<C: Render> Render for Link<C> {
    fn render(&self) -> Markup {
        html! { a href=(self.url) { (self.content) } }
    }
}

impl Render for ChatId {
    fn render(&self) -> Markup {
        html! {
            @match self {
                Self::Integer(chat_id) => code { (chat_id) },
                Self::Username(username) => code { (username) },
            }
        }
    }
}

impl Render for Price {
    fn render(&self) -> Markup {
        html! {
            @match self {
                Self::Fixed(asking) if *asking == Amount::ZERO => { em { "🆓 free" } }
                Self::Fixed(asking) => { strong { (asking) } }
                Self::OnRequest => { "🙋price on request" }
                Self::MinimalBid(asking) => { strong { (asking) } (DELIMITER) "⬆️ bidding" }
                Self::MaximalBid(asking) => { strong { (asking) } (DELIMITER) "⬇️ bidding" }
                Self::SeeDescription => { "📝 price in description" }
                Self::ToBeAgreed => { "🤝 price to be agreed" }
                Self::Reserved => { "⚠️ reserved" }
                Self::FastBid => { "⬆️ auction" }
                Self::Exchange => { "💱 exchange" }
            }
        }
    }
}

impl Render for Location {
    fn render(&self) -> Markup {
        let mut query = vec![("q", Cow::Borrowed(self.toponym.as_ref()))];
        if let Some(GeoLocation { latitude, longitude }) = self.geo {
            query.push(("ll", Cow::Owned(format!("{latitude},{longitude}"))));
        }
        html! {
            @match Url::parse_with_params("https://maps.apple.com/maps", &query) {
                Ok(url) => { a href=(url) { (self.toponym) } },
                Err(_) => (self.toponym)
            }
        }
    }
}

impl Render for Seller {
    fn render(&self) -> Markup {
        html! { a href=(self.profile_url) { "@" (self.username) } }
    }
}

impl Render for Condition {
    fn render(&self) -> Markup {
        html! {
            @match self {
                Self::New(crate::marketplace::item::New::WithTags) => "🟢 new with tags",
                Self::New(crate::marketplace::item::New::WithoutTags) => "🟢 new without tags",
                Self::New(crate::marketplace::item::New::AsGood) => "🟡 as good as new",
                Self::New(crate::marketplace::item::New::Unspecified) => "🟢 new",
                Self::Used(crate::marketplace::item::Used::VeryGood) => "🟠 very good",
                Self::Used(crate::marketplace::item::Used::Good) => "🟠 good",
                Self::Used(crate::marketplace::item::Used::Satisfactory) => "🟠 satisfactory",
                Self::Used(crate::marketplace::item::Used::Unspecified) => "🟠 used",
                Self::Used(crate::marketplace::item::Used::NotFullyFunctional) => "⛔️ not fully functional",
                Self::Refurbished => "🟡 refurbished",
            }
        }
    }
}

impl Render for Delivery {
    fn render(&self) -> Markup {
        html! {
            @match self {
                Self::CollectionOnly => "🚶 collection",
                Self::ShippingOnly => "📦 shipping",
                Self::Both => { (Self::ShippingOnly) (DELIMITER) (Self::CollectionOnly) }
            }
        }
    }
}

/// Search query as a text together with the management links.
#[derive(Copy, Clone)]
pub struct ManageSearchQuery<'a, C> {
    search_query: &'a str,
    links: &'a [&'a Link<C>],
}

impl<'a, C> ManageSearchQuery<'a, C> {
    pub const fn new(search_query: &'a str, links: &'a [&'a Link<C>]) -> Self {
        Self { search_query, links }
    }
}

impl<C: Render> Render for ManageSearchQuery<'_, C> {
    fn render(&self) -> Markup {
        html! {
            em { (self.search_query) }
            @for links in self.links {
                (DELIMITER) (links)
            }
        }
    }
}
