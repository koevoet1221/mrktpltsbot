use bon::Builder;

#[derive(Builder)]
pub struct Location {
    pub toponym: String,
    pub geo: Option<GeoLocation>,
}

#[derive(Copy, Clone, Builder)]
pub struct GeoLocation {
    pub latitude: f64,
    pub longitude: f64,
}
