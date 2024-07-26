mod auth;
mod db;
mod game;
mod location;
mod logging;
mod player;
mod router;
mod socket;
mod teams;
mod timer;

use axum::{extract::Extension, http::StatusCode, middleware, response::IntoResponse, Json};
use serde_json::json;

use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{game::Game, router::router};

#[shuttle_runtime::main]
pub async fn axum(
    #[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore,
) -> shuttle_axum::ShuttleAxum {
    // logging::init();
    let game = Game::new(
        &secrets.get("EDGEDB_INSTANCE").unwrap(),
        &secrets.get("EDGEDB_SECRET_KEY").unwrap(),
    )
    .await;

    let app = router().layer(Extension(Arc::new(RwLock::new(game))));

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
