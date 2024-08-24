use crate::{player, Game};
use axum::{
    extract::{Extension, Json, Request},
    http::StatusCode,
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::TypedHeader;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{TimeZone, Utc};
use edgedb_tokio::Queryable;
use headers::{authorization::Bearer, Authorization};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation, TokenData};
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
pub struct AuthClaims {
    pub sub: String,
    pub exp: usize,
    pub admin: bool,
}

pub async fn login(
    Extension(game): Extension<Arc<RwLock<Game>>>,
    Json(payload): Json<Player>,
) -> impl IntoResponse {
    let query = "select Player { username, password, admin } filter .username = <str>$0";

    #[derive(Debug, Deserialize, Serialize, Queryable)]
    pub struct DbPlayer {
        username: String,
        password: String,
        admin: bool,
    }

    let res: Result<Option<DbPlayer>, edgedb_tokio::Error> = game
        .read()
        .await
        .db
        .0
        .query_single(query, &(payload.username.clone(),))
        .await; // TODO Simplify this into the DB.

    if let Ok(Some(player)) = res { // If there is a player with the given username, log them in
        match verify(&payload.password, &player.password) {
            Ok(true) => {
                let token = jwt(&payload.username, player.admin);
                    let mut game_write = game.write().await;
                    if let Some(player) = game_write.players.get_mut(&payload.username) {
                        player.token.clone_from(&token);
                    }

                (StatusCode::ACCEPTED, Json(json!({"token": token}))).into_response()
            }
            _ => (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Invalid password"})),
            ).into_response(),
        }
    } else if res.is_ok() {
        let hashed_password = hash(&payload.password, DEFAULT_COST).unwrap();
        let query = "insert Player { username := <str>$0, password := <str>$1, admin := false}";
        if game
            .read()
            .await
            .db
            .0
            .execute(query, &(payload.username.clone(), hashed_password))
            .await
            .is_err()
        {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to insert new player"})),
            )
                .into_response();
        }

        let token = jwt(&payload.username, false);
        let mut game_write = game.write().await;
        game_write.players.insert(
            payload.username.clone(),
            player::Player::new(payload.username.clone(), token.clone()),
        );

        return (StatusCode::ACCEPTED, Json(json!({"token": token}))).into_response();
    } else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error":"Server error"})),
        ).into_response();
    }
}

pub fn jwt(username: &str, admin: bool) -> String {
    let secret = match std::env::var("JWT_SECRET") {
        Ok(secret) => secret,
        Err(_) => "defaultsecret".to_string(),
    };

    let claims = AuthClaims {
        sub: username.to_owned(),
        exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize, // expires 24 hours later
        admin,
    };

    match encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    ) {
        Ok(token) => token,
        Err(_) => panic!("Failed to encode JWT"),
    }
}

pub fn validate(token: &str) -> Result<TokenData<AuthClaims>, jsonwebtoken::errors::Error> {
    let secret = match std::env::var("JWT_SECRET") {
        Ok(secret) => secret,
        Err(_) => "defaultsecret".to_string(),
    };
    let validation = Validation::new(Algorithm::HS256);
    decode::<AuthClaims>(token, &DecodingKey::from_secret(secret.as_ref()), &validation)
}

pub async fn authenticate(
    expiry: usize,
    username: &str,
    token: &str,
    game: Arc<RwLock<Game>>,
) -> bool {
    game.read()
        .await
        .players
        .get(username)
        .map_or(false, |player| {
            player.token == token && expiry > Utc::now().timestamp() as usize
        })
}


pub async fn middleware(
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    req: Request,
    next: Next,
) -> impl IntoResponse {
    let token = bearer.token();

    match validate(token) {
        Ok(token_data) => {
            let claims = token_data.claims;
            let username = &claims.sub;
            if (chrono::Utc::now().timestamp() as usize) > claims.exp {
                return (StatusCode::UNAUTHORIZED, "Token expired".to_string()).into_response();
            }
            let mut req = req;
            req.extensions_mut().insert(username.to_string());
            next.run(req).await.into_response()
        }
        Err(_) => (StatusCode::UNAUTHORIZED, "Invalid token".to_string()).into_response(),
    }
}
