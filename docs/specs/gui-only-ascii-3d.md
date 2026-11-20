# Section 1 — Title, status, summary

**Spec id:** `gui-only-ascii-3d`
**Version:** 1.0
**Date:** 2026-11-20
**Status:** draft — awaiting owner approval
**Author role:** spec author (subagent spawned by the Dust Trail session)
**Target release:** v2.0.0
**Parent project:** Dust Trail (`apps/dust-trail`)
**Branch:** `opencode/ci`

## Binding decisions (user decisions 1–3)

1. **Renderer:** Procedural 3D primitive rasteriser. Each world object (tree, shrub, character, building, prop) is defined as a set of 3D primitives (trunk cone, canopy ellipsoid, torso box, hat cylinder, limb boxes, etc.). A software rasteriser renders each object into a depth buffer + luminance buffer, then a glyph ramp converts luminance to characters. Objects genuinely look like shaded 3D models and respond to light. Deterministic and headless-testable. Needs a cache layer because it is CPU-heavy.
2. **Camera:** Fixed isometric camera. Keep the existing dimetric/isometric projection; objects gain real height and cast depth. No orbitable perspective camera, no zoom/rotate 3D navigation in this milestone.
3. **v2.0.0 milestone scope:** Playable core. TUI removed, GUI-only entry point, the dynamic ASCII renderer live for terrain, props and characters, and the existing move/combat/loot/save loop working through the new viewport. Proves the whole pipeline end to end. Do NOT expand v2.0.0 to include: the full content set re-authored as 3D ASCII models, day/night lighting, signed installers, SBOM, accessibility rendered-evidence pass, or README screenshots. Those belong to later milestones and are listed as explicitly OUT OF SCOPE below.

## One-paragraph summary

This spec defines the v2.0.0 milestone of Dust Trail: a GUI-only Western ASCII adventure game whose world objects are rendered by a software 3D primitive rasteriser into depth + luminance buffers, converted to characters via a glyph ramp, and composited into the existing ASCII frame with depth-aware occlusion. The TUI frontend and its `crossterm` dependency are removed, leaving the egui/eframe GUI as the sole entry point. The existing move/combat/loot/save loop continues to work through the new viewport, proving the full pipeline end to end.

# Section 2 — Overview and user value

## What the player experiences after this milestone

The player launches Dust Trail as a desktop window (GUI only). The game world is drawn in an isometric ASCII projection where terrain tiles still form a flat ground plane, but objects such as trees, buildings, NPCs, and the player character are rendered as shaded 3D ASCII shapes built from cones, boxes, ellipsoids, and cylinders. Each object responds to a directional light: faces turned toward the light appear brighter, faces turned away appear darker, and the depth buffer ensures nearer objects hide farther ones (a tree in front of the player correctly occludes the player). The same WASD movement, E interaction, F attack, save/load, and combat loop from earlier versions works unchanged through this new viewport. The game remains fully playable from the keyboard.

## Why it matters

This milestone replaces the flat, single-glyph-per-tile view with a genuinely shaded 3D ASCII look. Objects now have height, volume, and lighting response, so the world reads as a scene with depth rather than a painted grid. Because the rasteriser is deterministic and headless-testable, every rendered frame can be verified by CI without a display, and the render cache keeps CPU cost bounded. Removing the TUI simplifies the codebase to a single frontend, reducing maintenance surface and eliminating the `crossterm` dependency.

## What is explicitly NOT in it

- The full content set re-authored as 3D ASCII models (only a minimal model set is built in v2.0.0).
- Day/night lighting cycles or a second lighting direction.
- Signed installers or SBOM artefacts.
- An accessibility rendered-evidence pass (contrast/keyboard tests exist already; rendered snapshot evidence is deferred).
- README screenshots.
- Orbitable perspective camera, zoom, or 3D navigation.

# Section 3 — Scope

## In scope

| # | Capability |
|---|------------|
| 1 | Remove the `tui` module (`src/tui/`) and the `tui` Cargo feature; remove the `crossterm` dependency; gate `lib.rs` modules behind `cfg(feature = "gui")` where applicable. |
| 2 | GUI-only entry point: `main.rs` selects the egui/eframe desktop window when no terminal is attached; when the `gui` feature is off and no terminal is attached, the process exits with a clear message and code 2. |
| 3 | New `render/` module tree under `apps/dust-trail/src/render/` containing `primitive.rs`, `model.rs`, `raster.rs`, `glyph.rs`, `sprite.rs`, `composite.rs`, `cache.rs`, `camera.rs`. |
| 4 | `Primitive` enum with variants cone, ellipsoid, box, cylinder, capsule, each with transform and material. |
| 5 | `ObjectModel` type = a named set of primitives plus a bounding box plus a draw key; a registry mapping each game entity kind (tree, shrub, cactus, rock, water, barn, player, NPC kinds, loot) to a model. |
| 6 | Software rasteriser: project primitives to screen space, fill covered pixels, write per-pixel depth to a z-buffer, apply per-face/per-pixel Lambertian shading with a single directional light. |
| 7 | Glyph ramp: luminance → character mapping via a ramp table (e.g. a denser ramp such as ` .:-=+*#%@`). |
| 8 | `Sprite` type: a width × height grid of `(char, fg, bg)` plus a per-cell depth value and a footprint anchor. |
| 9 | `SpriteCache`: LRU/bounded cache of rendered sprites keyed by (model key, light state, size, orientation), with eviction and invalidation. |
| 10 | Depth-aware compositing: write sprite cells into `Frame`/`Cell` with per-cell depth comparison so nearer cells hide farther cells (replacing unconditional overwrite). Correct draw ordering (back-to-front by screen depth). |
| 11 | Fixed isometric camera: the existing `world_to_screen` / `screen_depth` maths, fixed scale, screen↔world anchor maths; no rotation, no zoom in this milestone. |
| 12 | Terrain rendering: ground plane stays cheap and flat (tiles drawn as single cells); the 3D primitive treatment applies to props and characters. |
| 13 | Frame-cache integration: `Game::frame()` reuses cached sprites instead of re-rasterising every repaint; cache invalidated when world state or light changes. |
| 14 | Save/load compatibility: existing save format keeps working; app version moves to 2.x; `migrate()` extended deliberately; v1.x saves still load. |
| 15 | Headless testability: the render module's tests run without any GUI dependency or display. |
| 16 | Existing move/combat/loot/save loop works through the new viewport (headless workflow test). |
| 17 | Theme-token compliance: all colours come from `theme/`; no raw `Color32::from_*` outside the theme module (CI grep gate). |
| 18 | Graceful degradation: unknown model key → defined placeholder, not a crash; empty/degenerate model → placeholder, not a panic. |

## Out of scope

| # | Capability | Reason | Deferred to |
|---|------------|--------|-------------|
| 1 | Full content set re-authored as 3D ASCII models | Requires art authoring for every entity; v2.0.0 proves the pipeline with a minimal model set | v2.1.0 |
| 2 | Day/night lighting or second lighting direction | Lighting model is out of scope for the playable core | v2.x |
| 3 | Signed installers | Packaging/signing is a release-infrastructure task, not a gameplay feature | v2.x |
| 4 | SBOM | Supply-chain artefact, not required for the playable core | v2.x |
| 5 | Accessibility rendered-evidence pass | Rendered snapshot evidence is deferred per milestone scope | v2.x |
| 6 | README screenshots | Documentation polish, not required for functionality | v2.x |
| 7 | Orbitable perspective camera / zoom / rotate 3D navigation | Binding decision 2: fixed isometric camera only | post-v2.0 |

# Section 4 — System architecture

## Module layout (new modules only)

Per guide §12.2, new commands and logic go into new modules, never edited into an existing module. A new `render/` module tree is created under `apps/dust-trail/src/render/`:

| File | Responsibility |
|------|----------------|
| `primitive.rs` | `Primitive` enum (cone, ellipsoid, box, cylinder, capsule), transform, material. |
| `model.rs` | `ObjectModel` = named set of primitives + bounding box + draw key; registry mapping game entity kinds to models. |
| `raster.rs` | Software rasteriser: project primitives to screen space, fill, z-buffer write, per-face/per-pixel Lambertian shading with directional light. |
| `glyph.rs` | Luminance → glyph ramp mapping and the ramp table. |
| `sprite.rs` | Cached multi-cell ASCII sprite: width × height grid of `(char, fg, bg)` plus per-cell depth and footprint anchor. |
| `composite.rs` | Sprite → `Frame`/`Cell` write with depth-aware occlusion (replaces unconditional overwrite); correct draw ordering. |
| `cache.rs` | LRU/bounded cache of rendered sprites keyed by (model key, light state, size, orientation). |
| `camera.rs` | Fixed isometric camera: projection maths, scale, screen↔world anchor maths. |

All files live under `apps/dust-trail/src/render/`. The `render` module is declared in `lib.rs` behind `#[cfg(feature = "gui")]` and is not compiled into the domain.

## Data flow

```
Game state (domain)  →  object selection (which entities/tiles are visible)
                         →  model lookup (registry: entity kind → ObjectModel)
                           →  cache lookup (SpriteCache keyed by model+light+size+orientation)
                             →  cache hit: reuse cached Sprite
                             →  cache miss: rasterise primitives → Sprite → store in cache
                           →  depth-aware composite (Sprite → Frame/Cell with z-compare)
                         →  Frame
                           →  egui GUI rendering (existing Gui::ui path)
```

## How `Cell` and `Frame` change

- `Cell` remains `ch`/`fg`/`bg` only (no depth field). Depth does NOT live in `Cell`.
- Depth lives in the `Sprite` grid: each sprite cell carries its own depth value (the z-value at that screen pixel, computed during rasterisation).
- The composite step decides occlusion: when writing a sprite cell into `Frame` at screen position (x, y), the composite compares the sprite cell's depth against the depth already stored at that frame position. A separate depth buffer (parallel to the frame grid, same dimensions) tracks the nearest depth written so far. If the new cell's depth is nearer (smaller z), it overwrites; otherwise it is skipped.
- `Frame::set` is kept for backward-compatible flat writes but a new depth-aware method is added: `set_at_depth(x, y, cell, depth)` that consults the frame's depth buffer before overwriting. The depth buffer is reset at the start of each frame build.
- Draw ordering guarantee: objects are sorted back-to-front by screen depth (far first, near last), matching the existing painter's-algorithm convention in `Game::frame()`. The per-cell depth buffer then resolves any remaining overlaps within and between objects.

## Frame write path

`Game::frame()` builds the frame as today (tiles → items → NPCs → player, back-to-front by screen depth), but each object is now emitted as a `Sprite` rather than a single `Cell`. Tiles remain single-cell sprites (flat ground). Objects (trees, buildings, NPCs, player, loot) are multi-cell sprites from the rasteriser. The composite writes each sprite into the frame with depth-aware occlusion. No full rewrite of the frame builder is required: the existing `drawn` Vec of `(sx, sy, Cell)` becomes a Vec of sprite placements, and the final loop calls `set_at_depth` per sprite cell instead of unconditional `cells[row][col] = cell`.

# Section 5 — Interface contracts

All signatures are Rust-like text (not compiled). Errors use `Result` where fallible; unknown/model-missing cases never panic.

## Primitive

```rust
/// A single 3D primitive with a local transform and material.
pub struct Primitive {
    pub kind: PrimitiveKind,
    pub transform: Transform,        // translation, rotation (yaw/pitch/roll), scale
    pub material: Material,          // albedo luminance base, shading side
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveKind {
    Cone,     // trunk / tapered shape
    Ellipsoid,// canopy / rounded mass
    Box,      // torso / building block
    Cylinder, // hat / pillar
    Capsule,  // limb / rounded bar
}

pub struct Transform {
    pub translation: (f32, f32, f32), // (x, y, z) in local object space
    pub rotation: (f32, f32, f32),    // yaw, pitch, roll in radians
    pub scale: (f32, f32, f32),
}

pub struct Material {
    pub albedo: f32,   // base luminance 0.0..=1.0
    pub shade_side: bool, // whether Lambertian shading applies per face
}
```

Contract: `Primitive` is a plain data type; no panic on construction.

## ObjectModel

```rust
/// A named set of primitives forming one renderable object.
pub struct ObjectModel {
    pub key: ModelKey,               // stable identifier (e.g. "tree_oak", "player")
    pub name: String,
    pub primitives: Vec<Primitive>,
    pub bounding_box: BoundingBox,   // min/max in object space
    pub draw_key: DrawKey,           // ordering hint for composite
}

pub type ModelKey = &'static str;

pub struct BoundingBox {
    pub min: (f32, f32, f32),
    pub max: (f32, f32, f32),
}

/// Registry mapping game entity kinds to models.
pub struct ModelRegistry {
    models: Map<EntityKind, ModelKey>,
    definitions: Map<ModelKey, ObjectModel>,
    placeholder: ObjectModel,
}

impl ModelRegistry {
    pub fn new() -> Self;
    /// Register a model under an entity kind. Returns error if key already registered.
    pub fn register(&mut self, kind: EntityKind, model: ObjectModel) -> Result<(), String>;
    /// Look up the model for an entity kind. Returns the placeholder if unknown — never panics.
    pub fn model_for(&self, kind: EntityKind) -> &ObjectModel;
    pub fn placeholder() -> &'static ObjectModel;
}
```

Contract: `model_for` on an unknown kind returns `&placeholder` (a single-cell '?' sprite) and logs at debug level; it never panics.

## Rasteriser

```rust
pub struct Rasteriser {
    light_dir: (f32, f32, f32), // normalized directional light
    width: u32, height: u32,    // raster dimensions in screen cells
}

impl Rasteriser {
    pub fn new(width: u32, height: u32) -> Self;
    pub fn with_light(mut self, dir: (f32, f32, f32)) -> Self;
    /// Rasterise one model into a Sprite. Projects primitives, fills, writes z-buffer,
    /// applies Lambertian shading, converts luminance to glyph. Never panics on
    /// degenerate input; returns placeholder sprite for empty/degenerate models.
    pub fn render(&self, model: &ObjectModel, camera: &Camera) -> Sprite;
}
```

Contract: `render` is deterministic (same model + camera → byte-identical Sprite). Zero-size or empty primitive list → empty sprite with zero width/height, not a panic.

## Sprite

```rust
pub struct Sprite {
    pub key: ModelKey,
    pub width: usize,
    pub height: usize,
    /// Grid of (char, fg, bg) per cell.
    pub cells: Vec<Vec<(char, Rgb, Rgb)>>,
    /// Per-cell depth (z) matching each cell; used by composite for occlusion.
    pub depth: Vec<Vec<f32>>,
    /// Footprint anchor: screen offset of the sprite's origin (bottom-center by convention).
    pub anchor: (i32, i32),
}
```

Contract: `cells` and `depth` have identical dimensions. `anchor` defaults to (0, 0).

## SpriteCache

```rust
pub struct SpriteCache {
    entries: Map<CacheKey, Sprite>,
    order: Vec<CacheKey>, // LRU order
    max_entries: usize,
}

pub struct CacheKey {
    pub model_key: ModelKey,
    pub light_state: LightState,   // quantised light direction + intensity
    pub size: (u32, u32),          // raster size
    pub orientation: Orientation,  // camera orientation (fixed in v2.0.0)
}

impl SpriteCache {
    pub fn new(max_entries: usize) -> Self;
    pub fn get(&self, key: &CacheKey) -> Option<&Sprite>;
    pub fn insert(&mut self, key: CacheKey, sprite: Sprite);
    /// Evict least-recently-used entry when over budget.
    pub fn len(&self) -> usize;
    pub fn clear(&mut self);
}
```

Contract: bounded by `max_entries`; on overflow the LRU entry is evicted. Same key always returns the same sprite (deterministic).

## Camera

```rust
pub struct Camera {
    pub scale: f32,            // fixed isometric scale
    pub anchor: (i32, i32),    // world position mapped to screen origin
}

impl Camera {
    pub fn fixed_isometric() -> Self;
    /// World (wx, wy, wz) → screen (sx, sy, sz). Fixed projection; no rotation/zoom.
    pub fn project(&self, wx: i32, wy: i32, wz: i32) -> (i32, i32, f32);
    /// Screen depth for draw ordering (far = small, near = large).
    pub fn screen_depth(&self, wx: i32, wy: i32, wz: i32) -> f32;
}
```

Contract: fixed projection only; no public method to rotate or zoom in v2.0.0. `project` reproduces the existing `world_to_screen` for wz=0 and adds a height term for wz>0.

## Changed Frame API

```rust
impl Frame {
    pub fn new(width: usize, height: usize) -> Self; // unchanged
    pub fn cell(&self, x: usize, y: usize) -> Option<Cell>; // unchanged
    /// Flat write (unchanged semantics, kept for compatibility).
    pub fn set(&mut self, x: usize, y: usize, cell: Cell);
    /// Depth-aware write: overwrites only if `depth` is nearer than the stored depth
    /// at (x, y); updates the depth buffer. No-op out of bounds.
    pub fn set_at_depth(&mut self, x: usize, y: usize, cell: Cell, depth: f32);
    /// Depth buffer accessor (for tests).
    pub fn depth_at(&self, x: usize, y: usize) -> Option<f32>;
    /// Reset the depth buffer to infinity (called at start of each frame build).
    pub fn clear_depth(&mut self);
    pub fn rows(&self) -> impl Iterator<Item = (usize, &[Cell])>; // unchanged
}
```

Contract: `set_at_depth` never panics out of bounds; it is a no-op. `clear_depth` resets all stored depths to `f32::INFINITY`.

## Error / empty-state contract

- Unknown model key → `ModelRegistry::model_for` returns the placeholder; rasteriser renders the placeholder as a single '?' cell; composite writes it normally. No panic.
- Empty model (no primitives) → `Rasteriser::render` returns an empty sprite (0×0); composite writes nothing. No panic.
- Degenerate/zero-size raster (width or height = 0) → returns empty sprite. No panic.
- All errors are `Result` with `String` or are handled internally with fallback; no `panic!`, `unwrap`, or `expect` in public API functions (per guide §2.6 and the CI lint rule).

# Section 6 — Functional requirements (EARS format)

Each requirement is numbered `FR-###` and written in EARS style: "When <condition>, the system shall <action>."

## TUI removal and module gating

FR-001. When the `tui` feature is removed, the system shall delete `apps/dust-trail/src/tui/` and the `tui` module declaration from `lib.rs`.
FR-002. When the `tui` feature is removed, the system shall remove the `crossterm` dependency from `Cargo.toml`.
FR-003. When the `tui` feature is removed, the system shall ensure `crossterm` appears nowhere in the dependency tree (verified by CI dependency check).
FR-004. When `lib.rs` declares modules, the system shall gate `gui` and `theme` behind `#[cfg(feature = "gui")]` and shall not declare `tui` unconditionally.
FR-005. When the build has no `gui` feature and no terminal is attached, the system shall exit with code 2 and print a clear message directing the user to rebuild with `gui` or run from a terminal.

## GUI-only entry point

FR-006. When `main.rs` runs with the `gui` feature and no terminal is attached, the system shall open the egui/eframe desktop window.
FR-007. When `main.rs` runs with `--gui`, the system shall force the GUI even if a terminal is attached.
FR-008. When `main.rs` runs with `--tui`, the system shall return an error stating the TUI is removed in v2.0.0.
FR-009. When the GUI window is open, the system shall render the game viewport plus toolbar, inventory panel, save/load dialogs, version/release-notes button, and game-over overlay exactly as before.

## Primitive model type

FR-010. When the `Primitive` enum is defined, the system shall provide variants `Cone`, `Ellipsoid`, `Box`, `Cylinder`, `Capsule`.
FR-011. When a `Primitive` is constructed, the system shall require a `Transform` (translation, rotation, scale) and a `Material` (albedo, shade flag).
FR-012. When a `Primitive` is constructed with invalid values, the system shall not panic; it shall store the values as given and let the rasteriser handle degeneracy.

## Entity-to-model registry

FR-013. When the `ModelRegistry` is built, the system shall register a model for each game entity kind: tree, shrub, cactus, rock, water, barn, player, sheriff, deputy, outlaw, loot, horse.
FR-014. When `model_for(kind)` is called for a registered kind, the system shall return the corresponding `ObjectModel`.
FR-015. When `model_for(kind)` is called for an unregistered kind, the system shall return the placeholder model and shall not panic.
FR-016. When the registry is queried twice for the same kind, the system shall return the same model (deterministic).

## Rasteriser: projection, fill, depth, shading

FR-017. When the rasteriser projects a primitive, the system shall map object-space vertices to screen-space cells using the camera's fixed isometric projection.
FR-018. When the rasteriser fills a projected primitive, the system shall mark every screen cell covered by the primitive's footprint.
FR-019. When the rasteriser writes a cell, the system shall write the cell's depth to the z-buffer and keep only the nearest depth per cell.
FR-020. When the rasteriser shades a cell, the system shall apply Lambertian shading: luminance = albedo × max(0, normal · light_dir) for a single directional light.
FR-021. When the rasteriser runs twice on the same model and camera, the system shall produce byte-identical sprites (determinism).
FR-022. When the rasteriser receives an empty model (no primitives), the system shall return an empty 0×0 sprite and shall not panic.
FR-023. When the rasteriser receives a zero-size raster, the system shall return an empty sprite and shall not panic.

## Glyph ramp

FR-024. When the glyph ramp converts luminance to a character, the system shall map luminance 0.0 → the first ramp character (darkest) and luminance 1.0 → the last ramp character (brightest).
FR-025. When luminance increases, the system shall map to a glyph at the same or a later position in the ramp (monotonic mapping).
FR-026. When the ramp table is defined, the system shall include at least 8 distinct characters spanning dark to bright (e.g. ` .:-=+*#%@`).

## Sprite cache

FR-027. When the `SpriteCache` stores a sprite, the system shall key it by (model key, light state, size, orientation).
FR-028. When the cache is full and a new entry is inserted, the system shall evict the least-recently-used entry.
FR-029. When the cache is queried with an existing key, the system shall return the cached sprite without re-rasterising.
FR-030. When the cache is queried with a missing key, the system shall return None.
FR-031. When the cache exceeds its bounded maximum size, the system shall never grow beyond that size (memory bound enforced by eviction).

## Depth-aware compositing and occlusion

FR-032. When a sprite cell is written into the frame, the system shall compare its depth against the stored depth at that screen position and overwrite only if nearer.
FR-033. When a tree sprite is drawn in front of the player sprite, the system shall show the tree's cells and hide the player's cells at overlapping positions.
FR-034. When objects are composited, the system shall draw them back-to-front by screen depth (far first, near last).
FR-035. When the frame is rebuilt, the system shall reset the depth buffer to infinity before the first composite.

## Fixed camera

FR-036. When the camera projects a world position, the system shall use the fixed isometric projection (sx=(wx-wy)*2, sy=wx+wy plus height term) with fixed scale.
FR-037. When the camera is used in v2.0.0, the system shall provide no method to rotate, orbit, or zoom the camera.

## Terrain rendering

FR-038. When terrain tiles are rendered, the system shall draw each tile as a single flat cell on the ground plane (no 3D primitive for tiles).
FR-039. When props and characters are rendered, the system shall draw them as 3D primitive sprites with height and depth.

## Frame-cache integration

FR-040. When `Game::frame()` runs on a repaint, the system shall reuse cached sprites for unchanged objects instead of re-rasterising.
FR-041. When world state or light changes, the system shall invalidate the affected cache entries so the next render reflects the change.

## Save/load compatibility

FR-042. When the app version moves to 2.x, the system shall keep the existing JSON save format readable.
FR-043. When a v1.x save file is loaded, the system shall run `migrate()` and produce a playable game without data loss of version-known fields.
FR-044. When `migrate()` encounters an unknown future version prefix, the system shall not reject the file; it shall load with safe defaults via `#[serde(default)]`.

## Headless testability

FR-045. When the render module's tests run, the system shall require no GUI dependency, no display, and no `DISPLAY` environment variable.

## Gameplay loop through new viewport

FR-046. When the player moves, interacts, attacks, saves, or loads, the system shall update the world and render the new frame through the depth-aware viewport; the move/combat/loot/save loop behaves identically to v1.x.

## Theme-token compliance

FR-047. When the GUI renders colours, the system shall obtain every colour from `theme/` (`Theme::color32(role)`) and shall not use raw `Color32::from_*` outside the theme module.

## Graceful degradation

FR-048. When an unknown model key is encountered during frame build, the system shall render the placeholder ('?') and shall not crash.
FR-049. When a degenerate model is encountered, the system shall render an empty sprite and shall not crash.

# Section 7 — Non-functional requirements

NFR-001. Performance budget: the rasteriser shall render a single object sprite in under 1 ms on the primary platform's CI runner; a full frame with cached sprites shall composite in under 5 ms. These are targets to be measured in CI, not shipping claims.
NFR-002. Cache-hit budget: on a steady-state repaint with no world change, at least 90% of object sprites shall be served from cache (no re-rasterisation). Target to be measured in CI.
NFR-003. Determinism: given identical world state, light, and camera, the system shall produce byte-identical sprite grids and depth maps across runs.
NFR-004. Headless testability: the render module's unit and workflow tests shall run on CI with no display/GPU (no `DISPLAY`, no `xvfb` required for render tests).
NFR-005. Memory bound: the `SpriteCache` shall hold at most a fixed number of entries (configured maximum); unbounded growth is prohibited.
NFR-006. Accessibility — keyboard: every gameplay flow (move, interact, attack, save, load) shall be completable from the keyboard; the ASCII renderer is not the only way state is communicated — state shall also be available as text in the inspector panel / toolbar / message line.
NFR-007. No GUI dependency in core render module: `primitive.rs`, `model.rs`, `raster.rs`, `glyph.rs`, `sprite.rs`, `cache.rs`, `camera.rs` shall not import egui/eframe; only `composite.rs` and the GUI layer depend on the domain `Frame`/`Cell`.
NFR-008. No crash discipline: public API functions shall not use `panic!`, `unwrap`, `expect`, `unreachable!`, `todo!`, or `unimplemented!` (guide §2.6).
NFR-009. Saturation: all world-coordinate and depth arithmetic derived from input shall use checked or saturating operators; bounded recursion depth is enforced in the rasteriser.

# Section 8 — Acceptance criteria (Given/When/Then)

Each criterion is numbered `AC-###` and is objectively checkable in CI.

## TUI removal

AC-001. Given the `tui` feature is removed, When `cargo doc --no-deps` runs on the workspace, Then `apps/dust-trail/src/tui/` does not exist and `crossterm` is absent from `Cargo.toml` and `Cargo.lock`.
AC-002. Given a clean checkout, When CI runs the dependency check, Then `crossterm` appears in no dependency manifest or lockfile entry.

## GUI-only entry point

AC-003. Given a build without the `gui` feature and no terminal attached, When `main` runs, Then it exits with code 2 and prints a message about rebuilding with `gui` or running from a terminal.
AC-004. Given a build with the `gui` feature, When `main` runs with no terminal attached, Then it opens the egui/eframe desktop window (verified by CI build succeeding; runtime window checked via artefact run).

## Headless render tests

AC-005. Given a headless test module in `render/`, When a tree `ObjectModel` is rasterised, Then the resulting `Sprite` is non-empty (width > 0 and height > 0), its `cells` grid and `depth` grid have identical dimensions, and at least one cell is a non-space character.
AC-006. Given a tree sprite and a player sprite, When both are composited into a frame with the tree nearer than the player at overlapping positions, Then the overlapping cells show the tree's glyph/depth and the player's cells at those positions are hidden.
AC-007. Given the same world state and camera, When `Rasteriser::render` is called twice, Then the two sprites are byte-identical (cells, depth, anchor all equal).
AC-008. Given a cache with one entry, When the same cache key is queried twice, Then the second call returns the cached sprite and the rasteriser is invoked only once (verified by a call-count instrumented rasteriser in test).
AC-009. Given an unregistered model key, When `ModelRegistry::model_for` is called, Then it returns the placeholder model and does not panic.
AC-010. Given a v1.x save file, When `SaveData::into_game` runs `migrate()` then `Game::from_save_data`, Then it produces a playable game with health restored to safe defaults and no error.
AC-011. Given the existing gameplay loop, When a headless test drives move → interact → attack → save → load, Then all operations succeed and the reloaded state matches the saved state (roundtrip).
AC-012. Given the glyph ramp table, When luminance values from 0.0 to 1.0 are mapped, Then the output glyph index is non-decreasing (monotonic) and spans at least 8 distinct characters.
AC-013. Given the render module sources, When CI runs the grep gate, Then no `Color32::from_*` constructor appears outside `src/theme/`.
AC-014. Given the render module sources, When CI runs clippy with `-D warnings`, Then the render crate compiles with zero warnings.
AC-015. Given `main.rs`, When no terminal is attached and the `gui` feature is enabled, Then the entry point selects the GUI (verifiable by inspecting the `use_gui` boolean logic under `#[cfg(feature = "gui")]`).

## Frame depth-aware API

AC-016. Given a `Frame` with a depth buffer, When `set_at_depth` writes a cell with depth nearer than the stored depth, Then the cell is overwritten and `depth_at` returns the new depth; when the new depth is farther, Then the cell is not overwritten.
AC-017. Given a `Frame`, When `clear_depth` is called, Then all `depth_at` positions return `f32::INFINITY` (or a sentinel indicating infinity).

## CI gates

AC-018. Given the PR, When CI runs `cargo fmt --all -- --check`, Then the render module sources are formatted.
AC-019. Given the PR, When CI runs `cargo test --workspace`, Then all render module tests pass headlessly with no display.
AC-020. Given the PR, When CI runs the full test suite, Then the existing move/combat/loot/save domain tests still pass unchanged.

# Section 9 — Error-handling table

| Condition | Detection | User-visible behaviour | Log/telemetry | Test |
|-----------|-----------|------------------------|---------------|------|
| Unknown model key | `ModelRegistry::model_for` returns placeholder | Renders '?' placeholder cell; gameplay continues | Debug log entry | AC-009 |
| Empty model (no primitives) | `Primitive` list is empty | Returns empty 0×0 sprite; composite writes nothing | Debug log | AC-005 (empty case) |
| Degenerate / zero-size raster | width or height == 0 | Returns empty sprite; no crash | Debug log | AC-005 (degenerate case) |
| Cache overflow | `len() == max_entries` on insert | Evicts LRU entry; new sprite stored | Debug log on evict | FR-028 / NFR-005 (measured) |
| Cache key collision | Two distinct models hash to same key | Handled by map equality on full key (not hash-only); no wrong sprite returned | None (internal) | AC-008 (keyed lookup) |
| World coordinates outside viewport | `project` yields screen position outside frame bounds | Composite skips out-of-bounds cells (`set_at_depth` no-op) | None | AC-006 / composite bounds test |
| Save file from unknown future version | `version` does not match `0.`, `1.0.`, `1.1.`, or `2.` | Loads with `#[serde(default)]` safe defaults; no rejection | Info log | FR-044 / AC-010 |
| Save file corrupt | `serde_json::from_str` fails | Load error shown to user ("Load failed: ...") | Error log | Existing save error tests |
| GUI feature not compiled in | `cfg(feature = "gui")` false | Exits with code 2 and message; no GUI attempted | None | AC-003 |
| Rasteriser budget exceeded | Single sprite render > 1 ms budget | Renders anyway; budget is a CI-measured target, not a hard failure | Warn log if instrumented | NFR-001 (measured target) |

# Section 10 — Data and state model

## Save format version vs app version

- The app Cargo version moves to `2.0.0` for this milestone.
- `SaveData.version` records `env!("CARGO_PKG_VERSION")` at save time, e.g. `"2.0.0"`.
- The on-disk JSON save format (fields: `version`, `slot`, `world`, `player`, `turn`, `sound_settings`) is unchanged from v1.x; no new fields are added in v2.0.0.

## What changes and what does not

- Changes: app version string bumped to 2.x; `migrate()` extended to recognize the `2.` prefix as current (no migration needed for 2.x saves).
- Does not change: the JSON field set, `World`, `Player`, `Npc`, `Item` structures, `slot` field semantics, `turn` field.

## How `migrate()` is extended

Current `migrate()` keys off version prefixes `0.`, `1.0.`, `1.1.` and restores health to safe defaults. The v2.0.0 extension:
- Adds a branch for the `2.` prefix that applies no field changes (2.x saves are current).
- Keeps the existing `0.` / `1.0.` / `1.1.` branches unchanged so v1.x saves still migrate.
- For any other prefix (unknown future version), applies no field changes and relies on `#[serde(default)]` for missing keys (loads with safe defaults).

## Backward compatibility rules

- A v1.x save file must load and produce a playable game (AC-010).
- A v2.0.0 save file must load in v2.0.0 without migration changes.
- The on-disk format must NOT: add required fields without defaults, remove fields, rename fields, or change the `version` field semantics.

## What the on-disk format must NOT do

- Must not introduce a new required top-level field.
- Must not rename `world`, `player`, `turn`, `slot`, or `version`.
- Must not change `SoundSettings` serialization in a way that breaks `#[serde(default)]`.

# Section 11 — Test strategy

## Headless test matrix

| Test module | Test kind | What it asserts | Runs without GUI? |
|-------------|-----------|-----------------|-------------------|
| `render/primitive.rs` | unit | `Primitive` construction, no panic on invalid values | Yes |
| `render/model.rs` | unit | registry lookup returns placeholder for unknown kind; deterministic lookup | Yes |
| `render/raster.rs` | unit | projection, fill, z-buffer nearest-wins, Lambertian shading monotonicity, determinism (two calls byte-identical), empty model → 0×0 sprite, zero-size raster → empty sprite | Yes |
| `render/glyph.rs` | unit | ramp monotonicity, endpoints map to first/last char, ≥8 distinct chars | Yes |
| `render/sprite.rs` | unit | cells/depth grids identical dimensions, anchor default | Yes |
| `render/cache.rs` | unit | LRU eviction on overflow, hit returns stored sprite, miss returns None, bounded size | Yes |
| `render/camera.rs` | unit | `project` reproduces `world_to_screen` for wz=0, adds height term for wz>0, fixed scale | Yes |
| `render/composite.rs` | unit | depth-aware overwrite (nearer wins), farther-wins skip, clear_depth resets to infinity, out-of-bounds no-op | Yes |
| `domain/frame.rs` | unit | `set_at_depth` / `depth_at` / `clear_depth` behavior | Yes |
| `domain/game.rs` | workflow | move → interact → attack → save → load roundtrip through frame() | Yes |
| `domain/save.rs` | unit | v1.x save loads and migrates; unknown future version loads with defaults | Yes |

## How rendering is asserted (no pixel baselines)

Rendering is asserted on structural properties only:
- The character grid (`cells`) — non-empty for a real model, correct glyph placement, monotonic glyph ramp.
- The depth map (`depth`) — identical dimensions to cells, nearest-wins semantics, farther cells not overwritten.
- Structural properties — byte-identical output across two runs (determinism), placeholder for unknown model, empty sprite for empty model.
- NEVER a golden PNG or pixel-baseline comparison (per guide II.30.8: no pixel baselines, they fail on font-metric changes).

## CI gates that must be green

- `cargo fmt --all -- --check` (render sources formatted)
- `cargo clippy --all-targets -- -D warnings` (zero warnings; render crate included)
- `cargo test --workspace` (all render unit tests + domain workflow tests pass headlessly)
- Grep gate: no `Color32::from_*` outside `src/theme/`
- Dependency check: `crossterm` absent from the dependency tree

# Section 12 — Implementation TODO checklist

Dependency-ordered work items. Each item is independently CI-verifiable. Order:
renderer core → model registry → composite → gameplay loop integration → TUI removal LAST (so the game stays playable throughout).

| # | Title | Target file paths | Blocking dependencies | Ticket placeholder |
|---|-------|-------------------|-----------------------|--------------------|
| 1 | Add `render/` module tree and declare module in `lib.rs` behind `gui` feature | `apps/dust-trail/src/render/mod.rs`, `lib.rs` | None | `docs/tickets/T-001.md` |
| 2 | Implement `Primitive`, `Transform`, `Material` types | `render/primitive.rs` | #1 | `docs/tickets/T-002.md` |
| 3 | Implement `ObjectModel`, `ModelRegistry`, placeholder model | `render/model.rs` | #2 | `docs/tickets/T-003.md` |
| 4 | Implement `Camera` (fixed isometric projection + height term) | `render/camera.rs` | #1 | `docs/tickets/T-004.md` |
| 5 | Implement glyph ramp table + luminance→glyph mapping | `render/glyph.rs` | #1 | `docs/tickets/T-005.md` |
| 6 | Implement `Sprite` type (cells + depth + anchor) | `render/sprite.rs` | #1 | `docs/tickets/T-006.md` |
| 7 | Implement software rasteriser (project, fill, z-buffer, Lambertian) | `render/raster.rs` | #2, #4, #5 | `docs/tickets/T-007.md` |
| 8 | Implement `SpriteCache` (bounded LRU) | `render/cache.rs` | #6 | `docs/tickets/T-008.md` |
| 9 | Extend `Frame` with depth buffer + `set_at_depth`/`depth_at`/`clear_depth` | `domain/frame.rs` | None | `docs/tickets/T-009.md` |
| 10 | Implement depth-aware composite (sprite → Frame with z-compare, back-to-front order) | `render/composite.rs` | #6, #9 | `docs/tickets/T-010.md` |
| 11 | Wire `Game::frame()` to emit sprites + use cache + composite | `domain/game.rs`, `render/` | #3, #7, #8, #10 | `docs/tickets/T-011.md` |
| 12 | Register entity→model mappings (tree, shrub, cactus, rock, barn, player, NPCs, loot, horse) | `render/model.rs` or `domain/game.rs` | #3, #11 | `docs/tickets/T-012.md` |
| 13 | Headless unit tests for all render modules | `render/*/tests` or inline `#[cfg(test)]` | #2–#10 | `docs/tickets/T-013.md` |
| 14 | Headless workflow test: move/combat/loot/save loop through new viewport | `domain/game.rs` tests | #11 | `docs/tickets/T-014.md` |
| 15 | Save/load v1.x migration test for v2.x | `domain/save.rs` tests | None | `docs/tickets/T-015.md` |
| 16 | CI grep gate: no `Color32::from_*` outside `theme/` | CI workflow + `.ci-required.txt` | None | `docs/tickets/T-016.md` |
| 17 | Remove `tui` module and `tui` feature; remove `crossterm` dependency | `src/tui/` deletion, `lib.rs`, `Cargo.toml` | #11 (gameplay proven through GUI) | `docs/tickets/T-017.md` |
| 18 | GUI-only entry point behavior (exit code 2 when no gui + no terminal) | `main.rs` | #17 | `docs/tickets/T-018.md` |
| 19 | Bump app version to 2.0.0; extend `migrate()` for `2.` prefix | `Cargo.toml`, `domain/save.rs` | #15 | `docs/tickets/T-019.md` |
| 20 | Full CI run: fmt, clippy -D warnings, test, build (primary OS) | `.github/workflows/ci.yml` | #1–#19 | `docs/tickets/T-020.md` |

# Section 13 — Risks and mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| CPU cost of rasterising every frame | Frame latency; unplayable if too slow | Cache sprites (FR-027–031); performance budget measured in CI (NFR-001, NFR-002) |
| Occlusion errors when sprites overlap | Wrong object hides another (e.g. player hidden behind tree incorrectly) | Per-cell depth buffer (FR-032–035); occlusion unit test (AC-006) |
| Save-format breakage across the 2.x version jump | v1.x saves fail to load | Explicit `2.` prefix branch in `migrate()` + v1.x branches kept; round-trip test (FR-042–044, AC-010) |
| Removing the TUI removes a working fallback | No way to play if GUI fails to initialize | TUI removed only after GUI path proven (T-017 lands after T-011); clear exit-code-2 message (AC-003) |
| Glyph readability at small sizes | Shaded ASCII hard to distinguish | Ramp with ≥8 distinct chars (FR-026); minimum sprite footprint via anchor + scale | None |
| Rasteriser determinism bugs | Same world renders differently across runs | Determinism test (FR-021, AC-007); byte-identical assertion |
| Depth buffer z-fighting at equal depths | Flicker between equal-depth cells | Tie-break: keep existing (do not overwrite on equal depth) |

# Section 14 — Open questions for the owner

Exactly the questions that need the owner's answer. Recommended answer given first for each.

1. **Glyph density preference:** Should the ramp be the denser 10-char ramp ` .:-=+*#%@` or a sparser 6–8 char ramp? Recommended: 10-char ramp ` .:-=+*#%@` for better shading gradation.
2. **Player character detail:** Should the player be a simple silhouette (single box/capsule) or a detailed multi-part model (torso box + hat cylinder + limb boxes)? Recommended: start with a simple box+hat model in v2.0.0; detail later.
3. **Second lighting direction:** Should the renderer support a second (fill/ambient) light direction later in v2.x, or stay single directional light for v2.0.0? Recommended: single directional light in v2.0.0; add fill light in v2.x if needed.
4. **Minimum frame rate target:** What is the minimum acceptable frame rate on the primary platform (Linux x86_64 CI runner)? Recommended: 30 fps equivalent (full frame composite + raster under ~33 ms) as a CI-measured target.
5. **Inspector panel ASCII:** Should the ASCII renderer also replace the TUI-era look in the in-game inspector panel, or only the main viewport? Recommended: main viewport only in v2.0.0; inspector panel stays text-based.

# Section 15 — Traceability

| FR ID | Verified by AC ID(s) | Ticket placeholder |
|-------|----------------------|--------------------|
| FR-001 | AC-001 | T-017.md |
| FR-002 | AC-001 | T-017.md |
| FR-003 | AC-002 | T-017.md |
| FR-004 | AC-001 | T-017.md |
| FR-005 | AC-003 | T-018.md |
| FR-006 | AC-004 | T-018.md |
| FR-007 | AC-004 | T-018.md |
| FR-008 | AC-001 (TUI removed) | T-017.md |
| FR-009 | AC-004 | T-018.md |
| FR-010 | AC-005 (render test) | T-002.md |
| FR-011 | AC-005 | T-002.md |
| FR-012 | AC-005 (no panic) | T-002.md |
| FR-013 | AC-009 (registry) | T-003.md |
| FR-014 | AC-009 | T-003.md |
| FR-015 | AC-009 | T-003.md |
| FR-016 | AC-009 | T-003.md |
| FR-017 | AC-005 | T-007.md |
| FR-018 | AC-005 | T-007.md |
| FR-019 | AC-005 | T-007.md |
| FR-020 | AC-005 | T-007.md |
| FR-021 | AC-007 | T-007.md |
| FR-022 | AC-005 (empty) | T-007.md |
| FR-023 | AC-005 (zero-size) | T-007.md |
| FR-024 | AC-012 | T-005.md |
| FR-025 | AC-012 | T-005.md |
| FR-026 | AC-012 | T-005.md |
| FR-027 | AC-008 | T-008.md |
| FR-028 | AC-008 | T-008.md |
| FR-029 | AC-008 | T-008.md |
| FR-030 | AC-008 | T-008.md |
| FR-031 | AC-008 | T-008.md |
| FR-032 | AC-016 | T-010.md |
| FR-033 | AC-006 | T-010.md |
| FR-034 | AC-006 | T-010.md |
| FR-035 | AC-017 | T-009.md |
| FR-036 | AC-005 (camera) | T-004.md |
| FR-037 | AC-005 (camera) | T-004.md |
| FR-038 | AC-005 | T-011.md |
| FR-039 | AC-005 | T-011.md |
| FR-040 | AC-008 | T-011.md |
| FR-041 | AC-008 | T-011.md |
| FR-042 | AC-010 | T-019.md |
| FR-043 | AC-010 | T-019.md |
| FR-044 | AC-010 | T-019.md |
| FR-045 | AC-019 | T-013.md |
| FR-046 | AC-011 | T-014.md |
| FR-047 | AC-013 | T-016.md |
| FR-048 | AC-009 | T-003.md |
| FR-049 | AC-005 | T-007.md |

No orphan requirements: every FR-001 through FR-049 maps to at least one AC. No orphan acceptance criteria: every AC-001 through AC-020 verifies at least one FR.




