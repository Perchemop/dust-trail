//! The player character.

use crate::domain::color::Rgb;
use crate::domain::world::Pos;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Player {
    pub pos: Pos,
    pub score: u32,
    pub mounted: bool,
    pub name: String,
    /// Health points — reaches 0 on death.
    pub health: u32,
}

const MAX_HEALTH: u32 = 100;

impl Player {
    pub fn new(pos: Pos) -> Self {
        Self {
            pos,
            score: 0,
            mounted: false,
            name: "Dust".to_string(),
            health: MAX_HEALTH,
        }
    }

    pub fn glyph(&self) -> char {
        '@'
    }

    pub fn fg(&self) -> Rgb {
        Rgb::WHITE
    }

    pub fn bg(&self) -> Rgb {
        Rgb::BLACK
    }

    /// Is the player dead?
    pub fn is_dead(&self) -> bool {
        self.health == 0
    }

    /// Apply damage, clamping at 0. Returns true if the player died from this hit.
    pub fn take_damage(&mut self, damage: u32) -> bool {
        self.health = self.health.saturating_sub(damage);
        self.health == 0
    }
}
