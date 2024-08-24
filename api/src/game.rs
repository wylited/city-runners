use std::{collections::HashMap, sync::Arc, time::Duration};

use crate::{auth, db::Db, graph::Graph, player::Player, socket::Tx, states::{GameState, Lobby, State}, teams::Team};
use axum::extract::ws::Message;
use tokio::{sync::RwLock, time};

pub struct Game {
    pub state: GameState,
    pub db: Db,
    pub players: HashMap<String, Player>,
    pub teams: HashMap<String, Team>,
    pub hidden_teams: HashMap<String, Team>, // teams that have already hidden once
    pub connections: HashMap<String, Tx>, //username agains string
    pub graph: Graph,
}

impl Game {
    pub async fn new(db_inst: &str, secret: &str) -> Arc<RwLock<Self>> {
        let db = Db::new(db_inst, secret).await;

        let game = Arc::new(RwLock::new(Game {
            state: GameState::Lobby(Lobby{}),
            players: db.init().await,
            db,
            teams: HashMap::new(),
            hidden_teams: HashMap::new(),
            connections: HashMap::new(),
            graph: Graph::from_csv(),
        }));

        let runner = game.clone();
        Game::run_state(runner).await;
        game
    }

    // State Machine management
    pub async fn run_state(game: Arc<RwLock<Game>>) {
        loop { // game loop lives here
            Game::run_state_loop(game.clone(), game.read().await.state.clone()).await;
        }
    }

    async fn run_state_loop(game: Arc<RwLock<Game>>, mut state: GameState) {
        match state {
            GameState::Lobby(ref mut l) => l.on_enter(game.clone()).await,
            GameState::HidePhase(ref mut h) => h.on_enter(game.clone()).await,
            GameState::SeekPhase(ref mut s) => s.on_enter(game.clone()).await,
            GameState::RoundEnd(ref mut r) => r.on_enter(game.clone()).await,
        }

        let mut interval = time::interval(Duration::from_secs(1));

        loop {
            interval.tick().await;

            let new_state = {
                match state {
                    GameState::Lobby(ref mut l) => l.tick(game.clone()).await,
                    GameState::HidePhase(ref mut h) => h.tick(game.clone()).await,
                    GameState::SeekPhase(ref mut t) => t.tick(game.clone()).await,
                    GameState::RoundEnd(ref mut r) => r.tick(game.clone()).await,
                }
            };

            // Check if the state has changed
            let state_changed = {
                let current_state = {
                    let game_read = game.read().await;
                    game_read.state.clone()
                };
                !matches!((current_state, new_state.clone()),
                    (GameState::Lobby(_), GameState::Lobby(_)) |
                    (GameState::HidePhase(_), GameState::HidePhase(_)) |
                    (GameState::SeekPhase(_), GameState::SeekPhase(_)) |
                    (GameState::RoundEnd(_), GameState::RoundEnd(_)))
            };

            if state_changed {
                // Acquire a write lock to update the state
                {
                    let mut game_write = game.write().await;
                    game_write.state = new_state;
                }
                break; // reset the loop
            }
        }
    }

    // Player methods
    // pub async fn new_player(&mut self, username: String) -> Result<(), String> {
    //     if self.players.contains_key(&username) {
    //         return Err("Player already exists".to_string());
    //     }
    //     let player = Player::new(username.clone(), auth::jwt(&username));
    //     self.players.insert(username, player);
    //     Ok(())
    // }

    pub async fn get_player(&self, username: &str) -> Result<&Player, String> {
        if let Some(player) = self.players.get(username) {
            Ok(player)
        } else {
            Err("Player not found".to_string())
        }
    }

    pub async fn get_mut_player(&mut self, username: &str) -> Result<&mut Player, String> {
        if let Some(player) = self.players.get_mut(username) {
            Ok(player)
        } else {
            Err("Player not found".to_string())
        }
    }

    pub async fn remove_player(&mut self, username: &str) -> Result<(), String> {
        if self.players.contains_key(username) {
            self.players.remove(username);
        } else {
            return Err("Player not found".to_string())
        }

        // remove them from any teams they may have been in
        for team in self.teams.iter_mut() {
            team.1.remove_player(username.to_string());
        }

        Ok(())
    }

    // Broadcast to all players
    pub async fn broadcast(&self, msg: Message) -> Result<(), String> {
        for (_, player) in self.players.iter() {
            if player.connected {
                player.send_msg(msg.clone()).await?;
            }
        }
        Ok(())
    }

    // Team methods
    pub fn get_teams(&self) -> HashMap<String, Team> {
        self.teams.clone()
    }

    pub async fn get_team(&self, name: &str) -> Option<&Team> {
        self.teams.get(name)
    }

    pub async fn get_mut_team(&mut self, name: &str) -> Option<&mut Team> {
        self.teams.get_mut(name)
    }

    pub async fn new_team(&mut self, team: Team) -> Result<(), String> {
        if self.teams.contains_key(&team.name) {
            return Err("Team already exists".to_string());
        }
        self.teams.insert(team.name.clone(), team);
        Ok(())
    }
}
