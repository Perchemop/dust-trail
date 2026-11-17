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
    Deputy,
    Outlaw,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Npc {
    pub kind: NpcKind,
    pub pos: Pos,
    /// Bounty value for outlaws (0 for non-outlaws).
    #[serde(default)]
    pub bounty: u32,
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
            NpcKind::Deputy => 'D',
            NpcKind::Outlaw => 'O',
        }
    }
    pub fn fg(self) -> Rgb {
        match self {
            NpcKind::Sheriff => Rgb::YELLOW,
            NpcKind::Deputy => Rgb::BLUE,
            NpcKind::Outlaw => Rgb::RED,
        }
    }
}

/// Wanted level derived from bounty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WantedLevel {
    None,   // 0-10
    Low,    // 11-30
    Medium, // 31-60
    High,   // 61+
}

impl Npc {
    /// Get the wanted level based on bounty.
    pub fn wanted_level(&self) -> WantedLevel {
        if self.kind != NpcKind::Outlaw {
            return WantedLevel::None;
        }
        match self.bounty {
            0..=10 => WantedLevel::None,
            11..=30 => WantedLevel::Low,
            31..=60 => WantedLevel::Medium,
            _ => WantedLevel::High,
        }
    }

    /// Add to bounty (for crimes).
    pub fn add_bounty(&mut self, amount: u32) {
        if self.kind == NpcKind::Outlaw {
            self.bounty = self.bounty.saturating_add(amount);
        }
    }

    /// Decay bounty over time.
    pub fn decay_bounty(&mut self) {
        if self.kind == NpcKind::Outlaw && self.bounty > 0 {
            self.bounty = self.bounty.saturating_sub(1);
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
    /// Create a new world with the given dimensions (all Ground tiles, no entities).
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

    /// Generate a world from a deterministic seed.
    ///
    /// Same seed always produces the same map (deterministic).  The world
    /// includes terrain, border walls, a town block, river, trees, NPCs
    /// (sheriff/outlaw), and items (loot/horse).  Connectivity between the
    /// player start and key entities is guaranteed by internal retries.
    pub fn generate(seed: u64) -> Self {
        use crate::domain::procgen;
        procgen::generate(seed)
    }
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
        self.items
            .iter()
            .find(|i| !i.collected && i.pos == pos)
            .copied()
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
        (2, 15),
        (4, 16),
        (6, 15),
        (34, 15),
        (36, 16),
        (38, 15),
        (2, 18),
        (36, 18),
        (10, 19),
        (30, 19),
    ] {
        world.set_tile(Pos::new(tx, ty), TileType::Tree);
    }

    // NPCs.
    world.npcs.push(Npc {
        kind: NpcKind::Sheriff,
        pos: Pos::new(7, 5),
        bounty: 0,
    });
    world.npcs.push(Npc {
        kind: NpcKind::Outlaw,
        pos: Pos::new(33, 15),
        bounty: 0,
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

/// Spawn deputies based on the outlaw's wanted level.
/// Returns the number of deputies spawned.
pub fn spawn_deputies_for_wanted(world: &mut World, outlaw_pos: Pos, wanted: WantedLevel) -> usize {
    let deputy_count = match wanted {
        WantedLevel::None | WantedLevel::Low => 0,
        WantedLevel::Medium => 1,
        WantedLevel::High => 2,
    };

    if deputy_count == 0 {
        return 0;
    }

    // Spawn deputies around the sheriff's position (7, 5) but not on top of each other
    let sheriff_pos = Pos::new(7, 5);
    let spawn_offsets = [
        (1, 1),
        (-1, 1),
        (1, -1),
        (-1, -1),
        (2, 0),
        (-2, 0),
        (0, 2),
        (0, -2),
    ];

    let mut spawned = 0;
    for (dx, dy) in spawn_offsets {
        if spawned >= deputy_count {
            break;
        }
        let pos = Pos::new(sheriff_pos.x + dx, sheriff_pos.y + dy);
        if world.in_bounds(pos) && world.is_walkable(pos) && pos != outlaw_pos {
            world.npcs.push(Npc {
                kind: NpcKind::Deputy,
                pos,
                bounty: 0,
            });
            spawned += 1;
        }
    }

    spawned
}
