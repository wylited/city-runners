use axum::extract::ws::Message;
use serde_json::json;
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

#[async_trait::async_trait]
pub trait State {
    async fn init(&mut self, game: Arc<RwLock<Game>>);
    async fn update(&mut self, game: Arc<RwLock<Game>>);
    fn new() -> Self;
}

#[derive(Clone)]
pub struct LobbyState;
#[async_trait::async_trait]
impl State for LobbyState {
    async fn init(&mut self, game: Arc<RwLock<Game>>) {
        let state = serde_json::to_string(&json!({
            "op": "state",
            "state": "Lobby",
        }))
        .unwrap();

        if let Err(e) = game
            .write()
            .await
            .broadcast(Message::Text(state))
            .await {
            tracing::error!("Failed to update the state. {}", e);
        }
        println!("Init Lobby state");
        // Initialize the hider seeker lists
    }

    async fn update(&mut self, game: Arc<RwLock<Game>>) {
        println!("Lobby state");
        // account for new teams?
    }

    fn new() -> Self {
        LobbyState
    }
}

#[derive(Clone)]
pub struct HideState;
#[async_trait::async_trait]
impl State for HideState {
    async fn init(&mut self, game: Arc<RwLock<Game>>) {
        let state = serde_json::to_string(&json!({
            "op": "state",
            "state": "Hide",
        }))
        .unwrap();

        if let Err(e) = game
            .write()
            .await
            .broadcast(Message::Text(state))
            .await {
            tracing::error!("Failed to update the state. {}", e);
        }
        println!("Init hide state");

        // pick hiders
        // Request for hiders base
        // START TIMER TO CHANGE
    }

    async fn update(&mut self, game: Arc<RwLock<Game>>) {
        println!("Hide state");
        // CHECK TIMER FOR CHANGEOVER
    }

    fn new() -> Self {
        HideState
    }
}

#[derive(Clone)]
pub struct SeekState;
#[async_trait::async_trait]
impl State for SeekState {
    async fn init(&mut self, game: Arc<RwLock<Game>>) {
        let state = serde_json::to_string(&json!({
            "op": "state",
            "state": "Seek",
        }))
        .unwrap();

        if let Err(e) = game
            .write()
            .await
            .broadcast(Message::Text(state))
            .await {
            tracing::error!("Failed to update the state. {}", e);
        }
        println!("Init Seek state");

        // START TIMER
    }

    async fn update(&mut self, game: Arc<RwLock<Game>>) {
        println!("Seek state");
        // CHECK END OF TIMER
        // CHECK IF SEEKER HAS FOUND HIDER
        //
    }

    fn new() -> Self {
        SeekState
    }
}

#[derive(Clone)]
pub struct RoundEndState;
#[async_trait::async_trait]
impl State for RoundEndState {
    async fn init(&mut self, game: Arc<RwLock<Game>>) {
        let state = serde_json::to_string(&json!({
            "op": "state",
            "state": "RoundEnd",
        }))
        .unwrap();

        if let Err(e) = game
            .write()
            .await
            .broadcast(Message::Text(state))
            .await {
            tracing::error!("Failed to update the state. {}", e);
        }
        println!("Init RoundEnd state");
    }

    async fn update(&mut self, game: Arc<RwLock<Game>>) {
        println!("RoundEnd state");
    }

    fn new() -> Self {
        RoundEndState
    }
}
