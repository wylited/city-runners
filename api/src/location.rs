use axum::{extract::Json, response::IntoResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    latitude: f64,
    longitude: f64,
}

pub async fn recieve(Json(location): Json<Location>) -> impl IntoResponse {
    tracing::info!("Received location: {:?}", location);

    "Location received successfully!".into_response()
}
