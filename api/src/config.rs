use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub game_password: String,
    pub database_url: String,
    pub address: String,
    // Add more config values as needed
}

impl Config {
    pub fn init() -> Config {
        let config_path = "config.json";
        let config_exists = Path::new(config_path).exists();

        if config_exists {
            let config_str = fs::read_to_string(config_path).expect("Failed to read config file");
            let config: Config =
                serde_json::from_str::<_>(&config_str).expect("Failed to deserialize config");
            config
        } else {
            // If the config file doesn't exist, create a default one
            let default_config = Config {
                game_password: "default_game_password".to_string(),
                database_url: "postgres://username:password@localhost/mygame".to_string(),
                address: "0.0.0.0:3000".to_string(),
                // Initialize other config values with default values
            };
            let config_str = serde_json::to_string_pretty(&default_config).unwrap();
            fs::write(config_path, config_str).expect("Failed to write default config file");
            default_config
        }
    }
}
