use std::{sync::Arc, time::Duration};

use axum::{response::IntoResponse, Extension};
use tokio::{sync::{mpsc, RwLock}, time::interval};

use crate::{game::Game, states::{State, GameState, HideState, LobbyState, RoundEndState, SeekState}};

pub enum Event {
    Lobby,        // switch over to Lobby state,
    Seek,
    Hide,
    RoundEnd,        // switch over to RoundEnd state
}

pub struct GameStateMachine {
    pub state: Arc<RwLock<GameState>>,               // current state
    pub rx: mpsc::Receiver<Event>, // event channel, recieve events
    pub game: Arc<RwLock<Game>>,
}

impl GameStateMachine {
    pub async fn update_state(&self, new_state: GameState) {
        let mut state = self.state.write().await;
        *state = new_state;
    }

    pub async fn get_state(&self) -> GameState {
        self.state.read().await.clone()
    }

    pub async fn run(&mut self) {
        let mut interval = interval(Duration::from_millis(500));
        loop {
            interval.tick().await;
            tokio::select! {
                Some(event) = self.rx.recv() => {
                    println!("got a event");
                    self.handle_event(event).await;
                }
                _ = interval.tick() => {
                    self.process_state().await;
                }
            }
        }
    }

    // init deinit code
    async fn handle_event(&mut self, event: Event) {
        match event {
            Event::Lobby => {
                let mut lobby_state = LobbyState::new();
                lobby_state.init(self.game.clone());
                self.update_state(GameState::Lobby(lobby_state)).await;
            }
            Event::Hide => {
                let mut hide_state = HideState::new();
                hide_state.init(self.game.clone());
                self.update_state(GameState::Hide(hide_state)).await;
            }
            Event::Seek => {
                let mut seek_state = SeekState::new();
                seek_state.init(self.game.clone());
                self.update_state(GameState::Seek(seek_state)).await;
            }
            Event::RoundEnd => {
                let mut round_end_state = RoundEndState::new();
                round_end_state.init(self.game.clone());
                self.update_state(GameState::RoundEnd(round_end_state)).await;
            }
        }
    }

    // loop code
    async fn process_state(&mut self) {
        match self.get_state().await {
            GameState::Lobby(mut state) => state.update(self.game.clone()),
            GameState::Hide(mut state) => state.update(self.game.clone()),
            GameState::Seek(mut state) => state.update(self.game.clone()),
            GameState::RoundEnd(mut state) => state.update(self.game.clone()),
        }
    }

    // TODO! implement pausing and resuming?
}

pub async fn get(Extension(game): Extension<Arc<RwLock<Game>>>) -> impl IntoResponse {
    // router function to get the current gamestate
    game.read().await.state.read().await.clone()
}

pub async fn start(Extension(game): Extension<Arc<RwLock<Game>>>) -> impl IntoResponse {
    // switch from lobby to hide
    if let Err(e) = game.read().await.tx.send(Event::Hide).await {
        return "error"
    }
    "success"
}
