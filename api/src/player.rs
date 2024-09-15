use std::sync::Arc;

use axum::{
    extract::ws::{Message, WebSocket},
};
use futures::{stream::SplitSink, SinkExt};
use tokio::sync::RwLock;

use crate::{location::Location, socket::Tx};

#[derive(Debug)]
pub enum PlayerType {
    Hider,
    SecondarySeeker,
    PrimarySeeker,
    Admin,
    Spectator,
}

#[derive(Debug)]
pub struct Player {
    pub username: String, // kinda like backlink, guaranteed the same as the key for players
    pub token: String,
    pub connected: bool,
    pub ptype: PlayerType,
    pub stream: Option<Tx>,
    pub current_location: Option<Location>,
    pub team: Option<String>,
}

impl Player {
    pub fn new(username: String, token: String) -> Self {
        Self {
            username,
            token,
            ptype: PlayerType::Spectator,
            connected: false,
            stream: None,
            current_location: None,
            team: None,
        }
    }

    pub fn update_token(&mut self, token: String) {
        tracing::info!("Updating token from {0} to {1}", self.token, token);
        self.token = token;
    }

    pub fn update_connection(&mut self, connected: bool) {
        tracing::info!(
            "Updating connection from {0} to {1}",
            self.connected,
            connected
        );
        self.connected = connected;
        if !connected {
            self.stream = None;
        }
    }

    pub fn set_location(&mut self, location: Location) {
        self.current_location = Some(location);
    }

    pub fn set_stream(&mut self, tx: SplitSink<WebSocket, Message>) {
        self.stream = Some(Arc::new(RwLock::new(tx)));
    }

    pub async fn send_msg(&self, msg: Message) -> Result<(), String> {
        if let Some(ref arctx) = self.stream {
            let mut tx = arctx.write().await;
            let res = tx.send(msg).await;
            if let Err(e) = res {
                return Err(e.to_string());
            }
            Ok(())
        } else {
            Err("No connection found".to_string())
        }
    }
}
