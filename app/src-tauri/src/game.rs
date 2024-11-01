use std::path::PathBuf;
use tauri::Wry;
use tauri::{AppHandle, Manager};
use tauri_plugin_http::reqwest;
use tauri_plugin_store::{with_store, StoreCollection};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[tauri::command]
pub async fn start(app: AppHandle) -> Result<(), String> {
    let stores = app.app_handle().try_state::<StoreCollection<Wry>>().ok_or("store not found")?;
    let path = PathBuf::from("store.bin");

    // Access data from store
    if let Ok((token, address)) = with_store(app.app_handle().clone(), stores, path, |store| {
        let token = store.get("token").expect("failed to get token").to_string().trim_matches('"').to_string();
        let address = store.get("address").expect("failed to get address").to_string().trim_matches('"').to_string();
        Ok((token, address))
    }) {
        let client = reqwest::Client::new();
        let start_url = format!("{address}/start");
        let header = format!("Bearer {token}");
        println!("start URL: {start_url}, Authorization: {header}");

        // Send POST request to the start endpoint
        let start_response = client
            .post(&start_url)
            .header("Authorization", header)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        // Check if the request was successful
        if start_response.status().is_success() {
            return Ok(())
        } else {
            return Err(format!("Failed to start game: {}", start_response.status()))
        }
    } else {
        return Err("Error in connecting to localstore".into())
    }
}
