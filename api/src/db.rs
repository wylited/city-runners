use std::collections::HashMap;

use edgedb_protocol::value::Value;
use edgedb_tokio::{Builder, Client};

use crate::{auth, player::Player};

//
pub struct Db {
    pub db: Client,
}

impl Db {
    pub async fn new(db_inst: &str, secret: &str) -> Db {
        let db = Client::new(
            &Builder::new()
                .secret_key(secret)
                .instance(db_inst)
                .expect("invalid secrets")
                .build_env()
                .await
                .unwrap(),
        );

        db.ensure_connected().await.unwrap(); // blocks until the db is initialized
        Db { db }
    }

    // Initializes a list of players from the database, username key.
    pub async fn init(&self) -> HashMap<String, Player> {
        const GET_USERS: &str = "select Player {username}";
        let players: Vec<Value> = self.db.query(GET_USERS, &()).await.unwrap();

        // the output will look like
        // {
        //     {
        //         "username": "user1"
        //     },
        //     {
        //         "username": "user2"
        //     }
        // }
        // so in terms of the value enum, its an Object, with fields, and each field is an optional value, each optional value if its some, is a string

        players
            .iter()
            .map(|player| {
                let fields = match player {
                    Value::Object { fields, .. } => fields,
                    _ => panic!("Player is not an object"),
                };

                let username = match fields.get(0) {
                    Some(Some(Value::Str(username))) => username,
                    _ => panic!("Username is not a string"),
                };

                let player = Player::new(username.clone(), auth::jwt(username));
                (username.clone(), player)
            })
            .collect()
    }
}
