use std::{collections::HashMap, time::Duration};

use crate::{auth, db::Db, player::Player, socket::Tx, teams::Team};
use axum::extract::ws::Message;

pub enum GameState {
    Lobby,     // Allow players to join and get ready
    HidePhase, // 15 minutes for hiders to hide
    SeekPhase, // Time for seekers to find the hiders
    RoundEnd,  // End of a round, moves back to lobby.
}

pub struct Game {
    pub state: GameState,
    pub db: Db,
    pub players: HashMap<String, Player>,
    pub teams: Vec<Team>,
    pub connections: HashMap<String, Tx>, //username agains string
}

impl Game {
    pub async fn new(db_inst: &str, secret: &str) -> Self {
        let db = Db::new(db_inst, secret).await;

        Game {
            state: GameState::Lobby,
            players: db.init().await,
            db,
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

    pub fn all_players_ready(&self) -> bool {
        self.players.values().all(|player| player.ready)
    }

    pub async fn broadcast(&self, msg: Message) -> Result<(), String> {
        for (_, player) in self.players.iter() {
            if player.connected {
                player.send_msg(msg.clone()).await?;
            }
        }
        Ok(())
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
}
