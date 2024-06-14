use crate::{game, Game};
use axum::{
    extract::{Extension, Json, Request},
    http::StatusCode,
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::TypedHeader;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use edgedb_tokio::Queryable;
use headers::{authorization::Bearer, Authorization};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Deserialize, Serialize, Queryable)]
pub struct Player {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub async fn login(
    Extension(game): Extension<Arc<RwLock<Game>>>,
    Json(payload): Json<Player>,
) -> impl IntoResponse {
    let query = "select Player { username, password } filter .username = <str>$0";
    let res: Result<Option<Player>, edgedb_tokio::Error> = game
        .read()
        .await
        .db
        .query_single(query, &(payload.username.clone(),))
        .await;

    if let Ok(Some(player)) = res {
        if verify(&payload.password, &player.password).unwrap_or(false) {
            let token = jwt(&payload.username);
            {
                let mut game_write = game.write().await;
                if let Some(player) = game_write.players.get_mut(&payload.username) {
                    player.token.clone_from(&token);
                }
            }

            (StatusCode::ACCEPTED, Json(json!({"token": token}))).into_response()
        } else {
            (StatusCode::UNAUTHORIZED, Json(json!({"error": "Invalid password"}))).into_response()
        }
    } else if res.is_ok() {
        let hashed_password = hash(&payload.password, DEFAULT_COST).unwrap();
        let query = "insert Player { username := <str>$0, password := <str>$1 }";
        if game
            .read()
            .await
            .db
            .execute(query, &(payload.username.clone(), hashed_password))
            .await
            .is_err()
        {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to insert new player"})),
            ).into_response();
        }

        let token = jwt(&payload.username);
        {
            let mut game_write = game.write().await;
            game_write.players.insert(
                payload.username.clone(),
                game::Player::new(payload.username.clone(), token.clone()),
            );
        }
        return (StatusCode::ACCEPTED, Json(json!({"token": token}))).into_response();
    } else {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error":"Server error"}))).into_response();
    }
}

pub fn jwt(username: &str) -> String {
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "lasecret".to_string()); // TODO: Make this a config variable or understand what environment is
    let claims = Claims {
        sub: username.to_owned(),
        exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize, // expires 24 hours later
    };
    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap()
}

async fn authenticate(expiry: usize, username: &str, token: &str, game: Arc<RwLock<Game>>) -> bool {
    game.read()
        .await
        .players
        .get(username)
        .map_or(false, |player| {
            player.token == token && expiry > Utc::now().timestamp() as usize
        })
}

pub async fn middleware(
    Extension(game): Extension<Arc<RwLock<Game>>>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    req: Request,
    next: Next,
) -> impl IntoResponse {
    let token = bearer.token();
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "lasecret".to_string());

    if let Ok(token_data) = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(jsonwebtoken::Algorithm::HS256),
    ) {
        let claims = token_data.claims;
        let username = &claims.sub;
        if authenticate(claims.exp, &claims.sub, token, game).await {
            let mut req = req;
            req.extensions_mut().insert(username.to_string());
            next.run(req).await.into_response()
        } else {
            (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()).into_response()
        }
    } else {
        (StatusCode::UNAUTHORIZED, "Invalid token".to_string()).into_response()
    }
}
