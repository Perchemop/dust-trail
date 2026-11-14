//! Serialisable game save data.

use crate::domain::game::Game;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SaveData {
    pub version: String,
    pub world: crate::domain::world::World,
    pub player: crate::domain::player::Player,
    pub turn: u64,
}

impl SaveData {
    pub fn new(game: &Game) -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            world: game.world.clone(),
            player: game.player.clone(),
            turn: game.turn,
        }
    }
}
