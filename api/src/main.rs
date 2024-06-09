mod auth;
mod config;
mod game;
mod location;
mod logging;
mod models;

use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};

use std::{net::SocketAddr, sync::Arc};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};

use crate::game::Game;

#[tokio::main]
async fn main() {
    let _guard = logging::init();
    let game = Game::new().await; // do later

    // address to host on
    // TODO! make config customizable
    let addr: SocketAddr = game
        .config
        .address
        .parse()
        .expect("invalid config server address");

    let app = Router::new()
        .route(
            "/",
            get(|| async { format!("City Runners, version {} \n", env!("CARGO_PKG_VERSION")) }),
        ) // initial check for the frontend.
        .route("/location", post(location::recieve))
        .route("/auth", post(auth::authenticate))
        .layer(Extension(Arc::new(game)))
        .layer(
            // a layer on the router so that it can trace all requests and responses for debugging.
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        );

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    tracing::info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap(); // serve the api
}
