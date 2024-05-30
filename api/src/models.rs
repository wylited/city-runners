pub enum PlayerType {
    Hider,
    SecondarySeeker,
    PrimarySeeker,
    Admin,
}

pub struct Player {
    pid: String,
    username: String,
    password_hash: String,
}

pub enum TeamType {
    Hider,
    Seeker,
}

pub struct Team {
    team_type: TeamType,
    player_ids: Vec<String>,
}
