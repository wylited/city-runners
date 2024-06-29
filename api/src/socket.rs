use std::sync::Arc;

use axum::{
    extract::{
        ws::{Message, WebSocket},
        Query, WebSocketUpgrade,
    },
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension,
};

use futures::{executor::block_on, sink::SinkExt, stream::StreamExt};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::{auth::{self, AuthClaims}, game::Game};

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
) -> Response {
    let token = params.token;
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "lasecret".to_string());

    return match decode::<AuthClaims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(jsonwebtoken::Algorithm::HS256),
    ) {
        Ok(token_data) => {
            let claims = token_data.claims;
            let username = claims.sub;
            if auth::authenticate(claims.exp, &username, &token, game.clone()).await && !game.read().await.players[&username].connected {
                {
                    game.write()
                        .await
                        .players
                        .get_mut(&username)
                        .unwrap()
                        .connected = true;
                }

                return ws
                    .on_upgrade(move |socket| websocket(socket, username, game))
                    .into_response();
            } else {
                return (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()).into_response();
            }
        }
        Err(_) => (StatusCode::UNAUTHORIZED, "Invalid token".to_string()).into_response(),
    };
}

pub async fn websocket(socket: WebSocket, who: String, game: Arc<RwLock<Game>>) {
    let (mut tx, mut rx) = socket.split();
    if !ping(&mut tx, &who).await {
        return;
    }

    assign_stream(&game, &who, tx).await;

    tokio::spawn(async move {
        handle_messages(&mut rx, &who, game).await;
    });
}

async fn ping(tx: &mut impl SinkExt<Message>, who: &str) -> bool {
    match tx.send(Message::Ping(vec![1])).await {
        Ok(_) => {
            tracing::info!("Pinged {}... ", who);
            true
        },
        Err(_) => {
            tracing::error!("Could not send ping to {}!", who);
            false
        },
    }
}

async fn assign_stream(game: &Arc<RwLock<Game>>, who: &str, tx: impl SinkExt<Message> + Send + 'static) {
    game.write()
        .await
        .players
        .get_mut(who)
        .unwrap()
        .set_stream(tx);
}

async fn handle_messages(rx: &mut impl StreamExt<Item = Result<Message, warp::Error>>, who: &str, game: Arc<RwLock<Game>>) {
    let mut cnt = 0;
    while let Some(Ok(msg)) = rx.next().await {
        cnt += 1;
        process_message(msg, who, &game).await;
    }

    tracing::info!("Connection with {} closed.", who);
    game.write().await.players.get_mut(who).unwrap().connected = false;
}

async fn process_message(msg: Message, who: &str, game: &Arc<RwLock<Game>>) {
    match msg {
        Message::Text(text) => handle_text_message(text, who, game).await,
        Message::Binary(bin) => tracing::info!("Received binary message from {}: {:?}", who, bin),
        Message::Ping(ping) => tracing::info!("Received ping from {}: {:?}", who, ping),
        Message::Pong(pong) => tracing::info!("Received pong from {}: {:?}", who, pong),
        Message::Close(reason) => tracing::info!("Received close from {}: {:?}", who, reason),
    }
}

async fn handle_text_message(text: String, who: &str, game: &Arc<RwLock<Game>>) {
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
    game.write().await.players.get_mut(who).unwrap().send_msg(Message::Text(error_msg)).await.unwrap();
}

async fn handle_location_op(json: &serde_json::Value, who: &str, game: &Arc<RwLock<Game>>) {
    let latitude = json.get("latitude").unwrap().as_f64().unwrap();
    let longitude = json.get("longitude").unwrap().as_f64().unwrap();
    game.write().await.get_player(who.to_string()).unwrap().set_location(Location::new(latitude, longitude)).await;
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
