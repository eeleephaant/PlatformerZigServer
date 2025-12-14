use crate::models::{game_state::GameState, player::{self, Player}};

#[derive(Debug, Clone)]
pub struct Lobby {
    pub id: u32,
    pub players: Vec<Player>,
    pub max_players: usize,
    pub game_state: GameState,
}

impl Lobby {
    pub fn new(id: u32, max_players: usize) -> Self {
        Self {
            id,
            players: Vec::new(),
            max_players,
            game_state: GameState::InLobby,
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
        for p in 0..self.players.len() {
            if self.players[p].id == player.id{
                self.players.remove(p);
                break;
            }
        }
    }
    pub fn start_game(&mut self){
        self.game_state = GameState::InGame;
    }
    pub fn finish_game(&mut self){
        self.game_state = GameState::InLobby;
    }
    pub fn change_max_players(&mut self, max_players: usize) -> bool{
        if self.players.len() > max_players{
            return false;
        }
        self.max_players = max_players;
        return true;
    }
}
