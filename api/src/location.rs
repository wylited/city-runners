use std::{collections::HashMap, sync::Arc};

use axum::{extract::Query, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::RwLock;

use crate::game::Game;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    pub timestamp: Option<i64>,
}

impl Location {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
            timestamp: None,
        }
    }

    pub fn now(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
            timestamp: Some(chrono::Utc::now().timestamp()),
        }
    }
}

pub async fn handle_location_op(json: &serde_json::Value, who: &str, game: &Arc<RwLock<Game>>) {
    let latitude = json.get("latitude").unwrap().as_f64().unwrap();
    let longitude = json.get("longitude").unwrap().as_f64().unwrap();

    game.write()
        .await
        .get_mut_player(who)
        .await
        .unwrap()
        .set_location(Location::new(latitude, longitude));
}

#[derive(Serialize, Deserialize)]
struct Station {
    name: String,
    code: String,
    location: Location,
}

#[derive(Serialize, Deserialize)]
enum Line {
    EastRail,
    KwunTong,
    TsuenWan,
    Island,
    TungChung,
    AirportExpress,
    TseungKwanO,
    TuenMa,
    DisneylandResort,
    SouthIsland,
}

#[derive(Serialize, Deserialize)]
struct Edge {
    from: String,
    to: String,
    line: Line,
    travel_time: u32, // in minutes
}

struct MTR {
    stations: HashMap<String, Station>,
    graph: HashMap<String, Vec<(Station, u32)>>,
}

// http post with query parameters lat and long return json x: i32, y: i32
pub async fn convert(Query(location): Query<Location>) -> impl IntoResponse {
    let (x, y) = converter(location.latitude, location.longitude);
    Json(json!({ "x": x, "y": y }))
}

pub fn converter(latitude: f64, longitude: f64) -> (i32, i32) {
    // Lai Chi Kok lat: 22.3373 long: 114.1482 y: 1359 x: 1465
    // dy/dlat = -6345.44055
    // dx/dlong = 5767.020133
    let dlat = 22.3373 - latitude;
    let dlong = 114.1482 - longitude;
    let dy = dlat * 6345.44055;
    let dx = dlong * 5767.02013;
    let y = 1359 + dy as i32;
    let x = 1465 + dx as i32;
    (x, y)
}
