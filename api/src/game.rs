use crate::config::Config;

pub enum GameState {
    Setup,     // Only admin is allowed to join.
    Lobby,     // Allow players to join and get ready
    HidePhase, // 15 minutes for hiders to hide
    SeekPhase, // Time for seekers to find the hiders
    RoundEnd,  // End of a round, moves back to lobby.
}

pub struct Game {
    state: GameState,
    config: Config,
}

impl Game {
    pub fn new() -> Self {
        Game {
            state: GameState::Setup,
            config: Config::init(),
        }
    }
}
