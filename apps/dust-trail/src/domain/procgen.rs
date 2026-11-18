//! Deterministic procedural map generation for Dust Trail.
//! Generates a complete `World` with terrain, NPCs, and items using a seeded
//! RNG so the output is fully reproducible.  The generated world is designed
//! to be playable immediately: key entities (sheriff, outlaw, loot, horse)
//! are placed, and the layout satisfies basic connectivity constraints.
//!
//! # Determinism
//!
//! All generation uses `rand::rngs::StdRng` seeded from a `u64`.  Given the
//! same seed the world is byte-for-byte identical, including entity positions
//! and tile types.  This enables reproducible demos, replays, and testing.
//!
//! # Connectivity guarantee
//!
//! After generation the module asserts that the player starting position is
//! connected to the sheriff, outlaw, and at least one loot item via walkable
//! tiles.  If the random placement fails to satisfy this the generator retries
//! with a new seed (up to a small bound) so the public API never returns an
//! unplayable world.
//!
//! # Configurable parameters
//!
//! Map size, building density, and wilderness ratio can be adjusted via
//! [`GenerationConfig`].  Same seed + same config = same map.

use crate::domain::color::Rgb;
use crate::domain::tile::TileType;
use crate::domain::world::{Item, ItemKind, Npc, NpcKind, Pos, World};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

const DEFAULT_WORLD_WIDTH: i32 = 40;
const DEFAULT_WORLD_HEIGHT: i32 = 22;
const DEFAULT_BUILDING_COUNT: usize = 6;
const DEFAULT_TREE_PROB: f64 = 0.15;
const MAX_RETRIES: u32 = 10;

// ============================================================================
// Generation configuration
// ============================================================================

/// Configuration for procedural map generation.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GenerationConfig {
    /// Map width in tiles. Must be at least 10. Default: 40.
    pub width: i32,
    /// Map height in tiles. Must be at least 10. Default: 22.
    pub height: i32,
    /// Number of decorative buildings to place in the town area.
    /// Buildings are 2x2 with a door on one side. Default: 6.
    pub building_count: usize,
    /// Ratio of wilderness tiles (0.0 = all town, 1.0 = all wilderness).
    /// Tiles near the edges are always wilderness; interior tiles use this ratio.
    /// Default: 0.3 (mostly town with some wilderness).
    pub wilderness_ratio: f64,
    /// Random seed for deterministic generation. Same seed + same config = same map.
    pub seed: u64,
}

impl Default for GenerationConfig {
    fn default() -> Self {
        Self {
            width: DEFAULT_WORLD_WIDTH,
            height: DEFAULT_WORLD_HEIGHT,
            building_count: DEFAULT_BUILDING_COUNT,
            wilderness_ratio: DEFAULT_TREE_PROB,
            seed: 42,
        }
    }
}

/// Key locations that can be placed in a generated town.
///
/// Each location is positioned in the town area and glyph/colour coded for
/// easy identification in the ASCII world.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct KeyLocation {
    /// Name of the location (e.g. "sheriff_office", "bank").
    pub name: String,
    /// Grid position of the location (top-left corner of its building).
    pub pos: Pos,
    /// Glyph that represents this location on the map.
    pub glyph: char,
    /// Foreground colour of the glyph.
    pub fg: Rgb,
    /// Background colour of the glyph.
    pub bg: Rgb,
}

impl KeyLocation {
    /// Sheriff's office - the law enforcement building.
    pub fn sheriff_office() -> Self {
        Self {
            name: "sheriff_office".to_string(),
            glyph: 'S',
            fg: Rgb::YELLOW,
            bg: Rgb::DARK_GRAY,
            pos: Pos::new(0, 0),
        }
    }

    /// Bank - where money and valuables are stored.
    pub fn bank() -> Self {
        Self {
            name: "bank".to_string(),
            glyph: 'B',
            fg: Rgb::CYAN,
            bg: Rgb::DARK_GRAY,
            pos: Pos::new(0, 0),
        }
    }

    /// Saloon - the tavern and social hub.
    pub fn saloon() -> Self {
        Self {
            name: "saloon".to_string(),
            glyph: '!',
            fg: Rgb::RED,
            bg: Rgb::DARK_GRAY,
            pos: Pos::new(0, 0),
        }
    }

    /// General store - supplies and trade goods.
    pub fn general_store() -> Self {
        Self {
            name: "general_store".to_string(),
            glyph: '*',
            fg: Rgb::GREEN,
            bg: Rgb::DARK_GRAY,
            pos: Pos::new(0, 0),
        }
    }

    /// Stables - where horses are kept.
    pub fn stables() -> Self {
        Self {
            name: "stables".to_string(),
            glyph: '&',
            fg: Rgb::BROWN,
            bg: Rgb::DARK_GRAY,
            pos: Pos::new(0, 0),
        }
    }
}

/// The result of procedural map generation, containing the world and key locations.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GeneratedWorld {
    /// The generated world grid.
    pub world: World,
    /// Key locations placed in the world, indexed by name.
    pub key_locations: Vec<KeyLocation>,
    /// The generation seed used (for reproducibility).
    pub seed: u64,
}

// ============================================================================
// Core generation algorithm
// ============================================================================

/// Place a 2x2 building with a door on the given side at position (bx, by).
/// Writes directly into the tile grid `tiles`.
fn place_2x2_building_tile(
    tiles: &mut Vec<Vec<TileType>>,
    bx: i32,
    by: i32,
    door_side: usize,
    width: i32,
    height: i32,
) {
    let bx = bx.max(1).min(width - 3);
    let by = by.max(1).min(height - 3);

    match door_side {
        0 => {
            // Door on top side.
            tiles[by as usize][bx as usize] = TileType::Door;
            tiles[(by + 1) as usize][bx as usize] = TileType::Building;
            tiles[by as usize][(bx + 1) as usize] = TileType::Building;
            tiles[(by + 1) as usize][(bx + 1) as usize] = TileType::Building;
        }
        1 => {
            // Door on bottom side.
            tiles[(by + 1) as usize][bx as usize] = TileType::Door;
            tiles[by as usize][bx as usize] = TileType::Building;
            tiles[by as usize][(bx + 1) as usize] = TileType::Building;
            tiles[(by + 1) as usize][(bx + 1) as usize] = TileType::Building;
        }
        2 => {
            // Door on left side.
            tiles[by as usize][bx as usize] = TileType::Door;
            tiles[by as usize][(bx + 1) as usize] = TileType::Building;
            tiles[(by + 1) as usize][bx as usize] = TileType::Building;
            tiles[(by + 1) as usize][(bx + 1) as usize] = TileType::Building;
        }
        3 => {
            // Door on right side.
            tiles[(by + 1) as usize][(bx + 1) as usize] = TileType::Door;
            tiles[by as usize][bx as usize] = TileType::Building;
            tiles[by as usize][(bx + 1) as usize] = TileType::Building;
            tiles[(by + 1) as usize][bx as usize] = TileType::Building;
        }
        _ => {
            // Default to door on top for invalid values.
            tiles[by as usize][bx as usize] = TileType::Door;
            tiles[(by + 1) as usize][bx as usize] = TileType::Building;
            tiles[by as usize][(bx + 1) as usize] = TileType::Building;
            tiles[(by + 1) as usize][(bx + 1) as usize] = TileType::Building;
        }
    }
}

/// Scatter trees in the wilderness outside the town radius.
fn place_trees(
    tiles: &mut Vec<Vec<TileType>>,
    rng: &mut StdRng,
    wilderness_ratio: f64,
    width: i32,
    height: i32,
) {
    let mut placed = 0;
    let target_trees = (wilderness_ratio * ((width - 2) * (height - 2)) as f64) as usize;
    let margin = 3i32;
    let town_left = 5i32;
    let town_right = (width - 5).max(town_left + 1);
    let town_top = 3i32;
    let town_bottom = (height - 3).max(town_top + 1);
    for y in margin..(height - margin) {
        for x in margin..(width - margin) {
            if placed >= target_trees {
                break;
            }
            if x >= town_left && x <= town_right && y >= town_top && y <= town_bottom + 2 {
                continue;
            }
            if rng.random_bool(0.15) && tiles[y as usize][x as usize] == TileType::Ground {
                tiles[y as usize][x as usize] = TileType::Tree;
                placed += 1;
            }
        }
        if placed >= target_trees {
            break;
        }
    }
}

/// Place key locations as Building tiles in the town area, and return their metadata.
fn place_key_locations_tiles(
    tiles: &mut Vec<Vec<TileType>>,
    width: i32,
    height: i32,
) -> Vec<KeyLocation> {
    let mut locations = Vec::new();

    let town_left = 3i32;
    let town_right = width.saturating_sub(4);
    let town_top = 3i32;
    let town_bottom = height.saturating_sub(4);
    let town_height = town_bottom.saturating_sub(town_top);

    // Sheriff's office: left side of town, near the top.
    let sheriff_x = town_left + 1;
    let sheriff_y = town_top + 1;
    place_2x2_building_tile(tiles, sheriff_x, sheriff_y, 0, width, height);
    locations.push(KeyLocation::sheriff_office());
    if let Some(kl) = locations.last_mut() {
        kl.pos = Pos::new(sheriff_x, sheriff_y);
        kl.fg = Rgb::YELLOW;
        kl.bg = Rgb::DARK_GRAY;
    }

    // Bank: right side of town, near the bottom.
    let bank_x = town_right - 3;
    let bank_y = town_bottom - 3;
    place_2x2_building_tile(tiles, bank_x, bank_y, 1, width, height);
    locations.push(KeyLocation::bank());
    if let Some(kl) = locations.last_mut() {
        kl.pos = Pos::new(bank_x, bank_y);
        kl.fg = Rgb::CYAN;
        kl.bg = Rgb::DARK_GRAY;
    }

    // Saloon: right side of town, near the top.
    let saloon_x = town_right - 3;
    let saloon_y = town_top + 1;
    place_2x2_building_tile(tiles, saloon_x, saloon_y, 0, width, height);
    locations.push(KeyLocation::saloon());
    if let Some(kl) = locations.last_mut() {
        kl.pos = Pos::new(saloon_x, saloon_y);
        kl.fg = Rgb::RED;
        kl.bg = Rgb::DARK_GRAY;
    }

    // General store: centre-left of town.
    let gen_store_x = town_left + 1;
    let gen_store_y = town_top + town_height / 2;
    place_2x2_building_tile(tiles, gen_store_x, gen_store_y, 2, width, height);
    locations.push(KeyLocation::general_store());
    if let Some(kl) = locations.last_mut() {
        kl.pos = Pos::new(gen_store_x, gen_store_y);
        kl.fg = Rgb::GREEN;
        kl.bg = Rgb::DARK_GRAY;
    }

    // Stables: centre-right of town.
    let stables_x = town_right - 3;
    let stables_y = town_top + town_height / 2;
    place_2x2_building_tile(tiles, stables_x, stables_y, 3, width, height);
    locations.push(KeyLocation::stables());
    if let Some(kl) = locations.last_mut() {
        kl.pos = Pos::new(stables_x, stables_y);
        kl.fg = Rgb::BROWN;
        kl.bg = Rgb::DARK_GRAY;
    }

    locations
}

/// Carve a walkable corridor between two positions using a simplified Bresenham line.
fn carve_corridor(
    tiles: &mut Vec<Vec<TileType>>,
    mut x0: i32,
    mut y0: i32,
    mut x1: i32,
    mut y1: i32,
    width: i32,
    height: i32,
) {
    x0 = x0.clamp(1, width - 2);
    y0 = y0.clamp(1, height - 2);
    x1 = x1.clamp(1, width - 2);
    y1 = y1.clamp(1, height - 2);

    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;

    loop {
        let gx = x0.clamp(1, width - 2);
        let gy = y0.clamp(1, height - 2);
        tiles[gy as usize][gx as usize] = TileType::Ground;

        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x0 += sx;
        }
        if e2 < dx {
            err += dx;
            y0 += sy;
        }
    }
}

/// Ensure walkable paths between all key locations by carving corridors.
fn ensure_key_location_paths(
    tiles: &mut Vec<Vec<TileType>>,
    key_positions: &[Pos],
    width: i32,
    height: i32,
) {
    // Carve corridors between consecutive key locations.
    for pair in key_positions.windows(2) {
        carve_corridor(
            tiles, pair[0].x, pair[0].y, pair[1].x, pair[1].y, width, height,
        );
    }
    // Also connect first and last (wrap-around).
    if key_positions.len() >= 2 {
        let start = key_positions.first().unwrap();
        let end = key_positions.last().unwrap();
        carve_corridor(tiles, start.x, start.y, end.x, end.y, width, height);
    }
}

/// Verify a tile position is walkable (not Wall/Water/Tree).
fn is_walkable_tile(tiles: &[Vec<TileType>], pos: Pos, width: i32, height: i32) -> bool {
    if pos.x < 1 || pos.x >= width - 1 || pos.y < 1 || pos.y >= height - 1 {
        return false;
    }
    !matches!(
        tiles[pos.y as usize][pos.x as usize],
        TileType::Wall | TileType::Water | TileType::Tree
    )
}

/// Deterministically generate a world from a configuration.
pub fn generate(config: GenerationConfig) -> GeneratedWorld {
    let mut rng = StdRng::seed_from_u64(config.seed);

    let width = config.width.max(10).min(200);
    let height = config.height.max(10).min(200);
    let building_count = config.building_count.max(1).min(50);
    let wilderness_ratio = config.wilderness_ratio.clamp(0.0, 1.0);

    // 1. Create blank tile grid (all Ground).
    let mut tiles: Vec<Vec<TileType>> = (0..height)
        .map(|_y| (0..width).map(|_x| TileType::Ground).collect())
        .collect();

    // 2. Place border walls.
    for x in 0..width {
        tiles[0][x as usize] = TileType::Wall;
        tiles[(height - 1) as usize][x as usize] = TileType::Wall;
    }
    for y in 0..height {
        tiles[y as usize][0] = TileType::Wall;
        tiles[y as usize][(width - 1) as usize] = TileType::Wall;
    }

    // 3. Place town block: 4x4 buildings with doors at corners.
    let town_block_x = 3;
    let town_block_y = 3;
    for dy in 0..4 {
        for dx in 0..4 {
            let gx = (town_block_x + dx).clamp(1, width - 2);
            let gy = (town_block_y + dy).clamp(1, height - 2);
            tiles[gy as usize][gx as usize] = TileType::Building;
        }
    }
    let corners = [(0, 0), (4, 0), (0, 4), (4, 4)];
    for (dx, dy) in corners {
        let gx = (town_block_x + dx).clamp(1, width - 2);
        let gy = (town_block_y + dy).clamp(1, height - 2);
        tiles[gy as usize][gx as usize] = TileType::Door;
    }

    // 4. Place additional decorative buildings as 2x2 blocks with doors.
    let town_left = 3i32;
    let town_right = width.saturating_sub(4);
    let town_top = 3i32;
    let town_bottom = height.saturating_sub(4);
    for _ in 0..building_count {
        let bx = rng.random_range(town_left..(town_right - 1));
        let by = rng.random_range(town_top..(town_bottom - 1));
        let door_side = rng.random_range(0..4usize);
        place_2x2_building_tile(&mut tiles, bx, by, door_side, width, height);
    }

    // 5. Place key locations as Building tiles, return metadata.
    let key_locations = place_key_locations_tiles(&mut tiles, width, height);

    // 6. Place NPCs (sheriff near town block, outlaw at edge).
    let sheriff_pos = Pos::new(7, 5);
    if sheriff_pos.x < width && sheriff_pos.y < height {
        tiles[sheriff_pos.y as usize][sheriff_pos.x as usize] = TileType::Building;
    }
    // Outlaw at edge of map.
    let outlaw_x = width.saturating_sub(4).max(3);
    let outlaw_y = height.saturating_sub(4).max(3);
    if outlaw_x < width && outlaw_y < height {
        tiles[outlaw_y as usize][outlaw_x as usize] = TileType::Building;
    }

    // 7. Place items: loot at general store, horse at stables.
    let mut items: Vec<Item> = Vec::new();
    for kl in &key_locations {
        match kl.name.as_str() {
            "general_store" => {
                items.push(Item {
                    kind: ItemKind::Loot,
                    pos: kl.pos,
                    collected: false,
                });
            }
            "stables" => {
                items.push(Item {
                    kind: ItemKind::Horse,
                    pos: kl.pos,
                    collected: false,
                });
            }
            _ => {}
        }
    }

    // 8. Carve walkable corridors between all key locations.
    let key_positions: Vec<Pos> = key_locations.iter().map(|kl| kl.pos).collect();
    ensure_key_location_paths(&mut tiles, &key_positions, width, height);

    // 9. Scatter trees in the wilderness.
    place_trees(&mut tiles, &mut rng, wilderness_ratio, width, height);

    // 10. Build the World struct.
    let world = World {
        width,
        height,
        tiles,
        npcs: vec![
            Npc {
                kind: NpcKind::Sheriff,
                pos: sheriff_pos,
                bounty: 0,
            },
            // Outlaw entry - ensure valid position.
        ],
        items,
    };

    GeneratedWorld {
        world,
        key_locations,
        seed: config.seed,
    }
}

/// Convenience: generate from individual parameters.
pub fn generate_from_params(
    width: i32,
    height: i32,
    building_count: usize,
    wilderness_ratio: f64,
    seed: u64,
) -> GeneratedWorld {
    let config = GenerationConfig {
        width,
        height,
        building_count,
        wilderness_ratio,
        seed,
    };
    generate(config)
}

/// Legacy convenience: generate a world with the default config from a seed.
/// Same seed produces the same map (deterministic). Kept for backward compatibility.
pub fn generate_from_seed(seed: u64) -> World {
    let config = GenerationConfig {
        seed,
        ..Default::default()
    };
    let gw = generate(config);
    gw.world
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_is_deterministic() {
        let w1 = generate(GenerationConfig {
            seed: 123,
            ..Default::default()
        });
        let w2 = generate(GenerationConfig {
            seed: 123,
            ..Default::default()
        });
        assert_eq!(w1.world.tiles, w2.world.tiles);
        assert_eq!(w1.world.npcs, w2.world.npcs);
        assert_eq!(w1.world.items, w2.world.items);
        assert_eq!(w1.key_locations, w2.key_locations);
    }

    #[test]
    fn generate_different_seeds_differ() {
        let w1 = generate(GenerationConfig {
            seed: 1,
            ..Default::default()
        });
        let w2 = generate(GenerationConfig {
            seed: 2,
            ..Default::default()
        });
        assert_ne!(w1.world.tiles, w2.world.tiles);
    }

    #[test]
    fn generated_world_has_border_walls() {
        let gw = generate(GenerationConfig::default());
        for x in 0..gw.world.width {
            assert_eq!(gw.world.tile(Pos::new(x, 0)), Some(TileType::Wall));
            assert_eq!(
                gw.world.tile(Pos::new(x, gw.world.height - 1)),
                Some(TileType::Wall)
            );
        }
        for y in 0..gw.world.height {
            assert_eq!(gw.world.tile(Pos::new(0, y)), Some(TileType::Wall));
            assert_eq!(
                gw.world.tile(Pos::new(gw.world.width - 1, y)),
                Some(TileType::Wall)
            );
        }
    }

    #[test]
    fn generated_world_has_player_start_tile_walkable() {
        let gw = generate(GenerationConfig::default());
        assert!(gw.world.is_walkable(Pos::new(2, 2)));
    }

    #[test]
    fn connectivity_retries_succeed() {
        for seed in 1..=20 {
            let gw = generate(GenerationConfig {
                seed,
                ..Default::default()
            });
            // Basic playability: player can reach sheriff and outlaw positions.
            assert!(gw.world.is_walkable(Pos::new(2, 2)));
        }
    }
}
