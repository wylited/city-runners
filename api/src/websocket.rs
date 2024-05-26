use std::net::SocketAddr;

use axum::{
    extract::{
        ws::{Message, WebSocket},
        ConnectInfo, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use axum_extra::TypedHeader;

pub async fn handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(address): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {

    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string();
    } else {
        "unknown client".to_string();
    };

    tracing::info!(?user_agent, "{address} connected."); // log the connection

    ws.on_upgrade(move |socket| websocket(socket, address))
}

// The actual websocket statemachine. One per connection.
pub async fn websocket(mut socket: WebSocket, who: SocketAddr) {
    // Make sure that the socket connection works
    if socket.send(Message::Ping(vec![1])).await.is_ok() {
        tracing::info!("Pinged {who}... ");
    } else {
        tracing::error!("Could not send ping to {who}!");
        // since we can't send messages, we have to end the connection
        return;
    }
}
