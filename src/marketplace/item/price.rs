use crate::marketplace::item::amount::Amount;

#[derive(Copy, Clone)]
pub enum Price {
    Fixed(Amount),
    OnRequest,
    MinimalBid(Amount),
    MaximalBid(Amount),
    SeeDescription,
    ToBeAgreed,
    Reserved,
    FastBid,
    Exchange,
}
