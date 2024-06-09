use edgedb_tokio::Queryable;

pub enum PlayerType {
    Hider,
    SecondarySeeker,
    PrimarySeeker,
    Admin,
}

#[derive(Queryable, Debug)]
pub struct Player {
    pub username: String,
    pub password: String,
}

pub enum TeamType {
    Hider,
    Seeker,
}

pub struct Team {
    team_type: TeamType,
    player_ids: Vec<String>,
}
