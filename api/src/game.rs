use std::collections::HashMap;

use crate::{auth, config::Config, player::Player, socket::Tx, teams::Team};
use axum::extract::ws::Message;
use edgedb_tokio::{Builder, Client, Queryable};
use serde::Serialize;

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

    pub async fn get_mut_player(&mut self, username: &str) -> Result<&mut Player, String> {
        if let Some(player) = self.players.get_mut(username) {
            Ok(player)
        } else {
            Err("Player not found".to_string())
        }
    }

    pub fn get_teams(&self) -> Vec<Team> {
        self.teams.clone()
    }

    pub async fn get_team(&self, name: &str) -> Option<&Team> {
        self.teams.iter().find(|team| team.name == name)
    }

    pub async fn get_mut_team(&mut self, name: &str) -> Option<&mut Team> {
        self.teams.iter_mut().find(|team| team.name == name)
    }

    pub async fn new_team(&mut self, team: Team) -> Result<(), String> {
        if self.teams.contains(&team) {
            return Err("Team already exists".to_string());
        }
        self.teams.push(team);
        Ok(())
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
