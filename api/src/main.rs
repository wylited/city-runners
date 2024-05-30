pub mod location;
pub mod logging;
pub mod config;
pub mod models;
pub mod game;

use axum::{
    routing::{get, post},
    Router,
};

use std::net::SocketAddr;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};

use crate::game::Game;

#[tokio::main]
async fn main() {
    let _guard = logging::init();
    let game = Game::new();

    // address to host on
    // TODO! make config customizable
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    let app = Router::new()
        .route(
            "/",
            get(|| async { format!("City Runners, version {} \n", env!("CARGO_PKG_VERSION")) }),
        ) // initial check for the frontend.
        .route("/location", post(location::recieve))
        .layer(
            // a layer on the router so that it can trace all requests and responses for debugging.
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        );

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    tracing::info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap(); // serve the api
}
