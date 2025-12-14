#[derive(Debug, Clone)]
pub struct Lobby {
    pub id: u32,
    pub players: Vec(Player),
    pub max_players: usize,
}

impl Lobby {
    pub fn new(id: u32, max_players: usize) -> Self {
        Self {
            id,
            players: Vec::new(),
            max_players,
        }
    }
    pub fn add_player(&mut self ,player: Player) -> bool {
        if self.is_full(){
            return false;
        }
        self.players.push(player);
        return true;
    }
    pub fn is_full(&self) -> bool {
        return self.players.len() >= self.max_players;
    }
    pub fn remove_player(&mut self, player: Player){
        self.players.remove(player);
    }
}
