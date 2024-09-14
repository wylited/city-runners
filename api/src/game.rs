use std::{collections::HashMap, sync::Arc};

use crate::{db::Db, graph::Graph, player::Player, socket::Tx, state_machine::{Event}, states::{State, GameState}, teams::Team};
use axum::{extract::ws::Message, response::IntoResponse, Extension};
use tokio::sync::{mpsc, RwLock};

pub struct Game {
    pub db: Db,
    pub players: HashMap<String, Player>,
    pub teams: HashMap<String, Team>,
    pub hidden_teams: HashMap<String, Team>, // teams that have already hidden once
    pub connections: HashMap<String, Tx>, //username agains string
    pub graph: Graph,
    pub tx: mpsc::Sender<Event>, // send event to the gamestate
    pub state: Arc<RwLock<GameState>>
}

impl Game {
    pub async fn new(db_inst: &str, secret: &str, state: Arc<RwLock<GameState>>) -> (Arc<RwLock<Self>>, mpsc::Receiver<Event>) {
        let db = Db::new(db_inst, secret).await;
        let (tx, rx) = mpsc::channel(2);

        let game = Game {
            players: db.init().await,
            db,
            teams: HashMap::new(),
            hidden_teams: HashMap::new(),
            connections: HashMap::new(),
            graph: Graph::from_csv(),
            tx,
            state,
        };

        (Arc::new(RwLock::new(game)), rx)
    }

    pub fn get_player(&self, username: &str) -> Result<&Player, String> {
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
