use axum::{extract::Json, response::IntoResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    pub timestamp: i64,
}

impl Location {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
            timestamp: chrono::Utc::now().timestamp(),
        }
    }
}
