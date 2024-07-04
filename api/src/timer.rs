use std::sync::Arc;

use axum::{response::IntoResponse, Extension, Json};
use axum_macros::debug_handler;
use tokio::{
    sync::RwLock,
    time::{Duration, Instant},
};

use crate::game::Game;

pub struct Timer {
    start: Instant,
    duration: Duration,
}

impl Timer {
    pub fn new(duration: Duration) -> Self {
        Timer {
            start: Instant::now(),
            duration,
        }
    }

    pub fn elapsed(&self) -> bool {
        self.start.elapsed() >= self.duration
    }

    pub fn remaining(&self) -> Duration {
        self.duration.saturating_sub(self.start.elapsed())
    }
}

#[debug_handler]
pub async fn get(Extension(game): Extension<Arc<RwLock<Game>>>) -> impl IntoResponse {
    let readgame = game.read().await;
    let timer = &readgame.timer;
    if let Some(timer) = timer {
        Json(timer.remaining().as_secs()).into_response();
    } else {
        Json("No timer running").into_response();
    }
}
