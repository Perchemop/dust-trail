# Changelog - Dust Trail

## v1.1.0 - 2026-11-18

### Added
- Multiple named save slots — `Game::save_named_to(slot, path)` with arbitrary slot names
- `Game::list_saves()` returns all available save slots
- GUI slot selection UI — selectable slot list in save/load dialogs
- TUI slot selection — keyboard-navigable slot list (1-9 keys, Esc to cancel)
- Save slot naming — user can name each save slot when saving

### Changed
- TUI save/load now prompts for slot selection instead of hardcoded "savegame"

### Tests
- Multi-save slot roundtrip test: save to "alpha" and "beta", load each and verify state

## v1.0.0 - 2026-11-18

### Added
- Sound effects (optional, feature-gated) — rodio + 10 embedded WAV files
- Accessibility: contrast unit tests (4.5:1 body text, 3:1 UI boundaries, light + dark themes)
- Keyboard focus and AccessKit labels for screen-reader semantics
- In-app release-notes popup + version display (suite standard)
- Auto-update download + signature verification — SHA-256 + minisign, anti-loop marker
- Procedural map generation — deterministic world generation from seed, connectivity guarantees
- Stable save format with version migration

### Changed
- Domain layer defaults to procedurally generated world
- Outlaw AI with Patrol/Chase/Flee states
- Deputy coordination with sheriff flanking behavior

### Fixed
- Connectivity guarantees ensure player start connects to sheriff, outlaw, and loot
- Wanted level calculation and bounty mechanics
- Deputy spawning based on wanted level

### Tests
- 7 headless procgen property tests (determinism, different seeds, connectivity, border walls, walkable start, retries)
- Bounty crimes, decay, wanted levels, deputy spawning, bounty collection tests

### Performance
- Map generation targets <10ms for 40×22 map

### CI & Release
- CI green: format, clippy, tests, multi-OS release builds (Linux arm64/x86_64, macOS arm64, Windows x86_64)
- Installers published: .deb (arm64/x86_64), .dmg (arm64), .msi (x86_64), SHA256SUMS.txt

## v0.3.0 - 2026-11-16

### Added
- **Procedural map generation** - deterministic world generation from seed
- `World::generate(seed)` method - generates complete 40×22 world with terrain, NPCs, items
- `Game::new_with_seed(seed)` - creates game with reproducible procedural world
- `Game::new()` - creates game with random procedural world

### Changed
- Domain layer now uses procedural generation as default world generation method
- Updated `Game::new()` and `Game::new_with_seed()` to use `World::generate()`

### Fixed
- Connectivity guarantees ensure player start (2,2) connects to sheriff, outlaw, and loot via walkable tiles
- Retry logic (up to 10 attempts) for robustness

### Tests
- Added 7 headless property tests in `procgen.rs`:
  - `generate_is_deterministic`
  - `generate_different_seeds_differ`
  - `player_start_connected_to_sheriff`
  - `player_start_connected_to_outlaw`
  - `player_start_connected_to_loot`
  - `generated_world_has_border_walls`
  - `generated_world_has_player_start_tile_walkable`
  - `connectivity_retries_succeed`

### Performance
- Generation targets <10ms for 40×22 map
- Well under the 100ms CI gate

## v0.4.0 - 2026-11-16

### Added
- Wanted Level / Bounty system
- Deputy NPC kind and AI
- Bounty decay and wanted level scaling

### Changed
- Outlaw AI with Patrol/Chase/Flee states
- Deputy coordination with sheriff flanking behavior

### Fixed
- Bounty collection and decay mechanics
- Wanted level calculation (None/Low/Medium/High)
- Deputy spawning based on wanted level

### Tests
- Comprehensive tests for bounty crimes, decay, wanted levels
- Deputy spawning and bounty collection tests
- Tests for bounty only affecting outlaws

---

v0.2.0 - 2026-11-16
- 3D isometric ASCII rendering with per-cell lighting/shading
- 3D world with height (z) - raised plateau, river, buildings, trees
- GUI-only desktop window (egui/eframe 0.37)
- WASD movement, height-clamped stepping
- Save/load (single slot, 3D state)

v0.1.0 - 2026-11-16
- Playable ASCII Western adventure (top-down TUI + GUI)
- Turn-based movement, NPC interaction, loot/horse collection
- Save/load (single slot)
- Crash-report panic hook, update-check stub
- CI: format, clippy, tests, multi-OS release builds