mod auth;
mod db;
mod game;
mod location;
mod logging;
mod player;
mod router;
mod socket;
mod teams;
mod station;
mod graph;
mod states;
mod state_machine;

use std::sync::Arc;

use axum::{extract::Extension, http::StatusCode, middleware, response::IntoResponse, Json};
use serde_json::json;
use state_machine::GameStateMachine;
use states::{GameState, LobbyState};
use tokio::sync::RwLock;


use crate::{game::Game, router::router, states::State};

#[shuttle_runtime::main]
pub async fn axum(
    #[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore,
) -> shuttle_axum::ShuttleAxum {
    // logging::init();
    let lobby_state = LobbyState::new();
    let state = Arc::new(RwLock::new(GameState::Lobby(lobby_state)));

    let (game, rx) = Game::new(
        &secrets.get("EDGEDB_INSTANCE").unwrap(),
        &secrets.get("EDGEDB_SECRET_KEY").unwrap(),
        state.clone(),
    ).await;

    {
        let mut s = state.write().await;
        if let GameState::Lobby(lobbystate) = &mut *s {
            lobbystate.init(game.clone());
        }
    }

    let mut state_machine = GameStateMachine {
        state,
        rx,
        game: game.clone(),
    };

    tokio::spawn(async move {
        state_machine.run().await;
    });

    let app = router().layer(Extension(game));

    // .layer(
    //     // a layer on the router so that it can trace all requests and responses for debugging.
    //     TraceLayer::new_for_http()
    //         .make_span_with (DefaultMakeSpan::default().include_headers(true)),
    // );

    Ok(app.into())
}

async fn validate_token(Extension(username): Extension<String>) -> impl IntoResponse {
    (StatusCode::OK, Json(json!({"username": username})))
}
