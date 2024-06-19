mod auth;
mod config;
mod game;
mod location;
mod logging;
mod models;
pub mod socket;

use axum::{
    extract::Extension, http::StatusCode, middleware, response::IntoResponse, routing::{get, post}, Json, Router
};
use serde_json::json;

use std::{net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};

use crate::game::Game;

#[shuttle_runtime::main]
pub async fn axum(#[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore,) -> shuttle_axum::ShuttleAxum {
    // logging::init();
    let game = Game::new(&secrets.get("EDGEDB_INSTANCE").unwrap(), &secrets.get("EDGEDB_SECRET_KEY").unwrap() ).await; // do later

    let app = Router::new()
        .route(
            "/",
            get(|| async { format!("City Runners, version {} \n", env!("CARGO_PKG_VERSION")) }),
        ) // initial check for the frontend.
        .route("/location", post(location::recieve))
        .route("/login", post(auth::login))
        .route("/validate", get(validate_token).layer(middleware::from_fn(auth::middleware)))
        .route("/ws/{token}", get(socket::handler))
        .layer(Extension(Arc::new(RwLock::new(game))));
        // .layer(
        //     // a layer on the router so that it can trace all requests and responses for debugging.
        //     TraceLayer::new_for_http()
        //         .make_span_with (DefaultMakeSpan::default().include_headers(true)),
        // );

    Ok(app.into())
}

// #[tokio::main]
// async fn main() {
//     logging::init();
//     let game = Game::new().await; // do later

//     // address to host on
//     // TODO! make config customizable
//     let addr: SocketAddr = game
//         .config
//         .address
//         .parse()
//         .expect("invalid config server address");

//     let app = Router::new()
//         .route(
//             "/",
//             get(|| async { format!("City Runners, version {} \n", env!("CARGO_PKG_VERSION")) }),
//         ) // initial check for the frontend.
//         .route("/location", post(location::recieve))
//         .route("/login", post(auth::login))
//         .route("/validate", get(validate_token).layer(middleware::from_fn(auth::middleware)))
//         .route("/ws", get(socket::handler).layer(middleware::from_fn(auth::middleware)))
//         .layer(Extension(Arc::new(RwLock::new(game))))
//         .layer(
//             // a layer on the router so that it can trace all requests and responses for debugging.
//             TraceLayer::new_for_http()
//                 .make_span_with(DefaultMakeSpan::default().include_headers(true)),
//         );

//     let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

//     tracing::info!("listening on {}", listener.local_addr().unwrap());

//     axum::serve(listener, app).await.unwrap(); // serve the api
// }

async fn validate_token(
    Extension(username): Extension<String>,
) -> impl IntoResponse {
    (StatusCode::OK, Json(json!({"username": username})))
}
