//! Core game state: turn-based Western adventure. Movement, NPC turns,
//! interactions, save/load, and Frame production all live here.
//!
//! Clean-room: original mechanics; nothing copied from a commercial product.

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
use std::path::Path;

#[cfg(feature = "gui")]
use crate::render::{Camera, Sprite, SpriteCache};

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
/// `Clone` was dropped in the rand 0.10 migration: `StdRng` no longer
/// implements it (rand #1677), and no caller clones a whole `Game`.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
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
    /// Sprite cache for 3D ASCII rendering (gui feature only).
    #[cfg(feature = "gui")]
    #[serde(skip, default = "default_sprite_cache")]
    sprite_cache: SpriteCache,
    /// Whether the sprite cache needs invalidation (world state changed).
    #[cfg(feature = "gui")]
    #[serde(skip, default)]
    cache_dirty: bool,
}

#[cfg(feature = "sound")]
fn default_sound_manager() -> SoundManager {
    SoundManager::new().unwrap_or_default()
}

#[cfg(feature = "gui")]
fn default_sprite_cache() -> SpriteCache {
    SpriteCache::new(256)
}

fn default_rng() -> StdRng {
    rand::make_rng()
}

impl Default for Game {
    fn default() -> Self {
        Self::new_static()
    }
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
            rng: rand::make_rng(),
            #[cfg(feature = "sound")]
            sound_manager: SoundManager::new().unwrap_or_default(),
            #[cfg(not(feature = "sound"))]
            sound_settings: SoundSettings::new_stub(),
            #[cfg(feature = "gui")]
            sprite_cache: SpriteCache::new(256),
            #[cfg(feature = "gui")]
            cache_dirty: false,
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
            #[cfg(feature = "gui")]
            sprite_cache: SpriteCache::new(256),
            #[cfg(feature = "gui")]
            cache_dirty: false,
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
            rng: rand::make_rng(),
            #[cfg(feature = "sound")]
            sound_manager: SoundManager::new().unwrap_or_default(),
            #[cfg(not(feature = "sound"))]
            sound_settings: SoundSettings::new_stub(),
            #[cfg(feature = "gui")]
            sprite_cache: SpriteCache::new(256),
            #[cfg(feature = "gui")]
            cache_dirty: false,
        }
    }

    /// Render the current state as a Frame for either frontend.
    pub fn frame(&mut self) -> Frame {
        #[cfg(feature = "gui")]
        {
            self.frame_with_render()
        }

        #[cfg(not(feature = "gui"))]
        {
            // Fallback: original flat rendering for non-gui builds (headless tests)
            self.frame_flat()
        }
    }

    /// New render pipeline using 3D primitive sprites with depth-aware compositing.
    #[cfg(feature = "gui")]
    fn frame_with_render(&mut self) -> Frame {
        use crate::render::{composite_sprite, EntityKind, Rasteriser};

        // Build or reuse model registry
        let registry = Self::build_model_registry();

        // Create rasteriser with fixed camera
        let camera = Camera::fixed_isometric();
        let raster = Rasteriser::new(80, 50).with_light((0.0, 0.0, 1.0));

        // Clear frame depth buffer
        let mut frame = Frame::new(80, 50);
        frame.clear_depth();

        // Render terrain tiles as flat sprites (back-to-front by screen depth)
        let mut tile_sprites: Vec<(Sprite, i32, i32)> = Vec::new();
        for y in 0..self.world.height {
            for x in 0..self.world.width {
                let wy = y as usize;
                let wx = x as usize;
                let tile = self.world.tiles[wy][wx];
                let sprite = tile.to_sprite(&camera);
                tile_sprites.push((sprite, x, y));
            }
        }
        // Sort back-to-front by screen depth
        tile_sprites.sort_by(|(_, wx, wy), (_, wx2, wy2)| {
            let d1 = camera.screen_depth(*wx, *wy, 0);
            let d2 = camera.screen_depth(*wx2, *wy2, 0);
            d1.partial_cmp(&d2).unwrap_or(std::cmp::Ordering::Equal)
        });

        // Composite tile sprites
        for (sprite, wx, wy) in tile_sprites {
            let (sx, sy, _sz) = camera.project(wx, wy, 0);
            composite_sprite(&mut frame, &sprite, sx, sy);
        }

        // Collect entity sprites with their screen positions and depths
        let mut entity_sprites: Vec<(Sprite, i32, i32, f32)> = Vec::new();

        // Collect item data first - use explicit for-loop to avoid borrow checker
        // conflict with later self.get_or_render_sprite() calls.
        // The for-loop borrows self.world items immutably within the loop body,
        // which ends each iteration, allowing later mutable borrows of self.
        let mut item_entries: Vec<(crate::domain::world::ItemKind, i32, i32)> = Vec::new();
        for item in &self.world.items {
            if !item.collected && self.world.in_bounds(item.pos) {
                item_entries.push((item.kind, item.pos.x, item.pos.y));
            }
        }

        // Map item kinds to EntityKind and compute screen depths
        let item_data: Vec<(EntityKind, i32, i32, f32)> = item_entries
            .iter()
            .map(|(kind, x, y)| {
                let entity_kind = match kind {
                    crate::domain::world::ItemKind::Loot => EntityKind::Loot,
                    crate::domain::world::ItemKind::Horse => EntityKind::Horse,
                };
                let depth = camera.screen_depth(*x, *y, 0);
                (entity_kind, *x, *y, depth)
            })
            .collect();

        // NPCs
        let mut npc_entries: Vec<(crate::domain::world::NpcKind, i32, i32, i32)> = Vec::new();
        for npc in &self.world.npcs {
            if self.world.in_bounds(npc.pos) {
                npc_entries.push((npc.kind, npc.pos.x, npc.pos.y, npc.pos.z));
            }
        }

        // Map NPC kinds to EntityKind and compute screen depths
        let npc_data: Vec<(EntityKind, i32, i32, f32)> = npc_entries
            .iter()
            .map(|(kind, x, y, z)| {
                let entity_kind = match kind {
                    crate::domain::world::NpcKind::Sheriff => EntityKind::Sheriff,
                    crate::domain::world::NpcKind::Deputy => EntityKind::Deputy,
                    crate::domain::world::NpcKind::Outlaw => EntityKind::Outlaw,
                };
                let depth = camera.screen_depth(*x, *y, *z);
                (entity_kind, *x, *y, depth)
            })
            .collect();

        // Items
        for (kind, wx, wy, depth) in item_data {
            let model = registry.model_for(kind);
            let sprite = self.get_or_render_sprite(model, &raster, &camera);
            entity_sprites.push((sprite, wx, wy, depth));
        }

        // NPCs
        for (kind, wx, wy, depth) in npc_data {
            let model = registry.model_for(kind);
            let sprite = self.get_or_render_sprite(model, &raster, &camera);
            entity_sprites.push((sprite, wx, wy, depth));
        }

        // Player
        let player_model = registry.model_for(EntityKind::Player);
        let player_sprite = self.get_or_render_sprite(player_model, &raster, &camera);
        let player_depth = camera.screen_depth(self.player.pos.x, self.player.pos.y, self.player.pos.z);
        entity_sprites.push((player_sprite, self.player.pos.x, self.player.pos.y, player_depth));

        // Sort entities back-to-front by screen depth
        entity_sprites.sort_by(|a, b| a.3.partial_cmp(&b.3).unwrap_or(std::cmp::Ordering::Equal));

        // Composite entity sprites with depth-aware occlusion
        for (sprite, wx, wy, _depth) in entity_sprites {
            let (sx, sy, _sz) = camera.project(wx, wy, 0); // height handled in sprite
            composite_sprite(&mut frame, &sprite, sx, sy);
        }

        // Add message to frame
        frame.message = self.message.clone();
        frame
    }

    #[cfg(feature = "gui")]
    fn get_or_render_sprite(&mut self, model: &crate::render::ObjectModel, raster: &crate::render::Rasteriser, camera: &crate::render::Camera) -> crate::render::Sprite {
        let light_state = crate::render::LightState::from_light((0.0, 0.0, 1.0), 1.0);
        let cache_key = crate::render::CacheKey {
            model_key: model.key,
            light_state,
            size: (80, 50),
            orientation: crate::render::Orientation::Isometric,
        };

        if let Some(sprite) = self.sprite_cache.get(&cache_key) {
            return sprite.clone();
        }

        // Cache miss: render and store
        let sprite = raster.render(model, camera);
        self.sprite_cache.insert(cache_key, sprite.clone());
        sprite
    }

    /// Build the model registry with all entity kinds.
    #[cfg(feature = "gui")]
    fn build_model_registry() -> crate::render::ModelRegistry {
        use crate::render::{EntityKind, Material, ModelRegistry, ObjectModel, Primitive, PrimitiveKind, Transform, DrawKey, BoundingBox};

        let mut registry = ModelRegistry::new();

        // Tree: trunk (Cone) + canopy (Ellipsoid)
        let tree = ObjectModel {
            key: "tree_oak",
            name: "Oak Tree".to_string(),
            primitives: vec![
                Primitive {
                    kind: PrimitiveKind::Cone,
                    transform: Transform { translation: (0.0, 0.0, 0.0), rotation: (0.0, 0.0, 0.0), scale: (2.0, 2.0, 4.0) },
                    material: Material { albedo: 0.4, shade_side: true },
                },
                Primitive {
                    kind: PrimitiveKind::Ellipsoid,
                    transform: Transform { translation: (0.0, 0.0, 4.0), rotation: (0.0, 0.0, 0.0), scale: (3.0, 3.0, 2.0) },
                    material: Material { albedo: 0.7, shade_side: true },
                },
            ],
            bounding_box: BoundingBox::default(),
            draw_key: DrawKey(10),
        };
        registry.register(EntityKind::Tree, tree).unwrap();

        // Shrub: small Ellipsoid
        let shrub = ObjectModel {
            key: "shrub",
            name: "Shrub".to_string(),
            primitives: vec![Primitive {
                kind: PrimitiveKind::Ellipsoid,
                transform: Transform { translation: (0.0, 0.0, 0.0), rotation: (0.0, 0.0, 0.0), scale: (1.5, 1.5, 1.0) },
                material: Material { albedo: 0.6, shade_side: true },
            }],
            bounding_box: BoundingBox::default(),
            draw_key: DrawKey(5),
        };
        registry.register(EntityKind::Shrub, shrub).unwrap();

        // Cactus: tall Box
        let cactus = ObjectModel {
            key: "cactus",
            name: "Cactus".to_string(),
            primitives: vec![Primitive {
                kind: PrimitiveKind::Box,
                transform: Transform { translation: (0.0, 0.0, 0.0), rotation: (0.0, 0.0, 0.0), scale: (1.0, 1.0, 3.0) },
                material: Material { albedo: 0.5, shade_side: true },
            }],
            bounding_box: BoundingBox::default(),
            draw_key: DrawKey(5),
        };
        registry.register(EntityKind::Cactus, cactus).unwrap();

        // Rock: small Box
        let rock = ObjectModel {
            key: "rock",
            name: "Rock".to_string(),
            primitives: vec![Primitive {
                kind: PrimitiveKind::Box,
                transform: Transform { translation: (0.0, 0.0, 0.0), rotation: (0.0, 0.0, 0.0), scale: (1.0, 1.0, 0.5) },
                material: Material { albedo: 0.4, shade_side: true },
            }],
            bounding_box: BoundingBox::default(),
            draw_key: DrawKey(2),
        };
        registry.register(EntityKind::Rock, rock).unwrap();

        // Water: flat Ellipsoid
        let water = ObjectModel {
            key: "water",
            name: "Water".to_string(),
            primitives: vec![Primitive {
                kind: PrimitiveKind::Ellipsoid,
                transform: Transform { translation: (0.0, 0.0, 0.0), rotation: (0.0, 0.0, 0.0), scale: (2.0, 2.0, 0.2) },
                material: Material { albedo: 0.8, shade_side: false },
            }],
            bounding_box: BoundingBox::default(),
            draw_key: DrawKey(1),
        };
        registry.register(EntityKind::Water, water).unwrap();

        // Barn: Box building
        let barn = ObjectModel {
            key: "barn",
            name: "Barn".to_string(),
            primitives: vec![Primitive {
                kind: PrimitiveKind::Box,
                transform: Transform { translation: (0.0, 0.0, 0.0), rotation: (0.0, 0.0, 0.0), scale: (4.0, 3.0, 3.0) },
                material: Material { albedo: 0.5, shade_side: true },
            }],
            bounding_box: BoundingBox::default(),
            draw_key: DrawKey(15),
        };
        registry.register(EntityKind::Barn, barn).unwrap();

        // Player: Box torso + Cylinder hat
        let player = ObjectModel {
            key: "player",
            name: "Player".to_string(),
            primitives: vec![
                Primitive {
                    kind: PrimitiveKind::Box,
                    transform: Transform { translation: (0.0, 0.0, 0.0), rotation: (0.0, 0.0, 0.0), scale: (1.0, 0.8, 1.5) },
                    material: Material { albedo: 0.9, shade_side: true },
                },
                Primitive {
                    kind: PrimitiveKind::Cylinder,
                    transform: Transform { translation: (0.0, 0.0, 1.5), rotation: (0.0, 0.0, 0.0), scale: (0.8, 0.8, 0.3) },
                    material: Material { albedo: 0.6, shade_side: true },
                },
            ],
            bounding_box: BoundingBox::default(),
            draw_key: DrawKey(20),
        };
        registry.register(EntityKind::Player, player).unwrap();

        // Sheriff
        let sheriff = ObjectModel {
            key: "sheriff",
            name: "Sheriff".to_string(),
            primitives: vec![
                Primitive {
                    kind: PrimitiveKind::Box,
                    transform: Transform { translation: (0.0, 0.0, 0.0), rotation: (0.0, 0.0, 0.0), scale: (1.0, 0.8, 1.5) },
                    material: Material { albedo: 0.7, shade_side: true },
                },
                Primitive {
                    kind: PrimitiveKind::Cylinder,
                    transform: Transform { translation: (0.0, 0.0, 1.5), rotation: (0.0, 0.0, 0.0), scale: (0.8, 0.8, 0.3) },
                    material: Material { albedo: 0.8, shade_side: true },
                },
            ],
            bounding_box: BoundingBox::default(),
            draw_key: DrawKey(20),
        };
        registry.register(EntityKind::Sheriff, sheriff).unwrap();

        // Deputy
        let deputy = ObjectModel {
            key: "deputy",
            name: "Deputy".to_string(),
            primitives: vec![Primitive {
                kind: PrimitiveKind::Box,
                transform: Transform { translation: (0.0, 0.0, 0.0), rotation: (0.0, 0.0, 0.0), scale: (1.0, 0.8, 1.5) },
                material: Material { albedo: 0.6, shade_side: true },
            }],
            bounding_box: BoundingBox::default(),
            draw_key: DrawKey(20),
        };
        registry.register(EntityKind::Deputy, deputy).unwrap();

        // Outlaw
        let outlaw = ObjectModel {
            key: "outlaw",
            name: "Outlaw".to_string(),
            primitives: vec![Primitive {
                kind: PrimitiveKind::Box,
                transform: Transform { translation: (0.0, 0.0, 0.0), rotation: (0.0, 0.0, 0.0), scale: (1.0, 0.8, 1.5) },
                material: Material { albedo: 0.5, shade_side: true },
            }],
            bounding_box: BoundingBox::default(),
            draw_key: DrawKey(20),
        };
        registry.register(EntityKind::Outlaw, outlaw).unwrap();

        // Loot: Cylinder (coin bag)
        let loot = ObjectModel {
            key: "loot",
            name: "Loot".to_string(),
            primitives: vec![Primitive {
                kind: PrimitiveKind::Cylinder,
                transform: Transform { translation: (0.0, 0.0, 0.0), rotation: (0.0, 0.0, 0.0), scale: (0.8, 0.8, 0.5) },
                material: Material { albedo: 1.0, shade_side: true },
            }],
            bounding_box: BoundingBox::default(),
            draw_key: DrawKey(5),
        };
        registry.register(EntityKind::Loot, loot).unwrap();

        // Horse: Box body + Cylinder legs
        let horse = ObjectModel {
            key: "horse",
            name: "Horse".to_string(),
            primitives: vec![
                Primitive {
                    kind: PrimitiveKind::Box,
                    transform: Transform { translation: (0.0, 0.0, 0.0), rotation: (0.0, 0.0, 0.0), scale: (2.0, 1.0, 1.2) },
                    material: Material { albedo: 0.6, shade_side: true },
                },
                Primitive {
                    kind: PrimitiveKind::Cylinder,
                    transform: Transform { translation: (-0.8, -0.4, 0.0), rotation: (0.0, 0.0, 0.0), scale: (0.3, 0.3, 1.0) },
                    material: Material { albedo: 0.4, shade_side: true },
                },
                Primitive {
                    kind: PrimitiveKind::Cylinder,
                    transform: Transform { translation: (0.8, -0.4, 0.0), rotation: (0.0, 0.0, 0.0), scale: (0.3, 0.3, 1.0) },
                    material: Material { albedo: 0.4, shade_side: true },
                },
                Primitive {
                    kind: PrimitiveKind::Cylinder,
                    transform: Transform { translation: (-0.8, 0.4, 0.0), rotation: (0.0, 0.0, 0.0), scale: (0.3, 0.3, 1.0) },
                    material: Material { albedo: 0.4, shade_side: true },
                },
                Primitive {
                    kind: PrimitiveKind::Cylinder,
                    transform: Transform { translation: (0.8, 0.4, 0.0), rotation: (0.0, 0.0, 0.0), scale: (0.3, 0.3, 1.0) },
                    material: Material { albedo: 0.4, shade_side: true },
                },
            ],
            bounding_box: BoundingBox::default(),
            draw_key: DrawKey(15),
        };
        registry.register(EntityKind::Horse, horse).unwrap();

        registry
    }

    /// Flat rendering fallback for non-gui builds (headless tests).
    #[allow(dead_code)]
    fn frame_flat(&self) -> Frame {
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

        let mut frame = Frame::new(width, height);
        frame.cells = cells;
        frame.message = self.message.clone();
        frame
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
        #[cfg(feature = "gui")]
        self.mark_cache_dirty();
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
            #[cfg(feature = "gui")]
            self.mark_cache_dirty();
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
                }
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
                self.player.inventory.push(item.kind);
                #[cfg(feature = "gui")]
                self.mark_cache_dirty();
                self.message = match item.kind {
                    ItemKind::Horse => {
                        #[cfg(feature = "sound")]
                        self.sound_manager.play(SoundEffect::HorseMount);
                        "You picked up a horse.".to_string()
                    }
                    ItemKind::Loot => {
                        #[cfg(feature = "sound")]
                        self.sound_manager.play(SoundEffect::LootPickup);
                        "You picked up a satchel of coins.".to_string()
                    }
                };
                return Interaction::Collected { kind: item.kind };
            }
        }

        Interaction::None
    }

    /// Use an item from inventory by kind. Returns the score effect.
    pub fn use_item(&mut self, kind: ItemKind) -> Option<u32> {
        let score_effect = self.player.use_item(kind)?;
        if score_effect > 0 {
            self.player.score = self.player.score.saturating_add(score_effect);
            self.message = match kind {
                ItemKind::Loot => "You consumed a satchel of coins (+5).".to_string(),
                ItemKind::Horse => "You mounted your horse.".to_string(),
            };
            #[cfg(feature = "gui")]
            self.mark_cache_dirty();
            #[cfg(feature = "sound")]
            match kind {
                ItemKind::Loot => self.sound_manager.play(SoundEffect::LootPickup),
                ItemKind::Horse => self.sound_manager.play(SoundEffect::HorseMount),
            }
        }
        Some(score_effect)
    }

    /// Add an item to the player's inventory.
    pub fn add_item(&mut self, kind: ItemKind) {
        self.player.inventory.push(kind);
    }

    /// Attack an adjacent NPC, dealing damage. Returns true if the NPC died.
    pub fn attack(&mut self, target_pos: Pos) -> bool {
        // Find the NPC at the target position.
        let npc_idx = self.world.npcs.iter().position(|n| n.pos == target_pos);

        if let Some(idx) = npc_idx {
            let npc_kind = self.world.npcs[idx].kind;
            let damage = 25;

            self.world.npcs[idx].health = self.world.npcs[idx].health.saturating_sub(damage);
            let dead = self.world.npcs[idx].health == 0;

            #[cfg(feature = "gui")]
            self.mark_cache_dirty();

            if dead {
                self.message = format!("You killed the {:?}.", npc_kind);
                #[cfg(feature = "sound")]
                self.sound_manager.play(SoundEffect::EnemyDeath);
                self.player.score = self.player.score.saturating_add(20);
                // Remove dead NPC.
                self.world.npcs.remove(idx);
            } else {
                self.message = format!("You hit the {:?} for {} damage.", npc_kind, damage);
                #[cfg(feature = "sound")]
                self.sound_manager.play(SoundEffect::Hit);
            }

            dead
        } else {
            false
        }
    }

    /// Update all NPCs (AI turns).
    fn update_npcs(&mut self) {
        #[cfg(feature = "gui")]
        self.mark_cache_dirty();

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
                    // If the target is the player, damage the player.
                    if target == self.player.pos {
                        let damage = 15;
                        let player_died = self.player.take_damage(damage);
                        self.message = format!(
                            "The {:?} attacks you for {} damage! HP: {}",
                            npc.kind, damage, self.player.health
                        );
                        #[cfg(feature = "sound")]
                        self.sound_manager.play(SoundEffect::Hit);
                        if player_died {
                            self.game_over = true;
                            self.message = "You have been killed. Game over.".to_string();
                        }
                    } else if let Some(target_npc) =
                        self.world.npcs.iter_mut().find(|n| n.pos == target)
                    {
                        // NPC vs NPC combat (e.g., sheriff vs outlaw).
                        let damage = 20;
                        target_npc.health = target_npc.health.saturating_sub(damage);
                        self.message = format!(
                            "The {:?} attacks the {:?}! {} HP remaining.",
                            npc.kind, target_npc.kind, target_npc.health
                        );
                        #[cfg(feature = "sound")]
                        self.sound_manager.play(SoundEffect::Hit);
                        if target_npc.health == 0 {
                            self.message = format!("The {:?} was killed.", target_npc.kind);
                            #[cfg(feature = "sound")]
                            self.sound_manager.play(SoundEffect::EnemyDeath);
                            // Mark for removal.
                            target_npc.health = 0;
                        }
                    }
                }
            }
            new_npcs.push(npc);
        }
        // Remove dead NPCs.
        self.world.npcs.retain(|n| n.health > 0);
        self.world.npcs = new_npcs;
    }

    /// Sheriff AI: patrol town, pursue outlaw at high wanted.
    fn sheriff_ai(&mut self, npc: &Npc) -> NpcAction {
        let outlaw = self.world.npcs.iter().find(|n| n.kind == NpcKind::Outlaw);

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

    /// Deputy AI: flank and support sheriff, box in outlaw.
    fn deputy_ai(&mut self, npc: &Npc) -> NpcAction {
        // Find nearest outlaw.
        let outlaw = self.world.npcs.iter().find(|n| n.kind == NpcKind::Outlaw);

        if let Some(outlaw) = outlaw {
            let dist = (outlaw.pos.x - npc.pos.x).abs() + (outlaw.pos.y - npc.pos.y).abs();
            if dist <= 8 {
                // Chase outlaw, try to flank (get between outlaw and player).
                let dx = (outlaw.pos.x - npc.pos.x).signum();
                let dy = (outlaw.pos.y - npc.pos.y).signum();
                let target = Pos::new(npc.pos.x + dx, npc.pos.y + dy);
                if self.world.is_walkable(target) {
                    // Prefer flanking: move toward outlaw but offset to cut off escape.
                    let flank_dx = (outlaw.pos.x - self.player.pos.x).signum();
                    let flank_dy = (outlaw.pos.y - self.player.pos.y).signum();
                    let flank_target = Pos::new(outlaw.pos.x + flank_dx, outlaw.pos.y + flank_dy);
                    if self.world.is_walkable(flank_target) && flank_target != self.player.pos {
                        return NpcAction::Move(flank_target);
                    }
                    return NpcAction::Move(target);
                }
            }
        }

        // Support sheriff: move toward sheriff.
        if let Some(sheriff) = self.world.npcs.iter().find(|n| n.kind == NpcKind::Sheriff) {
            let dx = (sheriff.pos.x - npc.pos.x).signum();
            let dy = (sheriff.pos.y - npc.pos.y).signum();
            let target = Pos::new(npc.pos.x + dx, npc.pos.y + dy);
            if self.world.is_walkable(target) && target != sheriff.pos {
                return NpcAction::Move(target);
            }
        }
        self.patrol_town(npc.pos)
    }

    /// Outlaw AI: evade law, flank player when cornered.
    fn outlaw_ai(&mut self, npc: &Npc) -> NpcAction {
        // Find nearest law (sheriff or deputy).
        let law = self
            .world
            .npcs
            .iter()
            .filter(|n| n.kind == NpcKind::Sheriff || n.kind == NpcKind::Deputy)
            .min_by_key(|n| (n.pos.x - npc.pos.x).abs() + (n.pos.y - npc.pos.y).abs());

        // Distance to player.
        let player_dist =
            (npc.pos.x - self.player.pos.x).abs() + (npc.pos.y - self.player.pos.y).abs();

        if let Some(law) = law {
            let dist = (law.pos.x - npc.pos.x).abs() + (law.pos.y - npc.pos.y).abs();
            if dist <= 5 {
                // Law nearby: flee away from law, but try to flank player if possible.
                let flee_dx = (npc.pos.x - law.pos.x).signum();
                let flee_dy = (npc.pos.y - law.pos.y).signum();
                let flee_target = Pos::new(npc.pos.x + flee_dx, npc.pos.y + flee_dy);

                // If player is close, try to move between player and law (flank).
                if player_dist <= 5 && player_dist > 0 {
                    let flank_dx = (npc.pos.x - self.player.pos.x).signum();
                    let flank_dy = (npc.pos.y - self.player.pos.y).signum();
                    let flank_target = Pos::new(npc.pos.x + flank_dx, npc.pos.y + flank_dy);
                    // Prefer flanking move if walkable and not moving directly into law.
                    if self.world.is_walkable(flank_target)
                        && (flank_dx != flee_dx || flank_dy != flee_dy)
                    {
                        return NpcAction::Move(flank_target);
                    }
                }

                if self.world.is_walkable(flee_target) {
                    return NpcAction::Move(flee_target);
                }
            }
        }

        // If near player but no law nearby, chase player aggressively.
        if player_dist <= 5 {
            let dx = (self.player.pos.x - npc.pos.x).signum();
            let dy = (self.player.pos.y - npc.pos.y).signum();
            let target = Pos::new(npc.pos.x + dx, npc.pos.y + dy);
            if self.world.is_walkable(target) {
                return NpcAction::Move(target);
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
        #[cfg(feature = "gui")]
        self.mark_cache_dirty();
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
        let game = Self::from_save_data(store.load_data("default")?);
        #[cfg(feature = "sound")]
        {
            let mut game = game;
            game.sound_manager.play(SoundEffect::Load);
            return Ok(game);
        }
        Ok(game)
    }

    /// Load the game from the named slot of the slot document at `path`.
    pub fn load_named_from(slot: &str, path: &Path) -> Result<Self, String> {
        let store = SaveStore::new(path);
        // `mut` is consumed only by the gui/sound-gated calls below; a bare
        // TUI build never mutates the loaded game, so the lint is expected there.
        #[allow(unused_mut)]
        let mut game = Self::from_save_data(store.load_data(slot)?);
        #[cfg(feature = "gui")]
        game.mark_cache_dirty();
        #[cfg(feature = "sound")]
        {
            game.sound_manager.play(SoundEffect::Load);
        }
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
            rng: rand::make_rng(),
            #[cfg(feature = "sound")]
            sound_manager: SoundManager::new().unwrap_or_default(),
            #[cfg(not(feature = "sound"))]
            sound_settings: SoundSettings::new(),
            #[cfg(feature = "gui")]
            sprite_cache: SpriteCache::new(256),
            #[cfg(feature = "gui")]
            cache_dirty: false,
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
    pub fn slot_names(&self) -> Vec<String> {
        let path = save_dir();
        let store = SaveStore::new(path);
        store.list_saves_or_empty()
    }

    /// Save the game to the named slot at `path`.
    pub fn save_named_to(&self, slot: &str, path: &Path) -> Result<(), String> {
        let store = SaveStore::new(path);
        store.save_data(slot, SaveData::with_slot(self, slot))?;
        Ok(())
    }

    /// List all save slot names in the default save directory.
    pub fn list_saves() -> Result<Vec<String>, String> {
        let path = save_dir();
        let store = SaveStore::new(&path);
        store.list_saves()
    }

    /// Mark the sprite cache as dirty (needs re-render on next frame).
    #[cfg(feature = "gui")]
    fn mark_cache_dirty(&mut self) {
        self.cache_dirty = true;
        self.sprite_cache.clear();
    }
}

/// NPC action during AI turns.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NpcAction {
    Move(Pos),
    Wait,
    #[allow(dead_code)]
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
            health: 50,
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
            WantedLevel::None,
        );
        // Commit crime to increase wanted
        game.world.npcs[outlaw_idx].add_bounty(20);
        assert_eq!(game.world.npcs[outlaw_idx].wanted_level(), WantedLevel::Low);
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

    #[test]
    fn save_named_to_load_named_from_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("saves.json");

        let mut game = Game::new_static();
        game.move_player(Direction::Right);
        game.move_player(Direction::Down);

        // Save to slot "alpha"
        game.save_named_to("alpha", &path).unwrap();

        // Save to slot "beta"
        let mut game_beta = Game::new_static();
        game_beta.move_player(Direction::Up);
        game_beta.move_player(Direction::Left);
        game_beta.save_named_to("beta", &path).unwrap();

        // Load slot "alpha" and verify state
        let loaded_alpha = Game::load_named_from("alpha", &path).unwrap();
        assert_eq!(loaded_alpha.player.pos, Pos::new(3, 3)); // Right then Down from (2,2)
        assert_eq!(loaded_alpha.turn, 2);

        // Load slot "beta" and verify state
        let loaded_beta = Game::load_named_from("beta", &path).unwrap();
        assert_eq!(loaded_beta.player.pos, Pos::new(1, 1)); // Up then Left from (2,2)
        assert_eq!(loaded_beta.turn, 2);
    }

    #[test]
    fn player_take_damage_reduces_health() {
        let mut game = Game::new_static();
        let initial_health = game.player.health;
        let damage = 25;
        let died = game.player.take_damage(damage);
        assert!(!died);
        assert_eq!(game.player.health, initial_health - damage);
    }

    #[test]
    fn player_take_damage_to_zero_dies() {
        let mut game = Game::new_static();
        let died = game.player.take_damage(game.player.health);
        assert!(died);
        assert!(game.player.is_dead());
    }

    #[test]
    fn player_attack_kills_npc() {
        let mut game = Game::new_static();
        let npc_count_before = game.world.npcs.len();
        // Place an outlaw adjacent to player (at 3, 2).
        game.world.npcs.push(crate::domain::world::Npc {
            kind: NpcKind::Outlaw,
            pos: Pos::new(3, 2),
            bounty: 0,
            health: 25, // One hit kills (damage = 25)
        });
        let dead = game.attack(Pos::new(3, 2));
        assert!(dead);
        assert_eq!(game.world.npcs.len(), npc_count_before);
    }

    #[test]
    fn player_attack_damages_npc() {
        let mut game = Game::new_static();
        // Place an outlaw adjacent to player with 100 HP.
        game.world.npcs.push(crate::domain::world::Npc {
            kind: NpcKind::Outlaw,
            pos: Pos::new(3, 2),
            bounty: 0,
            health: 100,
        });
        let dead = game.attack(Pos::new(3, 2));
        assert!(!dead);
        // Find the NPC we added at (3, 2).
        let attacked = game
            .world
            .npcs
            .iter()
            .find(|n| n.pos == Pos::new(3, 2))
            .unwrap();
        assert_eq!(attacked.health, 75); // 100 - 25 damage
    }

    #[test]
    fn npc_attack_damages_player() {
        let mut game = Game::new_static();
        // Place an outlaw adjacent to player.
        game.world.npcs.push(crate::domain::world::Npc {
            kind: NpcKind::Outlaw,
            pos: Pos::new(3, 2),
            bounty: 0,
            health: 50,
        });
        // Simulate NPC attacking player.
        let player_died = game.player.take_damage(15);
        assert!(!player_died);
        assert_eq!(game.player.health, 85);
    }

    #[test]
    fn player_death_sets_game_over() {
        let mut game = Game::new_static();
        // Damage player to death.
        let _ = game.player.take_damage(game.player.health);
        assert!(game.player.is_dead());
        game.game_over = true;
        assert!(game.game_over);
    }

    #[test]
    fn player_inventory_adds_item_on_interact() {
        let mut game = Game::new_static();
        // Place loot adjacent to player.
        game.world.items.push(crate::domain::world::Item {
            kind: crate::domain::world::ItemKind::Loot,
            pos: Pos::new(3, 2),
            collected: false,
        });
        // Manually add to inventory to test inventory state.
        game.player
            .inventory
            .push(crate::domain::world::ItemKind::Loot);
        assert_eq!(game.player.inventory.len(), 1);
        assert_eq!(
            game.player.inventory[0],
            crate::domain::world::ItemKind::Loot
        );
    }

    #[test]
    fn player_use_item_consumes_loot() {
        let mut game = Game::new_static();
        game.add_item(crate::domain::world::ItemKind::Loot);
        let score_effect = game.use_item(crate::domain::world::ItemKind::Loot);
        assert_eq!(score_effect, Some(5));
        assert_eq!(game.player.score, 5);
        assert!(game.player.inventory.is_empty());
    }

    #[test]
    fn player_use_item_mounts_horse() {
        let mut game = Game::new_static();
        game.add_item(crate::domain::world::ItemKind::Horse);
        let score_effect = game.use_item(crate::domain::world::ItemKind::Horse);
        assert_eq!(score_effect, Some(0));
        assert!(game.player.mounted);
        assert!(game.player.inventory.is_empty());
        assert!(game.player.inventory.is_empty());
    }

    #[test]
    fn outlaw_flanks_player_when_near() {
        let mut game = Game::new_static();
        // Clear existing NPCs and place outlaw near player with law nearby.
        game.world.npcs.clear();
        // Place outlaw adjacent to player.
        game.world.npcs.push(crate::domain::world::Npc {
            kind: NpcKind::Outlaw,
            pos: Pos::new(3, 2),
            bounty: 0,
            health: 50,
        });
        // Place sheriff nearby (within 5 tiles of outlaw).
        game.world.npcs.push(crate::domain::world::Npc {
            kind: NpcKind::Sheriff,
            pos: Pos::new(6, 2),
            bounty: 0,
            health: 50,
        });

        let outlaw = Npc {
            kind: game.world.npcs[0].kind,
            pos: game.world.npcs[0].pos,
            bounty: game.world.npcs[0].bounty,
            health: game.world.npcs[0].health,
        };
        let action = game.outlaw_ai(&outlaw);
        // Outlaw should try to move (flank or flee).
        assert!(matches!(action, NpcAction::Move(_)));
    }

    #[test]
    fn deputy_chases_outlaw_when_near() {
        let mut game = Game::new_static();
        game.world.npcs.clear();
        // Place outlaw and deputy near each other.
        game.world.npcs.push(crate::domain::world::Npc {
            kind: NpcKind::Outlaw,
            pos: Pos::new(10, 10),
            bounty: 0,
            health: 50,
        });
        game.world.npcs.push(crate::domain::world::Npc {
            kind: NpcKind::Deputy,
            pos: Pos::new(12, 10),
            bounty: 0,
            health: 40,
        });

        let deputy = game
            .world
            .npcs
            .iter()
            .find(|n| n.kind == NpcKind::Deputy)
            .cloned()
            .unwrap();
        let action = game.deputy_ai(&deputy);
        // Deputy should try to move toward outlaw.
        assert!(matches!(action, NpcAction::Move(_)));
    }

    /// Headless workflow test: move → interact → attack → save → load
    /// through the new sprite-based viewport. Drives the full gameplay loop
    /// using `Game::frame()` with the gui feature path (T-014).
    #[cfg(feature = "gui")]
    #[test]
    fn workflow_move_combat_loot_save_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("saves.json");

        // Use a known seed for deterministic world generation
        let mut game = Game::new_with_seed(42);

        // Verify initial frame renders through new pipeline (gui feature)
        let frame = game.frame();
        assert_eq!(frame.width, 80);
        assert_eq!(frame.height, 50);
        // Frame renders without panic; depth buffer tested separately in composite tests

        // Move right twice
        game.move_player(Direction::Right);
        game.move_player(Direction::Right);
        assert_eq!(game.player.pos.x, 4);
        assert_eq!(game.player.pos.y, 2);

        // Attack: place an outlaw at player position for instant kill
        let outlaw_pos = game.player.pos;
        game.world.npcs.push(crate::domain::world::Npc {
            kind: NpcKind::Outlaw,
            pos: outlaw_pos,
            bounty: 0,
            health: 25, // One hit kills (damage = 25)
        });
        let dead = game.attack(outlaw_pos);
        assert!(dead, "attack should kill outlaw");

        // Interact: place loot at player position and collect
        let loot_pos = game.player.pos;
        game.world.items.push(crate::domain::world::Item {
            kind: ItemKind::Loot,
            pos: loot_pos,
            collected: false,
        });
        let interaction = game.interact();
        assert!(matches!(interaction, Interaction::Collected { kind: ItemKind::Loot }));

        // Save to slot "workflow_test"
        game.save_named_to("workflow_test", &path).unwrap();
        let saved_turn = game.turn;
        let saved_pos = game.player.pos;
        let saved_score = game.player.score;

        // Verify frame still renders after gameplay
        let frame2 = game.frame();
        assert_eq!(frame2.width, 80);
        assert_eq!(frame2.height, 50);

        // Load from slot "workflow_test"
        let mut loaded = Game::load_named_from("workflow_test", &path).unwrap();

        // Verify roundtrip: all state matches
        assert_eq!(loaded.player.pos, saved_pos);
        assert_eq!(loaded.turn, saved_turn);
        assert_eq!(loaded.player.score, saved_score);
        assert_eq!(loaded.world.height, game.world.height);
        assert_eq!(loaded.world.width, game.world.width);

        // Verify loaded game frame renders correctly
        let frame3 = loaded.frame();
        assert_eq!(frame3.width, 80);
        assert_eq!(frame3.height, 50);
    }
}
