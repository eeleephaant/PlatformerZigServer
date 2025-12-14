use crate::models::{game_state::GameState, player::Player};

#[derive(Debug, Clone)]
pub struct Lobby {
    pub id: u32,
    pub players: Vec<Player>,
    pub max_players: usize,
    pub game_state: GameState,
}

impl Lobby {}
