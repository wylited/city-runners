use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::game::Game;

#[derive(Debug, Deserialize, Serialize)]
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
    game.write().await.get_mut_player(who).await.unwrap().set_location(Location::new(latitude, longitude));
}

#[derive(Serialize, Deserialize)]
struct Station {
    name: String,
    code: String,
    district: String,
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

#[derive(Serialize, Deserialize)]
struct MTRGraph {
    stations: Vec<Station>,
    edges: Vec<Edge>,
}
