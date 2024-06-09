use crate::config::Config;
use edgedb_tokio::Client;

pub enum GameState {
    Setup,     // Only admin is allowed to join.
    Lobby,     // Allow players to join and get ready
    HidePhase, // 15 minutes for hiders to hide
    SeekPhase, // Time for seekers to find the hiders
    RoundEnd,  // End of a round, moves back to lobby.
}

pub struct Game {
    pub state: GameState,
    pub config: Config,
    pub db: Client,
}

impl Game {
    pub async fn new() -> Self {
        Game {
            state: GameState::Setup,
            config: Config::init(),
            db: edgedb_tokio::create_client().await.unwrap(),
        }
    }
}
