#[derive(Debug, Clone)]
pub struct Lobby {
    pub id: u32,
    pub players: Vec(Player),
    pub max_players: usize,
}

impl Lobby {}
