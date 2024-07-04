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
        .route(
            "/teams",
            get(teams::getall).layer(middleware::from_fn(auth::middleware)),
        )
        .route("/convert", get(location::convert))
        .route("/ws", get(socket::handler))
        .route(
            "/ready",
            post(player::ready).layer(middleware::from_fn(auth::middleware)),
        )
        .route(
            "/timer",
            get(timer::get).layer(middleware::from_fn(auth::middleware)),
        )
}
