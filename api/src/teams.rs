pub enum TeamType {
    Seeker,
    Hider,
    Spectator,
}

pub struct Team {
    pub name: String,
    pub players: Vec<String>,
    pub ttype: TeamType,
}

impl Team {
    pub fn new(name: String) -> Team {
        Team {
            name,
            players: Vec::new(),
            ttype: TeamType::Spectator,
        }
    }

    pub fn add_player(&mut self, player: String) {
        self.players.push(player);
    }

    pub fn remove_player(&mut self, player: String) {
        self.players.retain(|p| p != &player);
    }

    pub fn is_player_on_team(&self, player: &str) -> bool {
        self.players.contains(&player.to_string())
    }

    pub fn update_type(&mut self, ttype: TeamType) {
        self.ttype = ttype;
    }
}
