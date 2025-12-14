use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LobbyLeave {
    pub players_count: u8,
    pub max_players: u8,
}
