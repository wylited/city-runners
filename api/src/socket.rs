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

use crate::{auth, game::Game};

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    error: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
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

    return match decode::<Claims>(
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
    // Make sure that the socket connection works
    if tx.send(Message::Ping(vec![1])).await.is_ok() {
        tracing::info!("Pinged {who}... ");
    } else {
        tracing::error!("Could not send ping to {who}!");
        // since we can't send messages, we have to end the connection
        return;
    }

    // give tx to the game
    {
        game.write()
            .await
            .players
            .get_mut(&who)
            .unwrap()
            .set_stream(tx);
    }

    tokio::spawn(async move {
        let mut cnt = 0;
        while let Some(Ok(msg)) = rx.next().await {
            cnt += 1;
            match msg {
                Message::Text(text) => {
                    tracing::info!("Received text message from {who}: {text}");

                    let json: serde_json::Value = match serde_json::from_str(&text) {
                        Ok(json) => json,
                        Err(_) => {
                            game.write().await.players.get_mut(&who).unwrap().send_msg(Message::Text(
                                serde_json::to_string(&ErrorResponse {
                                    error: "Invalid JSON".to_string(),
                                })
                                .unwrap(),
                            )).await;
                            tracing::error!("Invalid JSON from {who}: {text}");
                            continue;
                        }
                    };

                   json.get("op").map(|op| {
                        if let serde_json::Value::String(op) = op {
                            match op.as_str() {
                                "move" => {

                                }
                                "chat" => {
                                    let msg = json.get("msg").unwrap().as_str().unwrap();
                                    tracing::info!("{}", msg);
                                    block_on(chat(who.clone(), game.clone(), msg.to_string())); // await basically
                                }
                                _ => {
                                    tracing::error!("Invalid operation from {who}: {op}");
                                }
                            }
                        }
                    });
                }
                Message::Binary(bin) => {
                    tracing::info!("Received binary message from {who}: {:?}", bin);
                }
                Message::Ping(ping) => {
                    tracing::info!("Received ping from {who}: {:?}", ping);
                }
                Message::Pong(pong) => {
                    tracing::info!("Received pong from {who}: {:?}", pong);
                }
                Message::Close(reason) => {
                    tracing::info!("Received close from {who}: {:?}", reason);
                    break;
                }
            }
        }

        tracing::info!("Connection with {who} closed.");
        {
            game.write().await.players.get_mut(&who).unwrap().connected = false;
        }
        cnt
    });
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
