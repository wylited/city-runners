use crate::{game, Game};
use axum::extract::{Extension, Json};
use bcrypt::{hash, verify, DEFAULT_COST};
use edgedb_tokio::Queryable;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Deserialize, Serialize, Queryable)]
pub struct Player {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    token: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub async fn authenticate(
    Extension(game): Extension<Arc<RwLock<Game>>>,
    Json(payload): Json<Player>,
) -> Result<Json<AuthResponse>, (axum::http::StatusCode, Json<ErrorResponse>)> {
    let query = "select Player { username, password } filter .username = <str>$0";
    let res: Result<Option<Player>, edgedb_tokio::Error> = game
        .read()
        .await
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
            {
                // in a scope so that write lock is dropped immediately
                game.write()
                    .await
                    .players
                    .get_mut(&payload.username)
                    .unwrap()
                    .token = token.clone();
            }
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
        game.read()
            .await
            .db
            .execute(query, &(payload.username.clone(), hashed_password))
            .await
            .unwrap();

        let token = generate_jwt(&payload.username.clone());
        {
            // write lock to add a player droped once scope completes
            game.write().await.players.insert(
                payload.username.clone(),
                game::Player::new(payload.username.clone(), token.clone()),
            );
        }
        return Ok(Json(AuthResponse { token }));
    }
}

pub fn generate_jwt(username: &str) -> String {
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
