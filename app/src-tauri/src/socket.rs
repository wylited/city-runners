use futures_util::{SinkExt, StreamExt, Stream, stream::SplitStream};
use serde_json::json;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time;
use tokio_tungstenite::{connect_async, tungstenite::Message, WebSocketStream};
use tauri::{AppHandle, Wry, Manager};
use tauri_plugin_store::{with_store, StoreCollection};

pub async fn socket(app: AppHandle, token: String) {
    let stores = match app.app_handle().try_state::<StoreCollection<Wry>>() {
        Some(stores) => stores,
        None => {
            eprintln!("Store not found");
            return;
        }
    };
    let path = PathBuf::from("store.bin");

    let address = match with_store(app.app_handle().clone(), stores, path, |store| {
        let address = store.get("address").expect("failed to get address").to_string().trim_matches('"').to_string();
        Ok(address)
    }) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Error getting token and address: {}", e);
            return;
        }
    };

    let ws_url = address.replace("https://", "wss://") + "/ws?token=" + &token.trim_matches('"').to_string();
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
    let (mut tx, mut rx) = mpsc::channel::<Message>(32);

    tokio::spawn(async move {
        while let Some(Ok(msg)) = read.next().await {
            match msg {
                Message::Text(text) => handle_json_message(text).await,
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

async fn handle_json_message(text: String) {
    match serde_json::from_str::<serde_json::Value>(&text) {
        Ok(json) => {
            // Handle the JSON message
            println!("Received JSON: {:?}", json);
        }
        Err(_) => {
            println!("Invalid JSON");
        }
    }
}
