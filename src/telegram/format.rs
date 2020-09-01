use std::borrow::Cow;

use crate::marktplaats::{PriceInfo, PriceType, SearchListing};
use crate::math::div_rem;
use crate::prelude::*;

lazy_static! {
    /// Letters that must be escaped.
    static ref ESCAPE_MARKDOWN_V2_REGEX: regex::Regex =
        regex::Regex::new(r"[_\*\[\]\(\)\~`>\#\+\-=\|\{\}\.!]").unwrap();
}

pub fn format_listing_text(listing: &SearchListing) -> String {
    format!(
        "*{}*\n\n*{}*\n\n{}",
        escape_markdown_v2(&listing.title),
        format_price(&listing.price),
        escape_markdown_v2(&listing.description),
    )
}

/// Escape the text for Telegram `MarkdownV2` markup.
pub fn escape_markdown_v2(text: &str) -> Cow<str> {
    ESCAPE_MARKDOWN_V2_REGEX.replace_all(text, r"\$0")
}

fn format_price(price: &PriceInfo) -> String {
    let (euros, cents) = div_rem(price.cents, 100);
    match price.type_ {
        PriceType::Exchange => "💱 Exchange".into(),
        PriceType::FastBid => "🤔 Bid".into(),
        PriceType::Fixed => format!("💰 {}\\.{:02}", euros, cents),
        PriceType::Free => "🆓 Free".into(),
        PriceType::MinBid => format!("💰⬇️ {}\\.{:02}", euros, cents),
        PriceType::OnRequest => "❓ On Request".into(),
        PriceType::Reserved => "😕 Reserved".into(),
        PriceType::SeeDescription => "📝 See Description".into(),
        PriceType::ToBeAgreed => "🤝 To Be Agreed".into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_markdown_v2_ok() {
        assert_eq!(escape_markdown_v2("Hello, world!"), r"Hello, world\!");
        assert_eq!(escape_markdown_v2("hello, world"), r"hello, world");
        assert_eq!(
            escape_markdown_v2("Philips Hue GU10 White and Color Ambiance Splinternieuw!"),
            r"Philips Hue GU10 White and Color Ambiance Splinternieuw\!",
        );
    }
}
