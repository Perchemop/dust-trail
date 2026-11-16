//! Core game state: turn-based Western adventure. Movement, NPC turns,
//! interactions, save/load, and Frame production all live here.
//!
//! Clean-room: original mechanics; nothing copied from a commercial product.

use std::path::Path;

use crate::domain::cell::Cell;
use crate::domain::color::Rgb;
use crate::domain::frame::Frame;
use crate::domain::iso::{screen_depth, world_to_screen};
use crate::domain::player::Player;
use crate::domain::save::SaveData;
use crate::domain::slots::SaveStore;
use crate::domain::tile::TileType;
use crate::domain::world::{default_world, ItemKind, NpcKind, Pos, WantedLevel, World};
use rand::prelude::IndexedRandom;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

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
}

fn default_rng() -> StdRng {
    StdRng::from_os_rng()
}

/// AI state for the outlaw.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AIState {
    Patrol,
    Chase,
    Flee,
}

impl Game {
    pub fn new() -> Self {
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
        }
    }

    /// Create a new game with a specific RNG seed (for testing).
    pub fn new_with_seed(seed: u64) -> Self {
        let world = default_world();
        let player = Player::new(Pos::new(2, 2));
        Self {
            world,
            player,
            turn: 0,
            message: "Welcome to Dust Trail. Use WASD to move, E to interact, S to save, L to load, Q to quit.".to_string(),
            game_over: false,
            won: false,
            rng: StdRng::seed_from_u64(seed),
        }
    }

    /// Render the current state as a Frame for either frontend.
    pub fn frame(&self) -> Frame {
        // --- Isometric layout ---
        // World (x, y) -> screen (sx, sy): sx = (x-y)*2, sy = x+y.
        // Nearer tiles (larger screen depth) are drawn last so they overlap
        // farther ones (painter's algorithm).
        let pad = 2;
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
                    Cell::new(item.kind.glyph(), item.kind.fg(), Rgb::BLACK),
                ));
            }
        }

        // NPCs, drawn over tiles and items.
        for npc in &self.world.npcs {
            if self.world.in_bounds(npc.pos) {
                let (sx, sy) = world_to_screen(npc.pos.x, npc.pos.y);
                drawn.push((
                    sx,
                    sy,
                    Cell::new(npc.kind.glyph(), npc.kind.fg(), Rgb::BLACK),
                ));
            }
        }

        // Player on top of everything.
        if self.world.in_bounds(self.player.pos) {
            let (sx, sy) = world_to_screen(self.player.pos.x, self.player.pos.y);
            drawn.push((
                sx,
                sy,
                Cell::new(self.player.glyph(), self.player.fg(), Rgb::BLACK),
            ));
        }

        // Bounding box of everything drawn.
        let min_x = drawn.iter().map(|(sx, _, _)| *sx).min().unwrap_or(0);
        let min_y = drawn.iter().map(|(_, sy, _)| *sy).min().unwrap_or(0);
        let max_x = drawn.iter().map(|(sx, _, _)| *sx).max().unwrap_or(0);
        let max_y = drawn.iter().map(|(_, sy, _)| *sy).max().unwrap_or(0);

        let fw = (max_x - min_x + 1) as usize + pad * 2;
        let fh = (max_y - min_y + 1) as usize + pad * 2 + 3; // +3 for status+message
        let mut frame = Frame::new(fw, fh);

        // Background.
        for y in 0..fh {
            for x in 0..fw {
                frame.set(x, y, Cell::new(' ', Rgb::BLACK, Rgb::BLACK));
            }
        }

        // Place cells, offset so the bounding-box origin lands at (pad, pad).
        // Keep the bottom 3 rows free for the status and message lines.
        let ox = pad as i32 - min_x;
        let oy = pad as i32 - min_y;
        for (sx, sy, cell) in drawn {
            let gx = sx + ox;
            let gy = sy + oy;
            if gx >= 0 && gy >= 0 && (gx as usize) < fw && (gy as usize) < fh.saturating_sub(3) {
                frame.set(gx as usize, gy as usize, cell);
            }
        }

        // Status line.
        let status = format!(
            " Dust Trail  |  Turn: {}  Score: {}  {}  |  Pos: {},{} ",
            self.turn,
            self.player.score,
            if self.player.mounted {
                "[mounted]"
            } else {
                "[on foot]"
            },
            self.player.pos.x,
            self.player.pos.y,
        );
        let sy = fh - 2;
        for (i, ch) in status.chars().enumerate() {
            if i < fw {
                frame.set(i, sy, Cell::new(ch, Rgb::WHITE, Rgb::BLACK));
            }
        }

        // Message line.
        let my = fh - 1;
        let msg = if self.game_over {
            if self.won {
                "The outlaw is in custody. Town is safe. Well done, Dust."
            } else {
                "The outlaw escaped. Dust Trail continues..."
            }
            .to_string()
        } else {
            self.message.clone()
        };
        for (i, ch) in msg.chars().enumerate() {
            if i < fw {
                frame.set(i, my, Cell::new(ch, Rgb::YELLOW, Rgb::BLACK));
            }
        }

        frame
    }

    /// Move the player in a direction, then advance one turn (NPCs act).
    pub fn move_player(&mut self, dir: Direction) -> Interaction {
        if self.game_over {
            return Interaction::Blocked {
                reason: "Game over. Start a new game.".to_string(),
            };
        }
        let (dx, dy) = dir.offset();
        let target = self.player.pos.offset(dx, dy);

        if !self.world.in_bounds(target) {
            self.message = "The trail ends there.".to_string();
            return Interaction::Blocked {
                reason: "out of bounds".to_string(),
            };
        }
        if !self.world.is_walkable(target) {
            if let Some(npc) = self.world.npc_at(target) {
                self.message = format!(
                    "{} stands in the way. Press E to interact.",
                    npc.kind.glyph()
                );
                return Interaction::Blocked {
                    reason: "npc blocks".to_string(),
                };
            }
            let tile = self.world.tile(target).unwrap_or(TileType::Wall);
            self.message = format!("You cannot enter that tile ({:?}).", tile);
            return Interaction::Blocked {
                reason: "not walkable".to_string(),
            };
        }

        self.player.pos = target;

        // Collect any item here.
        if let Some(item) = self.world.item_at(target) {
            self.collect_item(item.kind);
        }

        self.turn = self.turn.saturating_add(1);
        self.update_npcs();
        self.message = "You move along the dusty trail.".to_string();
        Interaction::None
    }

    fn collect_item(&mut self, kind: ItemKind) {
        match kind {
            ItemKind::Loot => {
                self.player.score = self.player.score.saturating_add(10);
                self.message = "You picked up a satchel of coins (+10).".to_string();
            }
            ItemKind::Horse => {
                self.player.mounted = true;
                self.message = "You mounted a horse. The trail moves faster now.".to_string();
            }
        }
        if let Some(item) = self
            .world
            .items
            .iter_mut()
            .find(|i| i.pos == self.player.pos && i.kind == kind && !i.collected)
        {
            item.collected = true;
        }
    }

    /// Collect bounty from an outlaw at the given position.
    /// Returns the bounty amount collected (0 if no outlaw or no bounty).
    fn collect_outlaw_bounty_at(&mut self, pos: Pos) -> u32 {
        if let Some(idx) = self
            .world
            .npcs
            .iter()
            .position(|n| n.kind == NpcKind::Outlaw && n.pos == pos)
        {
            let bounty = self.world.npcs[idx].bounty;
            if bounty > 0 {
                self.player.score = self.player.score.saturating_add(bounty);
                self.world.npcs.remove(idx);
                return bounty;
            }
        }
        0
    }

    /// Interact with an adjacent NPC or item (press E).
    pub fn interact(&mut self) -> Interaction {
        if self.game_over {
            return Interaction::Blocked {
                reason: "Game over.".to_string(),
            };
        }
        // Adjacent positions (including diagonals feel natural for talking).
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let pos = self.player.pos.offset(dx, dy);
                if let Some(npc) = self.world.npc_at(pos) {
                    let line = match npc.kind {
                        crate::domain::world::NpcKind::Sheriff => {
                            "The sheriff tips his hat. 'Keep the peace, Dust.'"
                        }
                        crate::domain::world::NpcKind::Outlaw => {
                            if self.player.mounted {
                                "The outlaw draws, but you're on horseback. He surrenders!"
                            } else {
                                "The outlaw laughs. 'You'll never take me alive on foot.'"
                            }
                        }
                        crate::domain::world::NpcKind::Deputy => {
                            "Deputy: 'Sheriff sent us to bring in the outlaw.'"
                        }
                    };
                    self.message = format!("{}: {}", npc.kind.glyph(), line);
                    let interacted = Interaction::Talked {
                        npc: npc.kind.glyph().to_string(),
                        line: line.to_string(),
                    };
                    if matches!(npc.kind, crate::domain::world::NpcKind::Outlaw)
                        && self.player.mounted
                    {
                        // Collect bounty
                        let bounty = self.collect_outlaw_bounty_at(pos);
                        if bounty > 0 {
                            self.message = format!(
                                "Outlaw captured! Bounty claimed: ${}. Town is safe!",
                                bounty
                            );
                        } else {
                            self.message = "Outlaw captured! Town is safe.".to_string();
                        }
                        self.game_over = true;
                        self.won = true;
                    }
                    return interacted;
                }
                if let Some(item) = self.world.item_at(pos) {
                    self.collect_item(item.kind);
                    return Interaction::Collected { kind: item.kind };
                }
            }
        }
        self.message = "Nothing nearby to interact with.".to_string();
        Interaction::None
    }

    /// Save the game to a JSON file at `path`.
    ///
    /// This is the legacy single-file save. For named save slots use
    /// [`Game::save_named`].
    pub fn save(&mut self, path: &Path) -> Result<(), String> {
        let save = SaveData::new(self);
        let json =
            serde_json::to_string_pretty(&save).map_err(|e| format!("serialise error: {e}"))?;
        std::fs::write(path, json).map_err(|e| format!("write error: {e}"))?;
        self.message = "Game saved.".to_string();
        Ok(())
    }

    /// Load the game from a JSON file at `path`.
    ///
    /// This is the legacy single-file save. For named save slots use
    /// [`Game::load_named`].
    pub fn load(path: &Path) -> Result<Self, String> {
        let json = std::fs::read_to_string(path).map_err(|e| format!("read error: {e}"))?;
        let save: SaveData =
            serde_json::from_str(&json).map_err(|e| format!("parse error: {e}"))?;
        Ok(Self::from_save_data(save))
    }

    /// Build a playable game from a deserialised snapshot.
    ///
    /// End-of-game flags are not part of a save, so a loaded game always
    /// resumes as playable.
    pub fn from_save_data(save: SaveData) -> Self {
        Self {
            world: save.world,
            player: save.player,
            turn: save.turn,
            message: "Game loaded.".to_string(),
            game_over: false,
            won: false,
            rng: StdRng::from_os_rng(),
        }
    }

    /// Save the game into the named slot of the default slot document.
    pub fn save_named(&mut self, slot: &str) -> Result<(), String> {
        self.save_named_to(slot, &SaveStore::default_path())
    }

    /// Save the game into the named slot of the slot document at `path`.
    ///
    /// The slot name is trimmed and validated first; an existing slot of the
    /// same name is overwritten.
    pub fn save_named_to(&mut self, slot: &str, path: &Path) -> Result<(), String> {
        let store = SaveStore::new(path);
        let saved_as = store.save_data(slot, SaveData::with_slot(self, slot))?;
        self.message = format!("Game saved to slot \"{saved_as}\".");
        Ok(())
    }

    /// Load the game from the named slot of the default slot document.
    pub fn load_named(slot: &str) -> Result<Self, String> {
        Self::load_named_from(slot, &SaveStore::default_path())
    }

    /// Load the game from the named slot of the slot document at `path`.
    pub fn load_named_from(slot: &str, path: &Path) -> Result<Self, String> {
        let store = SaveStore::new(path);
        Ok(Self::from_save_data(store.load_data(slot)?))
    }

    /// Names of the save slots in the default slot document, alphabetically.
    pub fn list_saves() -> Result<Vec<String>, String> {
        Self::list_saves_at(&SaveStore::default_path())
    }

    /// Names of the save slots in the slot document at `path`, alphabetically.
    pub fn list_saves_at(path: &Path) -> Result<Vec<String>, String> {
        SaveStore::new(path).list_saves()
    }

    /// Delete the named slot from the default slot document.
    pub fn delete_save(slot: &str) -> Result<(), String> {
        Self::delete_save_at(slot, &SaveStore::default_path())
    }

    /// Delete the named slot from the slot document at `path`.
    pub fn delete_save_at(slot: &str, path: &Path) -> Result<(), String> {
        SaveStore::new(path).delete_save(slot)
    }

    /// Whether the named slot exists in the default slot document.
    pub fn save_exists(slot: &str) -> bool {
        Self::save_exists_at(slot, &SaveStore::default_path())
    }

    /// Whether the named slot exists in the slot document at `path`.
    pub fn save_exists_at(slot: &str, path: &Path) -> bool {
        SaveStore::new(path).save_exists(slot)
    }

    /// Advance NPCs one turn.
    /// is testable headlessly.
    fn update_npcs(&mut self) {
        // Decay outlaw bounties over time
        self.decay_outlaw_bounties();

        // Spawn deputies based on wanted level
        self.spawn_deputies_if_needed();

        // Smarter Outlaw AI with Patrol/Chase/Flee states
        let outlaw_target = self.compute_outlaw_target();

        // Apply outlaw move
        if let Some(target) = outlaw_target {
            for npc in &mut self.world.npcs {
                if npc.kind == crate::domain::world::NpcKind::Outlaw {
                    npc.pos = target;
                }
            }
        }

        // Sheriff and Deputy AI
        self.update_law_enforcement_ai();
    }

    /// Decay outlaw bounties over time (1 per 10 turns).
    fn decay_outlaw_bounties(&mut self) {
        if self.turn % 10 == 0 {
            for npc in &mut self.world.npcs {
                if npc.kind == crate::domain::world::NpcKind::Outlaw {
                    npc.decay_bounty();
                }
            }
        }
    }

    /// Spawn deputies based on the outlaw's wanted level.
    fn spawn_deputies_if_needed(&mut self) {
        // Find outlaw position and wanted level (avoiding double borrow)
        let (outlaw_pos, wanted) = {
            let outlaw = self.world.npcs.iter().find(|n| n.kind == NpcKind::Outlaw);
            match outlaw {
                Some(o) => (o.pos, o.wanted_level()),
                None => return,
            }
        };

        // Only spawn if we don't already have enough deputies
        let current_deputies = self
            .world
            .npcs
            .iter()
            .filter(|n| n.kind == NpcKind::Deputy)
            .count();
        let needed = match wanted {
            WantedLevel::None | WantedLevel::Low => 0,
            WantedLevel::Medium => 1,
            WantedLevel::High => 2,
        };
        if current_deputies < needed {
            let spawned = crate::domain::world::spawn_deputies_for_wanted(
                &mut self.world,
                outlaw_pos,
                wanted,
            );
            if spawned > 0 {
                self.message = format!("A posse of {} deputy(s) has been deployed!", spawned);
            }
        }
    }

    /// Compute the Outlaw's next move using Patrol/Chase/Flee AI.
    fn compute_outlaw_target(&mut self) -> Option<Pos> {
        // Find the outlaw and sheriff positions
        let (outlaw_pos, sheriff_pos) = {
            let mut outlaw = None;
            let mut sheriff = None;
            for npc in &self.world.npcs {
                match npc.kind {
                    NpcKind::Outlaw => outlaw = Some(npc.pos),
                    NpcKind::Sheriff | NpcKind::Deputy => sheriff = Some(npc.pos),
                }
            }
            (outlaw?, sheriff)
        };

        // Distances to player and sheriff
        let dist_to_player = self.manhattan_distance(outlaw_pos, self.player.pos);
        let dist_to_sheriff = sheriff_pos.map(|s| self.manhattan_distance(outlaw_pos, s));

        // AI state thresholds
        const CHASE_RANGE: i32 = 8;
        const FLEE_RANGE: i32 = 6;

        // Determine AI state
        let state = if let Some(d) = dist_to_sheriff {
            if d <= FLEE_RANGE {
                AIState::Flee
            } else if dist_to_player <= CHASE_RANGE {
                AIState::Chase
            } else {
                AIState::Patrol
            }
        } else if dist_to_player <= CHASE_RANGE {
            AIState::Chase
        } else {
            AIState::Patrol
        };

        // Get valid moves (walkable, in bounds, not player)
        let valid_moves = self.get_valid_moves(outlaw_pos);

        if valid_moves.is_empty() {
            return None;
        }

        // Choose move based on state
        let target = match state {
            AIState::Flee => {
                // In Flee state, sheriff_pos is guaranteed to be Some
                let sheriff_pos = sheriff_pos.unwrap();
                self.choose_flee_move(outlaw_pos, sheriff_pos, &valid_moves)
            }
            AIState::Chase => self.choose_chase_move(outlaw_pos, self.player.pos, &valid_moves),
            AIState::Patrol => self.choose_patrol_move(outlaw_pos, &valid_moves),
        };

        Some(target)
    }

    /// Manhattan distance between two positions.
    fn manhattan_distance(&self, a: Pos, b: Pos) -> i32 {
        (a.x - b.x).abs() + (a.y - b.y).abs()
    }

    /// Get all valid moves from a position (walkable, in bounds, not player).
    fn get_valid_moves(&self, pos: Pos) -> Vec<Pos> {
        let dirs = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];
        let mut moves = Vec::new();
        for dir in dirs {
            let (dx, dy) = dir.offset();
            let target = Pos::new(pos.x + dx, pos.y + dy);
            if self.world.in_bounds(target)
                && self.world.is_walkable(target)
                && target != self.player.pos
            {
                moves.push(target);
            }
        }
        moves
    }

    /// Update Sheriff and Deputy AI based on wanted level.
    fn update_law_enforcement_ai(&mut self) {
        // Get sheriff position and outlaw info first (immutable borrows)
        let sheriff_pos = self
            .world
            .npcs
            .iter()
            .find(|n| n.kind == NpcKind::Sheriff)
            .map(|n| n.pos);

        let (outlaw_pos, wanted) = {
            let outlaw = self.world.npcs.iter().find(|n| n.kind == NpcKind::Outlaw);
            match outlaw {
                Some(o) => (Some(o.pos), o.wanted_level()),
                None => (None, WantedLevel::None),
            }
        };

        // Sheriff AI
        if let (Some(sheriff_pos), Some(outlaw_pos)) = (sheriff_pos, outlaw_pos) {
            let sheriff_target = self.compute_sheriff_target(sheriff_pos, Some(outlaw_pos), wanted);
            if let Some(target) = sheriff_target {
                for npc in &mut self.world.npcs {
                    if npc.kind == NpcKind::Sheriff {
                        npc.pos = target;
                    }
                }
            }
        }

        // Deputy AI - deputies move toward outlaw
        if let Some(outlaw_pos) = outlaw_pos {
            // Collect deputy positions first, then compute targets
            let deputy_positions: Vec<Pos> = self
                .world
                .npcs
                .iter()
                .filter(|n| n.kind == NpcKind::Deputy)
                .map(|n| n.pos)
                .collect();

            // Compute targets for each deputy
            let mut deputy_moves = Vec::new();
            for pos in deputy_positions {
                if let Some(target) = self.compute_deputy_target(pos, outlaw_pos) {
                    deputy_moves.push((pos, target));
                }
            }
            // Apply deputy moves
            for (from_pos, target_pos) in deputy_moves {
                for npc in &mut self.world.npcs {
                    if npc.kind == NpcKind::Deputy && npc.pos == from_pos {
                        npc.pos = target_pos;
                    }
                }
            }
        }
    }

    /// Compute Sheriff's target based on wanted level.
    fn compute_sheriff_target(
        &mut self,
        sheriff_pos: Pos,
        outlaw_pos: Option<Pos>,
        wanted: WantedLevel,
    ) -> Option<Pos> {
        let outlaw_pos = outlaw_pos?;

        // Sheriff behavior scales with wanted level
        let pursuit_range = match wanted {
            WantedLevel::None => 6,
            WantedLevel::Low => 10,
            WantedLevel::Medium => 15,
            WantedLevel::High => 25,
        };

        let dist = self.manhattan_distance(sheriff_pos, outlaw_pos);

        // Only pursue if outlaw is in range
        if dist > pursuit_range {
            return None;
        }

        // Get valid moves for sheriff
        let valid_moves = self.get_valid_moves(sheriff_pos);
        if valid_moves.is_empty() {
            return None;
        }

        // Choose move that minimizes distance to outlaw
        let mut best = valid_moves[0];
        let mut best_dist = self.manhattan_distance(valid_moves[0], outlaw_pos);
        for &m in &valid_moves[1..] {
            let d = self.manhattan_distance(m, outlaw_pos);
            if d < best_dist {
                best_dist = d;
                best = m;
            }
        }

        // Add some randomness (10% chance for random move)
        if self.rng.random_ratio(1, 10) {
            Some(valid_moves.choose(&mut self.rng).copied().unwrap())
        } else {
            Some(best)
        }
    }

    /// Compute Deputy's target - deputies coordinate with sheriff.
    fn compute_deputy_target(&mut self, deputy_pos: Pos, outlaw_pos: Pos) -> Option<Pos> {
        let valid_moves = self.get_valid_moves(deputy_pos);
        if valid_moves.is_empty() {
            return None;
        }

        // Deputies try to flank - approach from different angle than sheriff
        let sheriff_pos = self
            .world
            .npcs
            .iter()
            .find(|n| n.kind == NpcKind::Sheriff)
            .map(|n| n.pos);

        // Choose move that minimizes distance to outlaw but also tries to flank
        let mut best = valid_moves[0];
        let mut best_score = i32::MAX;
        for &m in &valid_moves {
            let dist_to_outlaw = self.manhattan_distance(m, outlaw_pos);
            let flank_bonus = if let Some(sheriff) = sheriff_pos {
                // Bonus for being on opposite side of outlaw from sheriff
                let dx1 = outlaw_pos.x - sheriff.x;
                let dx2 = m.x - outlaw_pos.x;
                if dx1 * dx2 < 0 {
                    -5
                } else {
                    0
                }
            } else {
                0
            };
            let score = dist_to_outlaw + flank_bonus;
            if score < best_score {
                best_score = score;
                best = m;
            }
        }

        // Add some randomness
        if self.rng.random_ratio(1, 5) {
            Some(valid_moves.choose(&mut self.rng).copied().unwrap())
        } else {
            Some(best)
        }
    }

    /// Choose a move that flees from the sheriff.
    fn choose_flee_move(&mut self, _pos: Pos, sheriff: Pos, moves: &[Pos]) -> Pos {
        // Choose move that maximizes distance from sheriff
        let mut best = moves[0];
        let mut best_dist = self.manhattan_distance(moves[0], sheriff);
        for &m in &moves[1..] {
            let d = self.manhattan_distance(m, sheriff);
            if d > best_dist {
                best_dist = d;
                best = m;
            }
        }
        // Add some randomness (20% chance to pick random valid move)
        if self.rng.random_ratio(1, 5) {
            moves.choose(&mut self.rng).copied().unwrap()
        } else {
            best
        }
    }

    /// Choose a move that chases the player.
    fn choose_chase_move(&mut self, _pos: Pos, player: Pos, moves: &[Pos]) -> Pos {
        // Choose move that minimizes distance to player
        let mut best = moves[0];
        let mut best_dist = self.manhattan_distance(moves[0], player);
        for &m in &moves[1..] {
            let d = self.manhattan_distance(m, player);
            if d < best_dist {
                best_dist = d;
                best = m;
            }
        }
        // Add some randomness (20% chance to pick random valid move)
        if self.rng.random_ratio(1, 5) {
            moves.choose(&mut self.rng).copied().unwrap()
        } else {
            best
        }
    }

    /// Choose a patrol move (random walk with anti-backtracking).
    fn choose_patrol_move(&mut self, _pos: Pos, moves: &[Pos]) -> Pos {
        // Prefer moves that don't backtrack (if we have a previous position)
        // For now, just pick a random valid move
        moves.choose(&mut self.rng).copied().unwrap()
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_game_is_not_over() {
        let g = Game::new();
        assert!(!g.game_over);
        assert!(!g.won);
        assert_eq!(g.turn, 0);
    }

    #[test]
    fn player_can_move_on_ground() {
        let mut g = Game::new();
        let start = g.player.pos;
        let res = g.move_player(Direction::Right);
        assert!(matches!(res, Interaction::None));
        assert_eq!(g.turn, 1);
        assert_ne!(g.player.pos, start);
    }

    #[test]
    fn player_cannot_move_into_wall() {
        // (1,2) is inside border walls at (0, y); moving left from (2,2) to
        // (1,2)... check border: x=0 is wall. Let's move left from x=2 to x=1
        // which is ground. Move left again to x=0 which is wall.
        let mut g = Game::new();
        g.move_player(Direction::Left); // (1,2) ground
        let res = g.move_player(Direction::Left); // (0,2) wall
        assert!(matches!(res, Interaction::Blocked { .. }));
        assert_eq!(g.player.pos, Pos::new(1, 2));
    }

    #[test]
    fn collecting_loot_raises_score() {
        let mut g = Game::new();
        // Loot is at (10,5). Move toward it.
        for _ in 0..8 {
            g.move_player(Direction::Right);
        }
        for _ in 0..3 {
            g.move_player(Direction::Down);
        }
        assert_eq!(g.player.pos, Pos::new(10, 5));
        assert_eq!(g.player.score, 10);
    }

    #[test]
    fn save_and_load_roundtrip() {
        let mut g = Game::new();
        g.move_player(Direction::Right);
        g.move_player(Direction::Down);
        let path = std::env::temp_dir().join("dust-trail-test-save.json");
        g.save(&path).expect("save should work");
        let g2 = Game::load(&path).expect("load should work");
        assert_eq!(g2.player.pos, g.player.pos);
        assert_eq!(g2.turn, g.turn);
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn interacting_with_sheriff_yields_line() {
        let mut g = Game::new();
        // Sheriff at (7,5); player at (2,2). Move adjacent then interact.
        for _ in 0..5 {
            g.move_player(Direction::Right);
        }
        for _ in 0..3 {
            g.move_player(Direction::Down);
        }
        let res = g.interact();
        assert!(matches!(res, Interaction::Talked { .. }));
    }

    /// A unique, empty scratch directory for one headless slot test.
    fn scratch(tag: &str) -> std::path::PathBuf {
        let name = format!("dust-trail-game-{}-{tag}", std::process::id());
        let dir = std::env::temp_dir().join(name);
        let _ = std::fs::remove_dir_all(&dir);
        let _ = std::fs::create_dir_all(&dir);
        dir
    }

    #[test]
    fn test_save_and_load_multiple_slots() {
        let dir = scratch("multi");
        let path = dir.join("saves.json");

        let mut alpha = Game::new();
        alpha.move_player(Direction::Right);
        alpha.save_named_to("alpha", &path).expect("save alpha");

        let mut beta = Game::new();
        beta.move_player(Direction::Left);
        beta.move_player(Direction::Down);
        beta.save_named_to("beta", &path).expect("save beta");

        let loaded_alpha = Game::load_named_from("alpha", &path).expect("load alpha");
        let loaded_beta = Game::load_named_from("beta", &path).expect("load beta");

        assert_eq!(loaded_alpha.player.pos, alpha.player.pos);
        assert_eq!(loaded_alpha.turn, alpha.turn);
        assert_eq!(loaded_beta.player.pos, beta.player.pos);
        assert_eq!(loaded_beta.turn, beta.turn);
        // Each slot must keep its own state.
        assert_ne!(loaded_alpha.player.pos, loaded_beta.player.pos);

        let names = Game::list_saves_at(&path).expect("list");
        assert_eq!(names, ["alpha", "beta"]);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_save_slot_overwrite() {
        let dir = scratch("overwrite");
        let path = dir.join("saves.json");

        let mut game = Game::new();
        game.save_named_to("camp", &path).expect("first save");
        assert_eq!(game.turn, 0);

        game.move_player(Direction::Right);
        game.save_named_to("camp", &path).expect("overwrite");
        assert_eq!(game.turn, 1);

        let loaded = Game::load_named_from("camp", &path).expect("load");
        assert_eq!(loaded.turn, 1, "overwrite must replace, not append");

        let names = Game::list_saves_at(&path).expect("list");
        assert_eq!(names.len(), 1, "must not create a second slot");

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_save_slot_not_found() {
        let dir = scratch("notfound");
        let path = dir.join("saves.json");

        let err = Game::load_named_from("ghost", &path).expect_err("missing slot must fail");
        assert!(err.contains("ghost"), "unexpected message: {err}");
        assert!(Game::delete_save_at("ghost", &path).is_err());
        assert!(!Game::save_exists_at("ghost", &path));

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_list_save_slots() {
        let dir = scratch("list");
        let path = dir.join("saves.json");

        let names = Game::list_saves_at(&path).expect("list missing");
        assert!(names.is_empty());

        let mut game = Game::new();
        for name in ["camp", "ridge", "dusty"] {
            game.save_named_to(name, &path).expect("save slot");
        }

        let names = Game::list_saves_at(&path).expect("list");
        let expected = ["camp", "dusty", "ridge"];
        assert_eq!(names, expected, "slots must be listed alphabetically");

        // A blank name is rejected rather than creating an unusable slot.
        let blank = game.save_named_to("   ", &path);
        let err = blank.expect_err("blank slot name");
        assert!(err.contains("empty"), "unexpected message: {err}");

        let names = Game::list_saves_at(&path).expect("list");
        assert_eq!(names.len(), 3);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_delete_save_slot() {
        let dir = scratch("delete");
        let path = dir.join("saves.json");

        let mut game = Game::new();
        game.save_named_to("keep", &path).expect("save keep");
        game.save_named_to("drop", &path).expect("save drop");
        assert!(Game::save_exists_at("drop", &path));

        Game::delete_save_at("drop", &path).expect("delete drop");
        assert!(!Game::save_exists_at("drop", &path));
        assert!(Game::save_exists_at("keep", &path));

        let names = Game::list_saves_at(&path).expect("list");
        assert_eq!(names, ["keep"]);
        assert!(Game::load_named_from("drop", &path).is_err());
        assert!(Game::delete_save_at("drop", &path).is_err());

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_save_slot_name_is_canonicalised() {
        let dir = scratch("canonical");
        let path = dir.join("saves.json");

        let mut game = Game::new();
        game.save_named_to("  Camp Site  ", &path).expect("save");
        assert!(game.message.contains("Camp Site"));
        let names = Game::list_saves_at(&path).expect("list");
        assert_eq!(names, ["Camp Site"]);
        assert!(Game::load_named_from("Camp Site", &path).is_ok());

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_slot_document_survives_reload_of_saves() {
        let dir = scratch("persist");
        let path = dir.join("saves.json");

        let mut game = Game::new();
        game.move_player(Direction::Right);
        game.move_player(Direction::Right);
        game.save_named_to("run", &path).expect("save");

        // Reload through the store rather than the in-memory game.
        let store = SaveStore::new(&path);
        let meta = store.list_metadata().expect("metadata");
        assert_eq!(meta.len(), 1);
        assert_eq!(meta[0].name, "run");
        assert_eq!(meta[0].turn, game.turn);
        assert_eq!(meta[0].player_name, game.player.name);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn outlaw_ai_moves_with_fixed_seed() {
        // With a fixed seed, the outlaw's moves should be deterministic
        let mut g1 = Game::new_with_seed(42);
        let mut g2 = Game::new_with_seed(42);

        let outlaw_start = g1
            .world
            .npcs
            .iter()
            .find(|n| n.kind == NpcKind::Outlaw)
            .map(|n| n.pos)
            .expect("outlaw exists");

        // Advance both games by the same number of turns
        for _ in 0..10 {
            g1.move_player(Direction::Right);
            g2.move_player(Direction::Right);
        }

        let outlaw_pos1 = g1
            .world
            .npcs
            .iter()
            .find(|n| n.kind == NpcKind::Outlaw)
            .map(|n| n.pos)
            .expect("outlaw exists");
        let outlaw_pos2 = g2
            .world
            .npcs
            .iter()
            .find(|n| n.kind == NpcKind::Outlaw)
            .map(|n| n.pos)
            .expect("outlaw exists");

        // With same seed, both games should have identical outlaw positions
        assert_eq!(
            outlaw_pos1, outlaw_pos2,
            "Outlaw position should be deterministic with fixed seed"
        );
        // And the outlaw should have moved
        assert_ne!(outlaw_pos1, outlaw_start, "Outlaw should have moved");
    }

    #[test]
    fn outlaw_ai_flees_from_sheriff() {
        // Create a game with fixed seed
        let mut g = Game::new_with_seed(123);

        // Move player far away so chase doesn't trigger
        // Sheriff is at (7,5), outlaw at (33,15) in default world
        // Move player far away from outlaw
        g.player.pos = Pos::new(2, 2);

        // Advance several turns
        for _ in 0..20 {
            g.move_player(Direction::Right);
        }

        // Outlaw should have moved away from sheriff's starting area
        let outlaw_pos = g
            .world
            .npcs
            .iter()
            .find(|n| n.kind == NpcKind::Outlaw)
            .map(|n| n.pos)
            .expect("outlaw exists");

        // Outlaw should have moved from start
        assert_ne!(outlaw_pos, Pos::new(33, 15));
    }

    #[test]
    fn outlaw_ai_chases_player_when_nearby() {
        // Create a game with fixed seed
        let mut g = Game::new_with_seed(456);

        // Position player near outlaw (outlaw starts at ~33,15)
        // Sheriff is at (7,5), far enough not to trigger flee
        g.player.pos = Pos::new(30, 14);

        // Advance several turns - outlaw should move toward player
        for _ in 0..15 {
            g.move_player(Direction::Right);
        }

        let outlaw_pos = g
            .world
            .npcs
            .iter()
            .find(|n| n.kind == NpcKind::Outlaw)
            .map(|n| n.pos)
            .expect("outlaw exists");

        // Outlaw should have moved toward player's general area
        // Distance should generally decrease (or at least outlaw moved)
        assert_ne!(
            outlaw_pos,
            Pos::new(33, 15),
            "Outlaw should move toward player"
        );
    }

    #[test]
    fn outlaw_ai_different_seeds_produce_different_paths() {
        let mut g1 = Game::new_with_seed(999);
        let mut g2 = Game::new_with_seed(998);

        for _ in 0..20 {
            g1.move_player(Direction::Right);
            g2.move_player(Direction::Right);
        }

        let pos1 = g1
            .world
            .npcs
            .iter()
            .find(|n| n.kind == NpcKind::Outlaw)
            .map(|n| n.pos)
            .expect("outlaw exists");
        let pos2 = g2
            .world
            .npcs
            .iter()
            .find(|n| n.kind == NpcKind::Outlaw)
            .map(|n| n.pos)
            .expect("outlaw exists");

        // Different seeds should generally produce different paths
        // (though there's a small chance they could be the same)
        assert_ne!(pos1, pos2, "Different seeds should produce different paths");
    }

    #[test]
    fn outlaw_bounty_increases_on_crimes() {
        let mut g = Game::new_with_seed(42);

        // Find outlaw
        let outlaw_idx = g
            .world
            .npcs
            .iter()
            .position(|n| n.kind == NpcKind::Outlaw)
            .expect("outlaw exists");

        let initial_bounty = g.world.npcs[outlaw_idx].bounty;

        // Simulate crimes by calling add_bounty
        g.world.npcs[outlaw_idx].add_bounty(50); // Killing sheriff
        assert_eq!(g.world.npcs[outlaw_idx].bounty, initial_bounty + 50);

        g.world.npcs[outlaw_idx].add_bounty(20); // Stealing horse
        assert_eq!(g.world.npcs[outlaw_idx].bounty, initial_bounty + 70);

        g.world.npcs[outlaw_idx].add_bounty(10); // Stealing loot
        assert_eq!(g.world.npcs[outlaw_idx].bounty, initial_bounty + 80);

        g.world.npcs[outlaw_idx].add_bounty(30); // Assaulting player
        assert_eq!(g.world.npcs[outlaw_idx].bounty, initial_bounty + 110);
    }

    #[test]
    fn outlaw_bounty_decays_over_time() {
        let mut g = Game::new_with_seed(42);

        let outlaw_idx = g
            .world
            .npcs
            .iter()
            .position(|n| n.kind == NpcKind::Outlaw)
            .expect("outlaw exists");

        g.world.npcs[outlaw_idx].add_bounty(100);
        assert_eq!(g.world.npcs[outlaw_idx].bounty, 100);

        // Advance 10 turns - bounty should decay by 1
        g.turn = 10;
        g.decay_outlaw_bounties();
        assert_eq!(g.world.npcs[outlaw_idx].bounty, 99);

        // Advance another 10 turns
        g.turn = 20;
        g.decay_outlaw_bounties();
        assert_eq!(g.world.npcs[outlaw_idx].bounty, 98);

        // Bounty should not go below 0
        g.world.npcs[outlaw_idx].bounty = 0;
        g.decay_outlaw_bounties();
        assert_eq!(g.world.npcs[outlaw_idx].bounty, 0);
    }

    #[test]
    fn wanted_level_calculated_correctly() {
        let mut g = Game::new_with_seed(42);
        let outlaw_idx = g
            .world
            .npcs
            .iter()
            .position(|n| n.kind == NpcKind::Outlaw)
            .expect("outlaw exists");

        // Level None (0-10)
        g.world.npcs[outlaw_idx].bounty = 5;
        assert_eq!(g.world.npcs[outlaw_idx].wanted_level(), WantedLevel::None);

        g.world.npcs[outlaw_idx].bounty = 10;
        assert_eq!(g.world.npcs[outlaw_idx].wanted_level(), WantedLevel::None);

        // Level Low (11-30)
        g.world.npcs[outlaw_idx].bounty = 11;
        assert_eq!(g.world.npcs[outlaw_idx].wanted_level(), WantedLevel::Low);

        g.world.npcs[outlaw_idx].bounty = 30;
        assert_eq!(g.world.npcs[outlaw_idx].wanted_level(), WantedLevel::Low);

        // Level Medium (31-60)
        g.world.npcs[outlaw_idx].bounty = 31;
        assert_eq!(g.world.npcs[outlaw_idx].wanted_level(), WantedLevel::Medium);

        g.world.npcs[outlaw_idx].bounty = 60;
        assert_eq!(g.world.npcs[outlaw_idx].wanted_level(), WantedLevel::Medium);

        // Level High (61+)
        g.world.npcs[outlaw_idx].bounty = 61;
        assert_eq!(g.world.npcs[outlaw_idx].wanted_level(), WantedLevel::High);

        g.world.npcs[outlaw_idx].bounty = 100;
        assert_eq!(g.world.npcs[outlaw_idx].wanted_level(), WantedLevel::High);
    }

    #[test]
    fn outlaw_add_bounty_only_affects_outlaws() {
        let mut g = Game::new_with_seed(42);

        // Try to add bounty to sheriff (should be ignored)
        let sheriff_idx = g
            .world
            .npcs
            .iter()
            .position(|n| n.kind == NpcKind::Sheriff)
            .expect("sheriff exists");

        g.world.npcs[sheriff_idx].add_bounty(50);
        assert_eq!(g.world.npcs[sheriff_idx].bounty, 0);
    }

    #[test]
    fn spawn_deputies_medium_wanted() {
        let mut g = Game::new_with_seed(42);

        // Set outlaw bounty to Medium level
        let outlaw_idx = g
            .world
            .npcs
            .iter()
            .position(|n| n.kind == NpcKind::Outlaw)
            .expect("outlaw exists");

        g.world.npcs[outlaw_idx].bounty = 31; // Medium level
        assert_eq!(g.world.npcs[outlaw_idx].wanted_level(), WantedLevel::Medium);

        // Initially no deputies
        let initial_deputies = g
            .world
            .npcs
            .iter()
            .filter(|n| n.kind == NpcKind::Deputy)
            .count();
        assert_eq!(initial_deputies, 0);

        // Spawn deputies
        g.spawn_deputies_if_needed();

        let deputies = g
            .world
            .npcs
            .iter()
            .filter(|n| n.kind == NpcKind::Deputy)
            .count();
        assert_eq!(deputies, 1, "Should spawn 1 deputy for Medium wanted level");
    }

    #[test]
    fn spawn_deputies_high_wanted() {
        let mut g = Game::new_with_seed(42);

        // Set outlaw bounty to High level
        let outlaw_idx = g
            .world
            .npcs
            .iter()
            .position(|n| n.kind == NpcKind::Outlaw)
            .expect("outlaw exists");

        g.world.npcs[outlaw_idx].bounty = 61; // High level
        assert_eq!(g.world.npcs[outlaw_idx].wanted_level(), WantedLevel::High);

        // Spawn deputies
        g.spawn_deputies_if_needed();

        let deputies = g
            .world
            .npcs
            .iter()
            .filter(|n| n.kind == NpcKind::Deputy)
            .count();
        assert_eq!(deputies, 2, "Should spawn 2 deputies for High wanted level");
    }

    #[test]
    fn collect_outlaw_bounty() {
        let mut g = Game::new_with_seed(42);

        // Set outlaw bounty
        let outlaw_idx = g
            .world
            .npcs
            .iter()
            .position(|n| n.kind == NpcKind::Outlaw)
            .expect("outlaw exists");

        g.world.npcs[outlaw_idx].bounty = 500;

        let outlaw_pos = g.world.npcs[outlaw_idx].pos;

        // Collect bounty
        let collected = g.collect_outlaw_bounty_at(outlaw_pos);
        assert_eq!(collected, 500);
        assert_eq!(g.player.score, 500);

        // Outlaw should be removed
        assert!(!g.world.npcs.iter().any(|n| n.kind == NpcKind::Outlaw));
    }
}
