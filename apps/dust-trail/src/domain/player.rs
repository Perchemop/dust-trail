//! The player character.

use crate::domain::color::Rgb;
use crate::domain::world::Pos;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Player {
    pub pos: Pos,
    pub score: u32,
    pub mounted: bool,
    pub name: String,
}

impl Player {
    pub fn new(pos: Pos) -> Self {
        Self {
            pos,
            score: 0,
            mounted: false,
            name: "Dust".to_string(),
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
}