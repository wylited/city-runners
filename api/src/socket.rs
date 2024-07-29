use std::sync::Arc;

use axum::{
    extract::{
        ws::{Message, WebSocket},
        Query, WebSocketUpgrade,
    },
    http::StatusCode,
    response::{IntoResponse},
    Extension,
};
use futures::{
    sink::SinkExt,
    stream::{SplitSink, SplitStream, StreamExt},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::RwLock;

use crate::{
    auth::{self, validate},
    game::Game,
    location::handle_location_op,
};

pub type Tx = Arc<RwLock<SplitSink<WebSocket, Message>>>;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    error: String,
}

#[derive(Deserialize)]
pub struct QueryParams {
    token: String,
}

pub async fn handler(
    ws: WebSocketUpgrade,
    Query(params): Query<QueryParams>,
    Extension(game): Extension<Arc<RwLock<Game>>>,
) -> impl IntoResponse {
    let token = params.token;

    return match validate(&token) {
        Ok(token_data) => {
            let claims = token_data.claims;
            let username = claims.sub;
            if !auth::authenticate(claims.exp, &username, &token, game.clone()).await {
                return (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()).into_response()
            } else if game.read().await.players[&username].connected {
                return (StatusCode::CONFLICT, "Already connected".to_string()).into_response()
            }

            game.write()
                .await
                .players
                .get_mut(&username)
                .unwrap()
                .connected = true;

            ws.on_upgrade(move |socket| websocket(socket, username, game))
              .into_response()
        }
        _ => (StatusCode::UNAUTHORIZED, "Invalid token".to_string()).into_response(),
    };
}

pub async fn websocket(socket: WebSocket, who: String, game: Arc<RwLock<Game>>) {
    let (mut tx, rx) = socket.split();
    if !ping(&mut tx, &who).await {
        return;
    }

    assign_stream(&game, &who, tx).await;

    tokio::spawn(async move {
        handle_messages(rx, &who, game).await;
    });
}

async fn ping(tx: &mut SplitSink<WebSocket, Message>, who: &str) -> bool {
    match tx.send(Message::Ping(vec![1])).await {
        Ok(_) => {
            tracing::info!("Pinged {}... ", who);
            true
        }
        Err(_) => {
            tracing::error!("Could not send ping to {}!", who);
            false
        }
    }
}

async fn assign_stream(game: &Arc<RwLock<Game>>, who: &str, tx: SplitSink<WebSocket, Message>) {
    game.write()
        .await
        .players
        .get_mut(who)
        .unwrap()
        .set_stream(tx);
}

async fn handle_messages(mut rx: SplitStream<WebSocket>, who: &str, game: Arc<RwLock<Game>>) {
    let mut cnt = 0;
    while let Some(Ok(msg)) = rx.next().await {
        cnt += 1;
        process_message(msg, who, &game).await;
    }

    tracing::info!("Connection with {} closed. {} messages", who, cnt);
    game.write().await.players.get_mut(who).unwrap().connected = false;
}

async fn process_message(msg: Message, who: &str, game: &Arc<RwLock<Game>>) {
    match msg {
        Message::Text(text) => handle_json_message(text, who, game).await,
        Message::Binary(bin) => tracing::info!("Received binary message from {}: {:?}", who, bin),
        Message::Ping(ping) => tracing::info!("Received ping from {}: {:?}", who, ping),
        Message::Pong(pong) => tracing::info!("Received pong from {}: {:?}", who, pong),
        Message::Close(reason) => tracing::info!("Received close from {}: {:?}", who, reason),
    }
}

async fn handle_json_message(text: String, who: &str, game: &Arc<RwLock<Game>>) {
    tracing::info!("Received text message from {}: {}", who, text);

    let json = match serde_json::from_str::<serde_json::Value>(&text) {
        Ok(json) => json,
        Err(_) => {
            send_invalid_json_error(who, game).await;
            tracing::error!("Invalid JSON from {}: {}", who, text);
            return;
        }
    };

    if let Some(op) = json.get("op").and_then(|op| op.as_str()) {
        match op {
            "location" => handle_location_op(&json, who, game).await,
            "chat" => handle_chat_op(&json, who, game).await,
            _ => tracing::error!("Invalid operation from {}: {}", who, op),
        }
    }
}

async fn send_invalid_json_error(who: &str, game: &Arc<RwLock<Game>>) {
    let error_response = json!({ "error": "Invalid JSON" });
    let error_msg = serde_json::to_string(&error_response).unwrap();
    game.write()
        .await
        .players
        .get_mut(who)
        .unwrap()
        .send_msg(Message::Text(error_msg))
        .await
        .unwrap();
}

async fn handle_chat_op(json: &serde_json::Value, who: &str, game: &Arc<RwLock<Game>>) {
    let msg = json.get("msg").unwrap().as_str().unwrap();
    tracing::info!("{}", msg);
    chat(who.to_string(), game.clone(), msg.to_string()).await;
}

pub async fn chat(who: String, game: Arc<RwLock<Game>>, msg: String) {
    let msg = serde_json::to_string(&serde_json::json!({
        "op": "chat",
        "msg": msg,
        "who": who
    }))
    .unwrap();
    if let Err(e) = game.write().await.broadcast(Message::Text(msg)).await {
        tracing::error!("{}", e);
    };
}
