use std::sync::Arc;

use axum::{extract::Path, response::IntoResponse, Extension, Json};
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::RwLock;
use tracing::info;

use crate::game::Game;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TeamType {
    Seeker,
    Hider,
    Spectator,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Team {
    pub name: String,
    pub players: Vec<String>,
    pub ttype: TeamType,
}

impl PartialEq for Team {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Team {
    pub fn new(name: String) -> Team {
        Team {
            name,
            players: Vec::new(),
            ttype: TeamType::Spectator,
        }
    }

    pub fn add_player(&mut self, player: String) {
        self.players.push(player);
    }

    pub fn remove_player(&mut self, player: String) {
        self.players.retain(|p| p != &player);
    }

    pub fn is_player_on_team(&self, player: &str) -> bool {
        self.players.contains(&player.to_string())
    }

    pub fn update_type(&mut self, ttype: TeamType) {
        self.ttype = ttype;
    }
}

pub async fn getall(
    Extension(game): Extension<Arc<RwLock<Game>>>,
    Extension(username): Extension<String>,
) -> impl IntoResponse {
    let game = game.read().await;
    let teams = game.get_teams();
    info!("user {} requested teams", username);
    Json(teams).into_response()
}

#[debug_handler]
pub async fn get(
    Extension(game): Extension<Arc<RwLock<Game>>>,
    Extension(username): Extension<String>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    let game = game.read().await;
    let team: Option<&Team> = game.get_team(&name).await;
    info!("user {} requested team: {}", username, name);
    if let Some(team) = team {
        Json(team).into_response()
    } else {
        Json(json!({"error": "Team not found"})).into_response()
    }
}

pub async fn create(
    Extension(game): Extension<Arc<RwLock<Game>>>,
    Extension(username): Extension<String>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    let mut team = Team::new(name.clone());
    team.add_player(username.clone());
    if let Err(e) = game.write().await.new_team(team).await {
        return Json(json!({"error": e})).into_response();
    }
    info!("user {} created team: {:?}", username, name);
    Json(json!({"message": "Team created"})).into_response()
}

pub async fn join(
    Extension(game): Extension<Arc<RwLock<Game>>>,
    Path(team_name): Path<String>,
    Extension(username): Extension<String>,
) -> impl IntoResponse {
    let mut game = game.write().await;

    {
        let player = game.get_mut_player(&username).await;
        if player.is_err() {
            return Json(json!({"error": "Player not found"})).into_response();
        }
        let player = player.unwrap();
        if player.team.is_some() {
            return Json(json!({"error": "Player already on a team"})).into_response();
        }
        player.team = Some(team_name.clone());
    }

    let team = game.get_mut_team(&team_name).await;

    if let Some(team) = team {
        team.add_player(username.clone());
        info!("user {} added to team: {}", username, team_name);
        return Json(json!({"message": "Player added to team"})).into_response();
    }
    Json(json!({"error": "Team not found"})).into_response()
}

pub async fn leave(
    Extension(game): Extension<Arc<RwLock<Game>>>,
    Path(team_name): Path<String>,
    Extension(username): Extension<String>,
) -> impl IntoResponse {
    let mut game = game.write().await;
    {
        let player = game.get_mut_player(&username).await;
        if player.is_err() {
            return Json(json!({"error": "Player not found"})).into_response();
        }
        player.unwrap().team = None;
    }
    let team = game.get_mut_team(&team_name).await;
    if team.is_none() {
        return Json(json!({"error": "Team not found"})).into_response();
    }
    let team = team.unwrap();
    if team.is_player_on_team(&username) {
        team.remove_player(username.clone());
        info!("user {} removed from team: {}", username, team_name);
        Json(json!({"message": "Player removed from team"})).into_response()
    } else {
        Json(json!({"error": "You are not a member of this team"})).into_response()
    }
}

pub async fn update_team_name(
    Extension(game): Extension<Arc<RwLock<Game>>>,
    Path(team_name): Path<String>,
    Extension(username): Extension<String>,
    Json(new_team_name): Json<String>,
) -> impl IntoResponse {
    let mut game = game.write().await;
    let team = game.get_mut_team(&team_name).await;
    if team.is_none() {
        return Json(json!({"error": "Team not found"})).into_response();
    }
    let team = team.unwrap();
    if team.is_player_on_team(&username) {
        team.name = new_team_name;
        info!("user {} updated team name: {}", username, team_name);
        Json(json!({"message": "Team name updated"})).into_response()
    } else {
        Json(json!({"error": "You are not a member of this team"})).into_response()
    }
}
