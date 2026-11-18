//! Core game state: turn-based Western adventure. Movement, NPC turns,
//! interactions, save/load, and Frame production all live here.
//!
//! Clean-room: original mechanics; nothing copied from a commercial product.

use std::path::Path;

use crate::domain::cell::Cell;
use crate::domain::frame::Frame;
use crate::domain::iso::{screen_depth, world_to_screen};
use crate::domain::player::Player;
use crate::domain::save::SaveData;
use crate::domain::slots::SaveStore;
use crate::domain::sound::SoundSettings;
#[cfg(feature = "sound")]
use crate::domain::sound::{SoundEffect, SoundManager};
use crate::domain::tile::TileType;
use crate::domain::world::{
    ItemKind, Npc, NpcKind, Pos, WantedLevel, World, default_world, spawn_deputies_for_wanted,
};
use rand::{SeedableRng, seq::SliceRandom};
use rand::rngs::StdRng;
use std::path::Path;

/// A movement direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn offset(self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

/// The result of an interaction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Interaction {
    None,
    Talked { npc: String, line: String },
    Collected { kind: ItemKind },
    Blocked { reason: String },
}

/// The game state machine.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Game {
    pub world: World,
    pub player: Player,
    pub turn: u64,
    pub message: String,
    pub game_over: bool,
    pub won: bool,
    /// RNG for AI decisions (seeded for reproducibility).
    #[serde(skip, default = "default_rng")]
    rng: StdRng,
    /// Sound manager for optional sound effects.
    #[cfg(feature = "sound")]
    #[serde(skip, default = "default_sound_manager")]
    pub sound_manager: SoundManager,
    /// Sound settings for persistence (used when sound feature is disabled).
    #[cfg(not(feature = "sound"))]
    pub sound_settings: SoundSettings,
}

#[cfg(feature = "sound")]
fn default_sound_manager() -> SoundManager {
    SoundManager::new().unwrap_or_default()
}

fn default_rng() -> StdRng {
    StdRng::from_os_rng()
}

impl Game {
    /// Create a new game with a procedurally generated world using a random seed.
    pub fn new() -> Self {
        let seed: u64 = rand::random();
        let world = World::generate(seed);
        let player = Player::new(Pos::new(2, 2));
        Self {
            world,
            player,
            turn: 0,
            message: "Welcome to Dust Trail. Use WASD to move, E to interact, S to save, "
                "L to load, Q to quit."
                .to_string(),
            game_over: false,
            won: false,
            rng: StdRng::from_os_rng(),
            #[cfg(feature = "sound")]
            sound_manager: SoundManager::new().unwrap_or_default(),
            #[cfg(not(feature = "sound"))]
            sound_settings: SoundSettings::new_stub(),
        }
    }

    /// Create a new game with a specific RNG seed (for testing/procedural).
    pub fn new_with_seed(seed: u64) -> Self {
        let world = World::generate(seed);
        let player = Player::new(Pos::new(2, 2));
        Self {
            world,
            player,
            turn: 0,
            message: "Welcome to Dust Trail. Use WASD to move, E to interact, S to save, "
                "L to load, Q to quit."
                .to_string(),
            game_over: false,
            won: false,
            rng: StdRng::seed_from_u64(seed),
            #[cfg(feature = "sound")]
            sound_manager: SoundManager::new().unwrap_or_default(),
            #[cfg(not(feature = "sound"))]
            sound_settings: SoundSettings::new_stub(),
        }
    }

    /// Create a new game with a default (static) world.
    pub fn new_static() -> Self {
        let world = default_world();
        let player = Player::new(Pos::new(2, 2));
        Self {
            world,
            player,
            turn: 0,
            message: "Welcome to Dust Trail. Use WASD to move, E to interact, S to save, "
                "L to load, Q to quit."
                .to_string(),
            game_over: false,
            won: false,
            rng: StdRng::from_os_rng(),
            #[cfg(feature = "sound")]
            sound_manager: SoundManager::new().unwrap_or_default(),
            #[cfg(not(feature = "sound"))]
            sound_settings: SoundSettings::new_stub(),
        }
    }

    /// Render the current state as a Frame for either frontend.
    pub fn frame(&self) -> Frame {
        // --- Isometric layout ---
        // World (x, y) -> screen (sx, sy): sx = (x-y)*2, sy = x+y.
        // Nearer tiles (larger screen depth) are drawn last so they overlap
        // farther ones (painter's algorithm).
        let mut drawn: Vec<(i32, i32, Cell)> = Vec::new();

        // World tiles, back-to-front by screen depth.
        let mut tile_entries: Vec<(i32, i32, i32, TileType)> = Vec::new();
        for y in 0..self.world.height {
            for x in 0..self.world.width {
                let wx = x;
                let wy = y;
                let tile = self.world.tiles[wy as usize][wx as usize];
                tile_entries.push((screen_depth(wx, wy), wx, wy, tile));
            }
        }
        tile_entries.sort_by_key(|e| e.0);
        for (_, wx, wy, tile) in tile_entries {
            let (sx, sy) = world_to_screen(wx, wy);
            drawn.push((sx, sy, Cell::new(tile.glyph(), tile.fg(), tile.bg())));
        }

        // Items (not collected), drawn over tiles.
        for item in &self.world.items {
            if !item.collected && self.world.in_bounds(item.pos) {
                let (sx, sy) = world_to_screen(item.pos.x, item.pos.y);
                drawn.push((
                    sx,
                    sy,
                    Cell::new(item.kind.glyph(), item.kind.fg(), item.kind.bg()),
                ));
            }
        }

        // NPCs, drawn over items.
        for npc in &self.world.npcs {
            if self.world.in_bounds(npc.pos) {
                let (sx, sy) = world_to_screen(npc.pos.x, npc.pos.y);
                drawn.push((
                    sx,
                    sy,
                    Cell::new(npc.kind.glyph(), npc.kind.fg(), npc.kind.bg()),
                ));
            }
        }

        // Player, drawn last (top).
        let (px, py) = world_to_screen(self.player.pos.x, self.player.pos.y);
        drawn.push((
            px,
            py,
            Cell::new(self.player.glyph(), self.player.fg(), self.player.bg()),
        ));

        // Convert to frame grid.
        let min_x = drawn.iter().map(|d| d.0).min().unwrap_or(0);
        let max_x = drawn.iter().map(|d| d.0).max().unwrap_or(0);
        let min_y = drawn.iter().map(|d| d.1).min().unwrap_or(0);
        let max_y = drawn.iter().map(|d| d.1).max().unwrap_or(0);

        let width = (max_x - min_x + 1) as usize;
        let height = (max_y - min_y + 1) as usize;

        let mut cells = vec![vec![Cell::default(); width]; height];

        for (sx, sy, cell) in drawn {
            let row = (sy - min_y) as usize;
            let col = (sx - min_x) as usize;
            if row < height && col < width {
                cells[row][col] = cell;
            }
        }

        Frame {
            width,
            height,
            cells,
            message: self.message.clone(),
        }
    }

    /// Move the player one step in `dir`. Returns the interaction result.
    pub fn move_player(&mut self, dir: Direction) -> Interaction {
        let (dx, dy) = dir.offset();
        let new_pos = Pos::new(self.player.pos.x + dx, self.player.pos.y + dy);

        if !self.world.is_walkable(new_pos) {
            return Interaction::Blocked {
                reason: "Blocked.".to_string(),
            };
        }

        self.player.pos = new_pos;
        self.turn = self.turn.saturating_add(1);
        self.update_npcs();
        self.message = "You move along the dusty trail.".to_string();
        // Play footstep sound
        #[cfg(feature = "sound")]
        self.sound_manager.play(SoundEffect::Footstep);
        Interaction::None
    }

    /// Interact with the tile or entity at the player's position.
    pub fn interact(&mut self) -> Interaction {
        // Check for NPC at player position.
        if let Some(npc) = self.world.npc_at(self.player.pos) {
            return match npc.kind {
                NpcKind::Deputy => Interaction::Talked {
                    npc: "Deputy".to_string(),
                    line: "Sheriff's orders: watch the perimeter.".to_string(),
                },
                NpcKind::Outlaw => {
                    self.message = "Outlaw captured! Town is safe.".to_string();
                    #[cfg(feature = "sound")]
                    self.sound_manager.play(SoundEffect::Victory);
                    self.game_over = true;
                    self.won = true;
                    Interaction::None
                },
                NpcKind::Sheriff => Interaction::Talked {
                    npc: "Sheriff".to_string(),
                    line: "Keep the peace, stranger.".to_string(),
                },
            };
        }

        // Check for item at player position.
        if let Some(item) = self.world.item_at(self.player.pos) {
            if let Some(idx) = self.world.items.iter().position(|i| i.pos == item.pos) {
                self.world.items[idx].collected = true;
                self.player.score = match item.kind {
                    ItemKind::Horse => {
                        self.player.mounted = true;
                        self.player.score.saturating_add(5)
                    },
                    ItemKind::Loot => self.player.score.saturating_add(10),
                };
                self.message = match item.kind {
                    ItemKind::Horse => {
                        #[cfg(feature = "sound")]
                        self.sound_manager.play(SoundEffect::HorseMount);
                        "You mounted a horse. The trail moves faster now.".to_string()
                    },
                    ItemKind::Loot => "You picked up a satchel of coins (+10).".to_string(),
                };
                #[cfg(feature = "sound")]
                match item.kind {
                    ItemKind::Horse => self.sound_manager.play(SoundEffect::HorseMount),
                    ItemKind::Loot => self.sound_manager.play(SoundEffect::LootPickup),
                };
                return Interaction::Collected { kind: item.kind };
            }
        }

        Interaction::None
    }