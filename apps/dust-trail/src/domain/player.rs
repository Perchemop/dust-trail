//! The player character.

use crate::domain::color::Rgb;
use crate::domain::world::{ItemKind, Pos};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Player {
    pub pos: Pos,
    pub score: u32,
    pub mounted: bool,
    pub name: String,
    /// Health points — reaches 0 on death.
    pub health: u32,
    /// Inventory of collected items awaiting use.
    pub inventory: Vec<ItemKind>,
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
            inventory: Vec::new(),
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

    /// Toggle mounted status.
    pub fn toggle_mount(&mut self) {
        self.mounted = !self.mounted;
    }

    /// Use the first item of `kind` from inventory, returning its effect score.
    /// Returns `None` if no such item is in inventory.
    pub fn use_item(&mut self, kind: ItemKind) -> Option<u32> {
        if let Some(idx) = self.inventory.iter().position(|i| *i == kind) {
            self.inventory.remove(idx);
            Some(match kind {
                ItemKind::Loot => 5,
                ItemKind::Horse => {
                    self.mounted = true;
                    0
                }
            })
        } else {
            None
        }
    }
}
