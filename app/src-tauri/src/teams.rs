use std::path::PathBuf;
use tauri::Wry;
use tauri::{AppHandle, Manager};
use tauri_plugin_http::reqwest;
use tauri_plugin_store::{with_store, StoreCollection};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct Team {
    pub name: String,
    pub players: Vec<String>,
    pub ttype: String,
    pub ready: bool,
}

#[tauri::command]
pub async fn get(app: AppHandle) -> Result<Vec<Team>, String> {
    let stores = app
        .app_handle()
        .try_state::<StoreCollection<Wry>>()
        .ok_or("store not found")?;
    let path = PathBuf::from("store.bin");

    if let Ok(address) = with_store(app.app_handle().clone(), stores, path, |store| {
        let address: String = store.get("address")
                                   .expect("failed to get address")
                                   .to_string()
                                   .trim_matches('"')
                                   .to_string();
        Ok(address)
    }) {
        let teams_url = format!("{}/teams", address);
        let teams_response = reqwest::get(&teams_url)
            .await
            .map_err(|e| {
                println!("{:?}", e);
                e.to_string()
            })?;

        let teams_map: HashMap<String, Value> = teams_response.json().await.map_err(|e| e.to_string())?;
        let mut teams = Vec::new();

        for (name, team_value) in teams_map {
            let mut team: Team = serde_json::from_value(team_value).map_err(|e| e.to_string())?;
            team.name = name;
            teams.push(team);
        }

        Ok(teams)
    } else {
        Err("Error in connecting to localstore".into())
    }
}

#[tauri::command]
pub async fn join(app: AppHandle, team: String) -> Result<(), String> {
    println!("args {team}");
    let stores = app.app_handle().try_state::<StoreCollection<Wry>>().ok_or("store not found")?;
    let path = PathBuf::from("store.bin");

    // access data from store
    if let Ok((token, address)) = with_store(app.app_handle().clone(), stores, path, |store| {
        let token = store.get("token").expect("failed to get token").to_string().trim_matches('"').to_string();
        let address = store.get("address").expect("failed to get address").to_string().trim_matches('"').to_string();
        Ok((token, address))
    }) {
        let client = reqwest::Client::new();
        let join_url = format!("{address}/teams/{team}/join");
        let header = format!("Bearer {token}");
        println!("addy: {join_url}, Authorization: {header}");
        let join_response = client
            .post(&join_url)
            .header("Authorization", header)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if join_response.status().is_success() {
            return Ok(())
        } else {
            return Err(format!("Failed to join: {}", join_response.status()))
        }
    } else {
        return Err("Error in connecting to localstore".into())
    }
}

#[tauri::command]
pub async fn leave(app: AppHandle, team: String) -> Result<(), String> {
    let stores = app.app_handle().try_state::<StoreCollection<Wry>>().ok_or("store not found")?;
    let path = PathBuf::from("store.bin");

    // Access data from store
    if let Ok((token, address)) = with_store(app.app_handle().clone(), stores, path, |store| {
        let token = store.get("token").expect("failed to get token").to_string().trim_matches('"').to_string();
        let address = store.get("address").expect("failed to get address").to_string().trim_matches('"').to_string();
        Ok((token, address))
    }) {
        let client = reqwest::Client::new();
        let leave_url = format!("{address}/teams/{team}/leave");
        let header = format!("Bearer {token}");
        println!("Leave team URL: {leave_url}, Authorization: {header}");

        // Send POST request to the leave endpoint
        let leave_response = client
            .post(&leave_url)
            .header("Authorization", header)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        // Check if the request was successful
        if leave_response.status().is_success() {
            return Ok(())
        } else {
            return Err(format!("Failed to leave team: {}", leave_response.status()))
        }
    } else {
        return Err("Error in connecting to localstore".into())
    }
}
