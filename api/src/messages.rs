use serde::Deserialize;

#[derive(Deserialize)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}
