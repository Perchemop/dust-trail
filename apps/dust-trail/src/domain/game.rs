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
    default_world, spawn_deputies_for_wanted, ItemKind, Npc, NpcKind, Pos, WantedLevel, World,
};
use rand::rngs::StdRng;
use rand::{seq::SliceRandom, SeedableRng};

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
            message: "Welcome to Dust Trail. Use WASD to move, E to interact, S to save, L to load, Q to quit.".to_string(),
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
            message: "Welcome to Dust Trail. Use WASD to move, E to interact, S to save, L to load, Q to quit.".to_string(),
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
            message: "Welcome to Dust Trail. Use WASD to move, E to interact, S to save, L to load, Q to quit.".to_string(),
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
                NpcKind::Sheriff => Interaction::Talked {
                    npc: "Sheriff".to_string(),
                    line: "Keep the peace, stranger.".to_string(),
                },
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
                }
            };
        }

        // Check for item at player position.
        if let Some(item) = self.world.item_at(self.player.pos) {
            if let Some(idx) = self.world.items.iter().position(|i| i.pos == item.pos) {
                self.world.items[idx].collected = true;
                self.player.score = match item.kind {
                    ItemKind::Loot => self.player.score.saturating_add(10),
                    ItemKind::Horse => {
                        self.player.mounted = true;
                        self.player.score.saturating_add(5)
                    }
                };
                self.message = match item.kind {
                    ItemKind::Loot => "You picked up a satchel of coins (+10).".to_string(),
                    ItemKind::Horse => {
                        #[cfg(feature = "sound")]
                        self.sound_manager.play(SoundEffect::HorseMount);
                        "You mounted a horse. The trail moves faster now.".to_string()
                    }
                };
                #[cfg(feature = "sound")]
                match item.kind {
                    ItemKind::Loot => self.sound_manager.play(SoundEffect::LootPickup),
                    ItemKind::Horse => {}
                };
                return Interaction::Collected { kind: item.kind };
            }
        }

        Interaction::None
    }

    /// Update all NPCs (AI turns).
    fn update_npcs(&mut self) {
        // Update wanted level based on outlaw's bounty.
        let old_wanted = self
            .world
            .npcs
            .iter()
            .find(|n| n.kind == NpcKind::Outlaw)
            .map(|n| n.wanted_level())
            .unwrap_or(WantedLevel::None);

        // Check if outlaw's bounty changed (crimes committed).
        let new_wanted = self
            .world
            .npcs
            .iter()
            .find(|n| n.kind == NpcKind::Outlaw)
            .map(|n| n.wanted_level())
            .unwrap_or(WantedLevel::None);

        if new_wanted != old_wanted {
            #[cfg(feature = "sound")]
            match new_wanted {
                WantedLevel::Low | WantedLevel::Medium | WantedLevel::High => {
                    self.sound_manager.play(SoundEffect::GameOver);
                }
                _ => {}
            };
        }

        // Deputy spawning logic based on outlaw's wanted level.
        let outlaw_data = self
            .world
            .npcs
            .iter()
            .find(|n| n.kind == NpcKind::Outlaw)
            .map(|n| (n.pos, n.wanted_level()));
        if let Some((pos, wanted)) = outlaw_data {
            spawn_deputies_for_wanted(&mut self.world, pos, wanted);
        }

        // Move each NPC.
        let mut new_npcs = Vec::new();
        for mut npc in self.world.npcs.clone() {
            let action = match npc.kind {
                NpcKind::Sheriff => self.sheriff_ai(&npc),
                NpcKind::Deputy => self.deputy_ai(&npc),
                NpcKind::Outlaw => self.outlaw_ai(&npc),
            };

            match action {
                NpcAction::Move(pos) if self.world.is_walkable(pos) => {
                    npc.pos = pos;
                }
                NpcAction::Move(_) => {} // Invalid move, skip
                NpcAction::Wait => {}
                NpcAction::Attack(target) => {
                    if let Some(target_npc) = self.world.npcs.iter_mut().find(|n| n.pos == target) {
                        target_npc.bounty = target_npc.bounty.saturating_add(50);
                        self.message = format!("{:?} was wounded!", target_npc.kind);
                        #[cfg(feature = "sound")]
                        self.sound_manager.play(SoundEffect::Hit);
                    }
                }
            }
            new_npcs.push(npc);
        }
        self.world.npcs = new_npcs;
    }

    /// Sheriff AI: patrol town, pursue outlaw at high wanted.
    fn sheriff_ai(&mut self, npc: &Npc) -> NpcAction {
        let outlaw = self
            .world
            .npcs
            .iter()
            .find(|n| n.kind == NpcKind::Outlaw);

        if let Some(outlaw) = outlaw {
            let dist = (outlaw.pos.x - npc.pos.x).abs() + (outlaw.pos.y - npc.pos.y).abs();
            if dist <= 8 {
                // Chase outlaw.
                let dx = (outlaw.pos.x - npc.pos.x).signum();
                let dy = (outlaw.pos.y - npc.pos.y).signum();
                let target = Pos::new(npc.pos.x + dx, npc.pos.y + dy);
                if self.world.is_walkable(target) {
                    return NpcAction::Move(target);
                }
            }
        }
        self.patrol_town(npc.pos)
    }

    /// Deputy AI: flank and support sheriff.
    fn deputy_ai(&mut self, npc: &Npc) -> NpcAction {
        if let Some(sheriff) = self
            .world
            .npcs
            .iter()
            .find(|n| n.kind == NpcKind::Sheriff)
        {
            let dx = (sheriff.pos.x - npc.pos.x).signum();
            let dy = (sheriff.pos.y - npc.pos.y).signum();
            let target = Pos::new(npc.pos.x + dx, npc.pos.y + dy);
            if self.world.is_walkable(target) && target != sheriff.pos {
                return NpcAction::Move(target);
            }
        }
        self.patrol_town(npc.pos)
    }

    /// Outlaw AI: evade law, seek cover.
    fn outlaw_ai(&mut self, npc: &Npc) -> NpcAction {
        // Find nearest law (sheriff or deputy).
        let law = self
            .world
            .npcs
            .iter()
            .filter(|n| n.kind == NpcKind::Sheriff || n.kind == NpcKind::Deputy)
            .min_by_key(|n| {
                (n.pos.x - npc.pos.x).abs() + (n.pos.y - npc.pos.y).abs()
            });

        if let Some(law) = law {
            let dist = (law.pos.x - npc.pos.x).abs() + (law.pos.y - npc.pos.y).abs();
            if dist <= 5 {
                // Flee away from law.
                let dx = (npc.pos.x - law.pos.x).signum();
                let dy = (npc.pos.y - law.pos.y).signum();
                let target = Pos::new(npc.pos.x + dx, npc.pos.y + dy);
                if self.world.is_walkable(target) {
                    return NpcAction::Move(target);
                }
            }
        }
        // Random wander.
        self.random_wander(npc.pos)
    }

    /// Patrol around town center area.
    fn patrol_town(&mut self, pos: Pos) -> NpcAction {
        let center = Pos::new(self.world.width / 2, self.world.height / 2);
        let dx = (center.x - pos.x).signum();
        let dy = (center.y - pos.y).signum();
        let target = Pos::new(pos.x + dx, pos.y + dy);
        if self.world.is_walkable(target) {
            NpcAction::Move(target)
        } else {
            self.random_wander(pos)
        }
    }

    /// Random valid move.
    fn random_wander(&mut self, pos: Pos) -> NpcAction {
        let dirs = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];
        let mut dirs = dirs;
        dirs.shuffle(&mut self.rng);
        for dir in dirs {
            let (dx, dy) = dir.offset();
            let target = Pos::new(pos.x + dx, pos.y + dy);
            if self.world.is_walkable(target) {
                return NpcAction::Move(target);
            }
        }
        NpcAction::Wait
    }

    /// Save the game into the named slot of the default slot document.
    pub fn save_slot(&mut self, slot: &str) -> Result<String, String> {
        let path = save_dir();
        let store = SaveStore::new(path);
        let saved_as = store.save_data(slot, SaveData::with_slot(self, slot))?;
        self.message = format!("Game saved to slot \"{saved_as}\".");
        #[cfg(feature = "sound")]
        self.sound_manager.play(SoundEffect::Save);
        Ok(saved_as)
    }

    /// TUI-compatible save method (takes a path).
    pub fn save(&self, path: &Path) -> Result<(), String> {
        let store = SaveStore::new(path);
        store
            .save_data("default", SaveData::with_slot(self, "default"))
            .map(|_| ())
    }

    /// TUI-compatible load method.
    pub fn load(path: &Path) -> Result<Self, String> {
        let store = SaveStore::new(path);
        let mut game = Self::from_save_data(store.load_data("default")?);
        #[cfg(feature = "sound")]
        game.sound_manager.play(SoundEffect::Load);
        Ok(game)
    }

    /// Load the game from the named slot of the slot document at `path`.
    pub fn load_named_from(slot: &str, path: &Path) -> Result<Self, String> {
        let store = SaveStore::new(path);
        let mut game = Self::from_save_data(store.load_data(slot)?);
        #[cfg(feature = "sound")]
        game.sound_manager.play(SoundEffect::Load);
        Ok(game)
    }

    /// End-of-game flags are not part of a save, so a loaded game always
    /// resumes as playable.
    pub fn from_save_data(save: SaveData) -> Self {
        let mut game = Self {
            world: save.world,
            player: save.player,
            turn: save.turn,
            message: "Game loaded.".to_string(),
            game_over: false,
            won: false,
            rng: StdRng::from_os_rng(),
            #[cfg(feature = "sound")]
            sound_manager: SoundManager::new().unwrap_or_default(),
            #[cfg(not(feature = "sound"))]
            sound_settings: SoundSettings::new(),
        };
        // Restore sound settings from save data
        #[cfg(feature = "sound")]
        game.sound_manager.load_settings(save.sound_settings);
        #[cfg(not(feature = "sound"))]
        {
            game.sound_settings = save.sound_settings;
        }
        game
    }

    /// Names of the save slots in the default slot document, alphabetically.
    pub fn slot_names() -> Vec<String> {
        let path = save_dir();
        let store = SaveStore::new(path);
        store.list_saves_or_empty()
    }
}

/// NPC action during AI turn.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NpcAction {
    Move(Pos),
    Wait,
    Attack(Pos),
}

/// Resolve the save directory (config dir for dust-trail).
fn save_dir() -> std::path::PathBuf {
    directories::ProjectDirs::from("", "", "dust-trail")
        .map(|p| p.config_dir().to_path_buf())
        .unwrap_or_else(|| std::env::temp_dir().join("dust-trail"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::game::Direction;

    #[test]
    fn new_game_has_player_at_start() {
        let game = Game::new_static();
        assert_eq!(game.player.pos, Pos::new(2, 2));
        assert_eq!(game.turn, 0);
        assert!(!game.game_over);
        assert!(!game.won);
    }

    #[test]
    fn move_player_updates_position() {
        let mut game = Game::new_static();
        let start = game.player.pos;
        game.move_player(Direction::Right);
        assert_eq!(game.player.pos, Pos::new(start.x + 1, start.y));
        assert_eq!(game.turn, 1);
    }

    #[test]
    fn move_player_blocked_by_wall() {
        let mut game = Game::new_static();
        // Move to border (wall at x=0, y=0, etc.)
        for _ in 0..3 {
            game.move_player(Direction::Left);
        }
        let pos = game.player.pos;
        game.move_player(Direction::Left);
        assert_eq!(game.player.pos, pos); // Should not move
    }

    #[test]
    fn save_data_roundtrip() {
        let mut game = Game::new_static();
        game.move_player(Direction::Right);
        game.move_player(Direction::Down);
        let data = SaveData::with_slot(&game, "test");
        let restored = data.into_game();
        assert_eq!(restored.player.pos, game.player.pos);
        assert_eq!(restored.turn, game.turn);
    }

    #[test]
    fn outlaw_capture_ends_game() {
        let mut game = Game::new_static();
        // Manually place outlaw at player position
        game.world.npcs.push(crate::domain::world::Npc {
            kind: NpcKind::Outlaw,
            pos: game.player.pos,
            bounty: 0,
        });
        game.interact();
        assert!(game.game_over);
        assert!(game.won);
    }

    #[test]
    fn wanted_level_escalates() {
        let mut game = Game::new_static();
        let outlaw_idx = game
            .world
            .npcs
            .iter()
            .position(|n| n.kind == NpcKind::Outlaw)
            .unwrap();
        game.world.npcs[outlaw_idx].bounty = 0;
        assert_eq!(
            game.world.npcs[outlaw_idx].wanted_level(),
            WantedLevel::None
        );
        // Commit crime to increase wanted
        game.world.npcs[outlaw_idx].add_bounty(20);
        assert_eq!(
            game.world.npcs[outlaw_idx].wanted_level(),
            WantedLevel::Low
        );
    }

    #[test]
    fn deputy_spawns_at_medium_wanted() {
        let mut game = Game::new_static();
        // Force medium wanted level
        let outlaw_idx = game
            .world
            .npcs
            .iter()
            .position(|n| n.kind == NpcKind::Outlaw)
            .unwrap();
        game.world.npcs[outlaw_idx].bounty = 50;
        game.update_npcs();
        let has_deputy = game.world.npcs.iter().any(|n| n.kind == NpcKind::Deputy);
        assert!(has_deputy);
    }
}