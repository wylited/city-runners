use crate::{models::Player, Game};
use axum::extract::{Extension, Json};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    token: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    error: String,
}

pub async fn authenticate(
    Extension(game): Extension<Arc<Game>>,
    Json(payload): Json<AuthRequest>,
) -> Result<Json<AuthResponse>, (axum::http::StatusCode, Json<ErrorResponse>)> {
    let query = "select Player { username, password } filter .username = <str>$0";
    let res: Result<Option<Player>, edgedb_tokio::Error> = game
        .db
        .query_single(query, &(payload.username.clone(),))
        .await;

    println!("{:#?}", res);

    if !res.is_ok() {
        return Err((
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Server Error".into(),
            }),
        ));
    }


    if let Some(player) = res.unwrap() {
        if verify(&payload.password, &player.password).unwrap() {
            let token = generate_jwt(&payload.username.clone());
            return Ok(Json(AuthResponse { token }));
        }
        return Err((
            axum::http::StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                error: "Invalid password".into(),
            }),
        ));
    } else {
        let hashed_password = hash(&payload.password, DEFAULT_COST).unwrap();
        let query = "insert Player { username := <str>$0, password := <str>$1 }";
        game.db
            .execute(query, &(payload.username.clone(), hashed_password))
            .await
            .unwrap();

        let token = generate_jwt(&payload.username.clone());
        return Ok(Json(AuthResponse { token }));
    }
}

fn generate_jwt(username: &str) -> String {
    let secret = "lasecret"; // make config variable
    let claims = Claims {
        sub: username.to_owned(),
        exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize, // expires a day later
    };
    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap()
}

#[derive(Debug, Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}
