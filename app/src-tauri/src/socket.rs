use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use std::path::PathBuf;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tauri::{AppHandle, Wry, Manager};
use tauri_plugin_store::{with_store, StoreCollection};
use tauri::Emitter;
use tauri_plugin_http::reqwest;
use tokio::task;
use tauri_plugin_notification::NotificationExt;
use serde_json::Value;

#[tauri::command]
pub async fn connect(app: AppHandle) -> Result<(), String> {
    let stores = match app.app_handle().try_state::<StoreCollection<Wry>>() {
        Some(stores) => stores,
        None => {
            eprintln!("Store not found");
            return Err("Store not found".to_string());
        }
    };

    let path = PathBuf::from("store.bin");

    let (address, token) = match with_store(app.app_handle().clone(), stores, path, |store| {
        let address = store.get("address").expect("failed to get address").to_string().trim_matches('"').to_string();
        let token = store.get("token").expect("failed to get address").to_string().trim_matches('"').to_string();
        Ok((address, token))
    }) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("{}", e);
            return Err(e.to_string());
        }
    };

    let ws_url = address.replace("https://", "wss://") + "/ws?token=" + &token.trim_matches('"').to_string();
    task::spawn(async move { // need to spawn the socket later, and then i need an interface for the socket?
        socket(ws_url, app).await;
    });

    Ok(())
}

pub async fn socket(address: String, app: AppHandle) {
    let ws_url = address;
    println!("{ws_url}");

    // Connect with the request
    let (ws_stream, _) = match connect_async(ws_url).await {
        Ok(stream) => stream,
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
            return;
        }
    };

    println!("WebSocket connected");

    let (mut write, mut read) = ws_stream.split();

    // Channel for sending messages to the write loop
    let (tx, mut rx) = mpsc::channel::<Message>(32);

    tokio::spawn(async move {
        while let Some(Ok(msg)) = read.next().await {
            match msg {
                Message::Text(text) => handle_json_message(text, app.clone()).await,
                _ => println!("Received weird msg {:?}", msg),
            }
        }
    });

    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(1));
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    let location = json!({
                        "op": "location",
                        "latitude": 37.7749,
                        "longitude": -122.4194
                    });
                    let message = Message::Text(location.to_string());
                    if let Err(e) = tx.send(message).await {
                        eprintln!("Failed to send message: {}", e);
                        break;
                    }
                }
                Some(msg) = rx.recv() => {
                    if let Err(e) = write.send(msg).await {
                        eprintln!("Failed to send message: {}", e);
                        break;
                    } else {
                        println!("Sent location");
                    }
                }
            }
        }
    });
}

async fn handle_json_message(text: String, app: AppHandle) {
    match serde_json::from_str::<Value>(&text) {
        Ok(json) => {
            // Check if the JSON is an object and contains the "op" key
            if let Some(op) = json.get("op").and_then(Value::as_str) {
                match op {
                    "notif" => handle_notif(json, app).await,
                    "state" => handle_state(json, app).await.expect("STATE ERROR"),
                    "chat" => handle_chat(json, app).await.expect("CHAT ERROR"),
                    "location" => handle_location(json).await,
                    _ => println!("Unknown operation: {}", op),
                }
            } else {
                println!("'op' key not found or invalid");
            }
        }
        Err(_) => {
            println!("Invalid JSON");
        }
    }
}

async fn handle_notif(json: Value, app: AppHandle) {
    if let (Some(who), Some(msg)) = (json.get("who").and_then(Value::as_str), json.get("msg").and_then(Value::as_str)) {
        app.notification()
            .builder()
            .title(who)
            .body(msg)
            .show()
            .unwrap();
    } else {
        println!("Missing 'who' or 'msg' key in notification");
    }
}

async fn handle_chat(json: Value, app: AppHandle) -> Result<(), String> {
    if let Some(msg) = json.get("msg") {
        let stores = app.app_handle().try_state::<StoreCollection<Wry>>().ok_or("store not found")?;
        let path = PathBuf::from("store.bin");

        with_store(app.clone(), stores, path, |store| {
            // Get current messages or create empty vector if none exist
            let mut msgs: Vec<String> = store
                .get("msgs")
                .and_then(|v| v.as_array().cloned())
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap();

            // Append new message
            if let Some(new_msg) = msg.as_str() {
                msgs.push(new_msg.to_string());
            }

            // Save updated messages back to store
            store.insert("msgs".to_string(), json!(msgs))
        });
    }
    Ok(())
}
async fn handle_state(json: Value, app: AppHandle) -> Result<(), String> {
    if let Some(state) = json.get("state") {
        let stores = app.app_handle().try_state::<StoreCollection<Wry>>().ok_or("store not found")?;
        let path = PathBuf::from("store.bin");
        let mut page_update: Option<String> = None;

        with_store(app.clone(), stores, path, |store| {
           store.insert("state".to_string(), state.clone())
        });

        return match state.as_str() {
            Some("Lobby") => {
                        app.notification()
            .builder()
            .title("Round Started")
            .body("Ready up your team!")
            .show()
            .unwrap();
                Ok(())
            },
            Some("Hide") => {
                                        app.notification()
            .builder()
            .title("Game Started")
            .body("Go HIDE!")
            .show()
            .unwrap();

                Ok(())
            },
            Some("Seek") => {
                                                        app.notification()
            .builder()
            .title("Hiding time over")
            .body("Seekers go seek!")
            .show()
            .unwrap();

                Ok(())
            },
            Some("RoundEnd") => {
            Ok(())
            },
            Some(unknown_state) => {
                println!("unknown state {}", unknown_state);
                Err(format!("unknown state {}", unknown_state))
            },
            None => {
                println!("state value is not a string");
                Err("State value is not a string".to_string())
            }
        };

        Ok(())
    } else {
        println!("Missing 'state' key in update");
        return Err("Missing 'state' key in update".to_string());
    }
}

async fn handle_location(json: Value) {

}
