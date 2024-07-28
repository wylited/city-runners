use std::sync::Arc;
use tokio::sync::RwLock;

use crate::game::Game;

#[derive(Clone, Debug)]
pub enum GameState {
    Lobby(Lobby),
    HidePhase(HidePhase),
    SeekPhase(SeekPhase),
    RoundEnd(RoundEnd),
}

pub trait State {
    async fn on_enter(&self, game: Arc<RwLock<Game>>);
    async fn tick(&mut self, game: Arc<RwLock<Game>>) -> GameState;
}

#[derive(Clone, Debug)]
pub struct Lobby {
    // Custom data for Lobby
}

#[derive(Clone, Debug)]
pub struct HidePhase {
    pub hiders: String,
}

#[derive(Clone, Debug)]
pub struct SeekPhase {
    // Custom data for SeekPhase
}

#[derive(Clone, Debug)]
pub struct RoundEnd {
    // Custom data for RoundEnd
}

impl State for Lobby {
    async fn on_enter(&self, game: Arc<RwLock<Game>>) {
        tracing::info!("Entered Lobby state");
    }

    async fn tick(&mut self, game: Arc<RwLock<Game>>) -> GameState {
        GameState::Lobby(self.clone())
    }
}

impl State for HidePhase {
    async fn on_enter(&self, game: Arc<RwLock<Game>>) {
        tracing::info!("Entered HidePhase state");
    }

    async fn tick(&mut self, game: Arc<RwLock<Game>>) -> GameState {
        println!("HidePhase state tick");
        // Periodic logic for hide phase state

        // Return the same state or a different state based on conditions
        GameState::HidePhase(self.clone())
    }
}

impl State for SeekPhase {
    async fn on_enter(&self, game: Arc<RwLock<Game>>) {
        println!("Entered SeekPhase state");
        // Initialize seek phase state
    }

    async fn tick(&mut self, game: Arc<RwLock<Game>>) -> GameState {
        println!("SeekPhase state tick");
        // Periodic logic for seek phase state

        // Return the same state or a different state based on conditions
        GameState::SeekPhase(self.clone())
    }
}

impl State for RoundEnd {
    async fn on_enter(&self, game: Arc<RwLock<Game>>) {
        println!("Entered RoundEnd state");
        // Initialize round end state
    }

    async fn tick(&mut self, game: Arc<RwLock<Game>>) -> GameState {
        println!("RoundEnd state tick");
        // Periodic logic for round end state

        // Return the same state or a different state based on conditions
        GameState::RoundEnd(self.clone())
    }
}
