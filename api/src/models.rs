pub enum PlayerType {
    Hider,
    SecondarySeeker,
    PrimarySeeker,
    Admin,
}
pub struct Player {
    username: String,
    password: String,
    jwt_token: String,
}

pub enum TeamType {
    Hider,
    Seeker,
}

pub struct Team {
    team_type: TeamType,
    players: Vec<Player>,
}

pub enum GameState {
    Lobby,
    Hiding,
    Seeking,
    GameEnd,
}

pub struct Game {
    teams: Vec<Team>,
    game_password: String,
    current_state: GameState,
}
