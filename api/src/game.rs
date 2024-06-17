use std::collections::HashMap;

use crate::{auth, config::Config};
use edgedb_tokio::{Builder, Client, Queryable};
use serde::Serialize;

#[derive(Debug)]
pub struct Player {
    pub username: String, // kinda like backlink, guaranteed the same as the key for players
    pub token: String,
    pub connected: bool,
}

impl Player {
    pub fn new(username: String, token: String) -> Self {
        Self {
            username,
            token,
            connected: false,
        }
    }

    pub fn update_token(mut self, token: String) {
        tracing::info!("Updating token from {0} to {1}", self.token, token);
        self.token = token;
    }
    pub fn update_connection(mut self, connected: bool) {
        tracing::info!(
            "Updating connection from {0} to {1}",
            self.connected,
            connected
        );
        self.connected = connected;
    }
}

pub enum GameState {
    Setup,     // Only admin is allowed to join.
    Lobby,     // Allow players to join and get ready
    HidePhase, // 15 minutes for hiders to hide
    SeekPhase, // Time for seekers to find the hiders
    RoundEnd,  // End of a round, moves back to lobby.
}

pub struct Game {
    pub state: GameState,
    pub config: Config,
    pub db: Client,
    pub players: HashMap<String, Player>, // username against a player
}

impl Game {
    pub async fn new(db_inst: &str, secret: &str ) -> Self {
        let db = Client::new(
            &Builder::new()
                .secret_key(secret)
                .instance(db_inst).expect("invalid secrets")
                .build_env().await.unwrap()
        );

        db.ensure_connected().await.unwrap();
        
        #[derive(Queryable, Serialize)]
        struct DbPlayer {
            username: String,
        }

        let query = "select Player {username}";
        let res: Vec<DbPlayer> = db.query(query, &()).await.unwrap();
        let players: HashMap<String, Player> = res
            .into_iter()
            .map(|db_player| {
                let player = Player::new(
                    db_player.username.clone(),
                    auth::jwt(&db_player.username),
                );
                (db_player.username, player)
            })
            .collect();

        Game {
            state: GameState::Setup,
            config: Config::init(),
            db,
            players,
        }
    }
}
