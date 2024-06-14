use std::sync::Arc;

use axum::{
    extract::{
        ws::{Message, WebSocket},
        Path, WebSocketUpgrade,
    },
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension,
};
use axum_extra::TypedHeader;
use serde::Serialize;
use tokio::sync::RwLock;

use crate::game::Game;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    error: String,
}

pub async fn handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    Extension(game): Extension<Arc<RwLock<Game>>>,
    Extension(username): Extension<String>,
) -> Response {

    if let Some(TypedHeader(ua)) = user_agent {
        tracing::info!("{0} connected from {ua}", username);
    } else {
        tracing::info!("{0} connected from an unknown client", username);
    };

    {
        game.write()
            .await
            .players
            .get_mut(&username)
            .unwrap()
            .connected = true;
    }

    ws.on_upgrade(move |socket| websocket(socket, username, game))
        .into_response()
}

pub async fn websocket(mut socket: WebSocket, who: String, game: Arc<RwLock<Game>>) {
    // Make sure that the socket connection works
    if socket.send(Message::Ping(vec![1])).await.is_ok() {
        tracing::info!("Pinged {who}... ");
    } else {
        tracing::error!("Could not send ping to {who}!");
        // since we can't send messages, we have to end the connection
        return;
    }

    while let Some(Ok(message)) = socket.recv().await {
        match message {
            Message::Text(text) => {
                tracing::info!("Received text message from {who}: {text}");
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
        game.write().await.players.get_mut(&who).unwrap().connected = true;
    }
}
