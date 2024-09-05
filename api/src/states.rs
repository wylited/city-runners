use std::sync::Arc;
use tokio::sync::RwLock;

use axum::response::{IntoResponse, Response};

use crate::game::Game;

#[derive(Clone)]
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
    fn init(&mut self, game: Arc<RwLock<Game>>);
    fn update(&mut self, game: Arc<RwLock<Game>>);
    fn new() -> Self;
}

#[derive(Clone)]
pub struct LobbyState;
impl State for LobbyState {
    fn init(&mut self, game: Arc<RwLock<Game>>) {
        println!("init Lobby state");
    }

    fn update(&mut self, game: Arc<RwLock<Game>>) {
        println!("Lobby state");
    }

    fn new() -> Self {
        LobbyState
    }
}

#[derive(Clone)]
pub struct HideState;
impl State for HideState {
    fn init(&mut self, game: Arc<RwLock<Game>>) {
        println!("init Hide state");
    }
    fn update(&mut self, game: Arc<RwLock<Game>>) {
        println!("Hide state");
    }
    fn new() -> Self {
        HideState
    }
}

#[derive(Clone)]
pub struct SeekState;
impl State for SeekState {
    fn init(&mut self, game: Arc<RwLock<Game>>) {
        println!("Seek state");
    }
    fn update(&mut self, game: Arc<RwLock<Game>>) {
        println!("Seek state");
    }
    fn new() -> Self {
        SeekState
    }
}

#[derive(Clone)]
pub struct RoundEndState;
impl State for RoundEndState {
    fn init(&mut self, game: Arc<RwLock<Game>>) {
        println!("RoundEnd state");
    }
    fn update(&mut self, game: Arc<RwLock<Game>>) {
        println!("RoundEnd state");
    }
    fn new() -> Self {
        RoundEndState
    }
}
