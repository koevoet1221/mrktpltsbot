#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Delivery {
    CollectionOnly,
    ShippingOnly,
    Both,
}
