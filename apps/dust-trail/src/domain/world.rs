//! The ASCII world: a rectangular tile grid plus entities (NPCs and items).

use crate::domain::color::Rgb;
use crate::domain::tile::TileType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn offset(self, dx: i32, dy: i32) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum NpcKind {
    Sheriff,
    Outlaw,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Npc {
    pub kind: NpcKind,
    pub pos: Pos,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ItemKind {
    Loot,
    Horse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Item {
    pub kind: ItemKind,
    pub pos: Pos,
    pub collected: bool,
}

/// Glyph and colours for an NPC.
impl NpcKind {
    pub fn glyph(self) -> char {
        match self {
            NpcKind::Sheriff => 'S',
            NpcKind::Outlaw => 'O',
        }
    }
    pub fn fg(self) -> Rgb {
        match self {
            NpcKind::Sheriff => Rgb::YELLOW,
            NpcKind::Outlaw => Rgb::RED,
        }
    }
}

/// Glyph and colours for an item.
impl ItemKind {
    pub fn glyph(self) -> char {
        match self {
            ItemKind::Loot => '$',
            ItemKind::Horse => '&',
        }
    }
    pub fn fg(self) -> Rgb {
        match self {
            ItemKind::Loot => Rgb::CYAN,
            ItemKind::Horse => Rgb::BROWN,
        }
    }
}

/// The world grid and its entities.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct World {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<Vec<TileType>>,
    pub npcs: Vec<Npc>,
    pub items: Vec<Item>,
}

impl World {
    pub fn new(width: i32, height: i32) -> Self {
        let tiles = vec![vec![TileType::Ground; width as usize]; height as usize];
        Self {
            width,
            height,
            tiles,
            npcs: Vec::new(),
            items: Vec::new(),
        }
    }

    pub fn in_bounds(&self, pos: Pos) -> bool {
        pos.x >= 0 && pos.x < self.width && pos.y >= 0 && pos.y < self.height
    }

    pub fn tile(&self, pos: Pos) -> Option<TileType> {
        if !self.in_bounds(pos) {
            return None;
        }
        Some(self.tiles[pos.y as usize][pos.x as usize])
    }

    pub fn set_tile(&mut self, pos: Pos, tile: TileType) {
        if self.in_bounds(pos) {
            self.tiles[pos.y as usize][pos.x as usize] = tile;
        }
    }

    /// Whether a position is walkable by the player (tile walkable and no
    /// blocking entity).
    pub fn is_walkable(&self, pos: Pos) -> bool {
        match self.tile(pos) {
            Some(t) if t.walkable() => {}
            _ => return false,
        }
        // NPCs block movement; items do not.
        !self.npcs.iter().any(|n| n.pos == pos)
    }

    pub fn npc_at(&self, pos: Pos) -> Option<Npc> {
        self.npcs.iter().find(|n| n.pos == pos).copied()
    }

    pub fn item_at(&self, pos: Pos) -> Option<Item> {
        self.items.iter().find(|i| !i.collected && i.pos == pos).copied()
    }
}

/// Build the default starting world: a frontier town surrounded by wilderness.
pub fn default_world() -> World {
    let w = 40;
    let h = 22;
    let mut world = World::new(w, h);

    // Border walls.
    for x in 0..w {
        world.set_tile(Pos::new(x, 0), TileType::Wall);
        world.set_tile(Pos::new(x, h - 1), TileType::Wall);
    }
    for y in 0..h {
        world.set_tile(Pos::new(0, y), TileType::Wall);
        world.set_tile(Pos::new(w - 1, y), TileType::Wall);
    }

    // A river across the middle.
    for x in 5..35 {
        world.set_tile(Pos::new(x, 10), TileType::Water);
    }
    // A bridge at x=20.
    world.set_tile(Pos::new(20, 10), TileType::Door);

    // Buildings (town block).
    for (bx, by) in [(5, 3), (8, 3), (11, 3), (5, 6), (8, 6), (11, 6)] {
        world.set_tile(Pos::new(bx, by), TileType::Building);
        world.set_tile(Pos::new(bx + 1, by), TileType::Building);
        world.set_tile(Pos::new(bx, by + 1), TileType::Building);
        world.set_tile(Pos::new(bx + 1, by + 1), TileType::Building);
        world.set_tile(Pos::new(bx + 1, by + 1), TileType::Door);
    }

    // Trees in the wilderness.
    for (tx, ty) in [
        (2, 15), (4, 16), (6, 15), (34, 15), (36, 16), (38, 15),
        (2, 18), (36, 18), (10, 19), (30, 19),
    ] {
        world.set_tile(Pos::new(tx, ty), TileType::Tree);
    }

    // NPCs.
    world.npcs.push(Npc {
        kind: NpcKind::Sheriff,
        pos: Pos::new(7, 5),
    });
    world.npcs.push(Npc {
        kind: NpcKind::Outlaw,
        pos: Pos::new(33, 15),
    });

    // Items.
    world.items.push(Item {
        kind: ItemKind::Loot,
        pos: Pos::new(10, 5),
        collected: false,
    });
    world.items.push(Item {
        kind: ItemKind::Horse,
        pos: Pos::new(35, 17),
        collected: false,
    });

    world
}
