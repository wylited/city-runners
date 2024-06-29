use std::{collections::HashMap, sync::Arc};

use crate::{auth, config::Config, mtr::Location, socket::Tx};
use axum::extract::ws::{Message, WebSocket};
use edgedb_tokio::{Builder, Client, Queryable};
use futures::{stream::SplitSink, SinkExt};
use serde::Serialize;
use tokio::sync::RwLock;


#[derive(Debug)]
pub enum PlayerType {
    Hider,
    SecondarySeeker,
    PrimarySeeker,
    Admin,
    Spectator,
}

#[derive(Debug)]
pub struct Player {
    pub username: String, // kinda like backlink, guaranteed the same as the key for players
    pub token: String,
    pub connected: bool,
    pub ptype: PlayerType,
    pub stream: Option<Tx>,
    pub current_location: Option<Location>
}

impl Player {
    pub fn new(username: String, token: String) -> Self {
        Self {
            username,
            token,
            ptype: PlayerType::Spectator,
            connected: false,
            stream: None,
            current_location: None,
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
        if !connected {
            self.stream = None;
        }
    }

    pub fn set_location(&mut self, location: Location) {
        self.current_location = Some(location);
    }

    pub fn set_stream(&mut self, stream: SplitSink<WebSocket, Message>) {
        self.stream = Some(Arc::new(RwLock::new(stream)));
    }

    pub async fn send_msg(&self, msg: Message) -> Result<(), String> {
        if let Some(ref arctx) = self.stream {
            let mut tx = arctx.write().await;
            let res = tx.send(msg).await;
            if let Err(e) = res {
                return Err(e.to_string());
            }
            Ok(())
        } else {
            return Err("No connection found".to_string());
        }
    }
    pub async fn set_location(&mut self, location: Location) {
        self.current_location = Some(location);
    }
}

pub enum TeamType {
    Seeker,
    Hider,
    Spectator,
}

pub struct Team {
    pub name: String,
    pub players: Vec<String>,
    pub ttype: TeamType,
}

pub enum GameState {
    Lobby,     // Allow players to join and get ready
    HidePhase, // 15 minutes for hiders to hide
    SeekPhase, // Time for seekers to find the hiders
    RoundEnd,  // End of a round, moves back to lobby.
}

pub struct Game {
    pub state: GameState,
    pub config: Config,
    pub db: Client,
    pub players: HashMap<String, Player>,
    pub teams: Vec<Team>,
    pub connections: HashMap<String, Tx>, //username agains string
}

impl Game {
    pub async fn new(db_inst: &str, secret: &str) -> Self {
        let db = Client::new(
            &Builder::new()
                .secret_key(secret)
                .instance(db_inst)
                .expect("invalid secrets")
                .build_env()
                .await
                .unwrap(),
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
                let player =
                    Player::new(db_player.username.clone(), auth::jwt(&db_player.username));
                (db_player.username, player)
            })
            .collect();

        Game {
            state: GameState::Lobby,
            config: Config::init(),
            db,
            players,
            teams: Vec::new(),
            connections: HashMap::new(),
        }
    }

    pub async fn new_player(&mut self, username: String) -> Result<(), String> {
        if self.players.contains_key(&username) {
            return Err("Player already exists".to_string());
        }
        let player = Player::new(username.clone(), auth::jwt(&username));
        self.players.insert(username, player);
        Ok(())
    }

    pub async fn get_player(&self, username: &str) -> Result<&Player, String> {
        if let Some(player) = self.players.get(username) {
            Ok(player)
        } else {
            Err("Player not found".to_string())
        }
    }

    pub async fn broadcast(&self, msg: Message) -> Result<(), String> {
        for (_, player) in self.players.iter() {
            if player.connected {
                player.send_msg(msg.clone()).await?;
            }
        }
        Ok(())
    }
}
