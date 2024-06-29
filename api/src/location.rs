use serde::{Deserialize, Serialize};
ccccccc
#[derive(Debug, Deserialize, Serialize)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    pub timestamp: Option<i64>,
}

impl Location {
    pub fn plain(latitude: f64, longitude: f64) -> Self {
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
