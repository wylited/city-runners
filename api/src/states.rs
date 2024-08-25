use std::time::Duration;

use axum::response::{IntoResponse, Response};
use tokio::time::{interval, Interval};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GameState {
    Lobby(LobbyState),
    Hide(HideState),
    Seek(SeekState),
    RoundEnd(RoundEndState),
}

impl IntoResponse for GameState {
    fn into_response(self) -> Response {
        match self {
            GameState::Lobby(_) => "Lobby".into_response(),
            GameState::Hide(_) => "Hide".into_response(),
            GameState::Seek(_) => "Seek".into_response(),
            GameState::RoundEnd(_) => "RoundEnd".into_response(),
        }
    }
}

pub trait State {
    fn init();
    fn update();
    fn new() -> Self;
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct LobbyState;
impl State for LobbyState {
    fn init() {
        println!("Lobby state");
    }
    fn update() {
        println!("Lobby state");
    }
    fn new() -> Self {
        LobbyState
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct HideState;
impl State for HideState {
    fn init() {
        println!("Hide state");
    }
    fn update() {
        println!("Hide state");
    }
    fn new() -> Self {
        HideState
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SeekState;
impl State for SeekState {
    fn init() {
        println!("Seek state");
    }
    fn update() {
        println!("Seek state");
    }
    fn new() -> Self {
        SeekState
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct RoundEndState;
impl State for RoundEndState {
    fn init() {
        println!("RoundEnd state");
    }
    fn update() {
        println!("RoundEnd state");
    }
    fn new() -> Self {
        RoundEndState
    }
}
