//! Core game state: turn-based Western adventure. Movement, NPC turns,
//! interactions, save/load, and Frame production all live here.
//!
//! Clean-room: original mechanics; nothing copied from a commercial product.

use crate::domain::cell::Cell;
use crate::domain::color::Rgb;
use crate::domain::frame::Frame;
use crate::domain::iso::{screen_depth, world_to_screen};
use crate::domain::player::Player;
use crate::domain::save::SaveData;
use crate::domain::tile::TileType;
use crate::domain::world::{default_world, Item, ItemKind, Npc, Pos, World};

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
                let wx = x as i32;
                let wy = y as i32;
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
                drawn.push((sx, sy, Cell::new(item.kind.glyph(), item.kind.fg(), Rgb::BLACK)));
            }
        }

        // NPCs, drawn over tiles and items.
        for npc in &self.world.npcs {
            if self.world.in_bounds(npc.pos) {
                let (sx, sy) = world_to_screen(npc.pos.x, npc.pos.y);
                drawn.push((sx, sy, Cell::new(npc.kind.glyph(), npc.kind.fg(), Rgb::BLACK)));
            }
        }

        // Player on top of everything.
        if self.world.in_bounds(self.player.pos) {
            let (sx, sy) = world_to_screen(self.player.pos.x, self.player.pos.y);
            drawn.push((sx, sy, Cell::new(self.player.glyph(), self.player.fg(), Rgb::BLACK)));
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
                    };
                    self.message = format!("{}: {}", npc.kind.glyph(), line);
                    let interacted = Interaction::Talked {
                        npc: npc.kind.glyph().to_string(),
                        line: line.to_string(),
                    };
                    if matches!(npc.kind, crate::domain::world::NpcKind::Outlaw)
                        && self.player.mounted
                    {
                        self.game_over = true;
                        self.won = true;
                        self.message = "Outlaw captured! Town is safe.".to_string();
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
    pub fn save(&self, path: &std::path::Path) -> Result<(), String> {
        let save = SaveData::new(self);
        let json =
            serde_json::to_string_pretty(&save).map_err(|e| format!("serialise error: {e}"))?;
        std::fs::write(path, json).map_err(|e| format!("write error: {e}"))?;
        self.message = "Game saved.".to_string();
        Ok(())
    }

    /// Load the game from a JSON file at `path`.
    pub fn load(path: &std::path::Path) -> Result<Self, String> {
        let json = std::fs::read_to_string(path).map_err(|e| format!("read error: {e}"))?;
        let save: SaveData =
            serde_json::from_str(&json).map_err(|e| format!("parse error: {e}"))?;
        Ok(Self {
            world: save.world,
            player: save.player,
            turn: save.turn,
            message: "Game loaded.".to_string(),
            game_over: false,
            won: false,
        })
    }

    /// Advance NPCs one turn. Deterministic (based on turn count) so the game
    /// is testable headlessly.
    fn update_npcs(&mut self) {
        for npc in &mut self.world.npcs {
            match npc.kind {
                crate::domain::world::NpcKind::Sheriff => {
                    // Sheriff stays at his post.
                }
                crate::domain::world::NpcKind::Outlaw => {
                    // Outlaw wanders: move one step if the target is walkable
                    // and not occupied by the player. Direction derived from
                    // the turn so behaviour is deterministic and replayable.
                    let dirs = [
                        Direction::Up,
                        Direction::Down,
                        Direction::Left,
                        Direction::Right,
                    ];
                    let idx = (self.turn % dirs.len() as u64) as usize;
                    let (dx, dy) = dirs[idx].offset();
                    let target = npc.pos.offset(dx, dy);
                    if self.world.in_bounds(target)
                        && self.world.is_walkable(target)
                        && target != self.player.pos
                    {
                        npc.pos = target;
                    }
                }
            }
        }
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
        let mut g = Game::new();
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
}
