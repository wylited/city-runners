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
                    "state" => handle_state(json, app).await,
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

async fn handle_state(json: Value, app: AppHandle) {
    if let Some(state) = json.get("state") {
        let store = match app.get_store("store.bin") {
            Some(s) => s,
            None => {
                println!("Store missing");
                return;
            }
        };

        store.set("state", state)?; // Need to handle the Result

        match state.as_str() {
            Some("Lobby") => {},
            Some("Hide") => {},
            Some("Seek") => {},
            Some("RoundEnd") => {},
            Some(unknown_state) => {
                println!("unknown state {}", unknown_state)
            },
            None => {
                println!("state value is not a string")
            }
        }
    } else {
        println!("Missing 'state' key in update");
    }
}

async fn handle_location(json: Value) {

}
