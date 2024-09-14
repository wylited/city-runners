use crate::*;
use axum::{
    routing::{get, patch, post},
    Router,
};

pub fn router() -> Router {
    Router::new()
        .route(
            "/",
            get(|| async { format!("City Runners, version {} \n", env!("CARGO_PKG_VERSION")) }),
        )
        .route("/login", post(auth::login))
        .route(
            "/validate",
            get(validate_token).layer(middleware::from_fn(auth::middleware)),
        )
        .route(
            "/teams/:name",
            get(teams::get)
                .post(teams::create)
                .layer(middleware::from_fn(auth::middleware)),
        )
        .route(
            "/teams/:name/join",
            post(teams::join).layer(middleware::from_fn(auth::middleware)),
        )
        .route(
            "/teams/:name/leave",
            post(teams::leave).layer(middleware::from_fn(auth::middleware)),
        )
        .route(
            "/teams/:name",
            patch(teams::update_team_name).layer(middleware::from_fn(auth::middleware)),
        )
        .route("/teams", get(teams::getall))
        .route("/convert", get(location::convert))
        .route("/ws", get(socket::handler))
        .route("/stations", get(station::get))
        .route("/state", get(state_machine::get))
        .route("/start", post(state_machine::start).layer(middleware::from_fn(auth::middleware_admin)))
}
