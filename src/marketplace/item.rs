use maud::PreEscaped;

/// Marketplace item.
pub struct Item {}

/// Just `<strong> • </strong>`.
const DELIMITER: PreEscaped<&'static str> = PreEscaped(
    // language=html
    "<strong> • </strong>",
);
