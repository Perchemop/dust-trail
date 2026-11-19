# Changelog - Dust Trail

## v1.5.1 - 2026-11-20

### Fixed
- Release pipeline: removed the duplicate `publish.yml` workflow. Its unquoted `on:` YAML key made the whole workflow invalid, and it was the reason the v1.5.0 release published with no installers at all.
- Release pipeline: fixed the GUI compile error that failed every one of the four release build jobs for v1.5.0.
- Release pipeline: the release job now fails if any installer is missing instead of publishing a partial release, and `SHA256SUMS.txt` is generated only from the installers, using bare filenames so `sha256sum -c SHA256SUMS.txt` works.
- Application version: the manifest said `0.2.0` while the published tags said `v1.5.0`. The version is now `1.5.1` and the manifest, the lockfile, the installer package metadata and the release notes all agree.
- Linux package metadata: the `.deb` version is derived from the release tag instead of a hardcoded `0.1.0`.
- In-app release notes popup: the changelog parser only understood bracketed headings, so the popup showed no releases at all against this changelog's `## v1.5.0 - date` format.
- In-app release notes popup: the token ledger is a Markdown table but was parsed as JSON, so the cumulative token figure always read zero. It now reads the real measured numbers.
- Changelog parsing: a prose `##` heading such as `## Known limitations` is no longer mistaken for a release.

### Changed
- The GUI is now a default feature. Every build of the package, including the installers produced by the release pipeline, ships a window-capable binary.
- Launch behaviour: the desktop window opens when the app is started without a terminal attached, which is what the macOS app bundle and the Linux launcher do. `--gui` forces the window and `--tui` forces the terminal interface.
- Release workflow: the artifact download action is pinned to a full commit SHA, and the workflow no longer requests pull request write permission it does not use.
- CI builds the TUI-only configuration as well as the default one, so the fallback path stays warning-free.

### Known limitations
- The terminal interface and the desktop window are both still present in this release. The terminal interface is removed in v2.0.0.
- Release artefacts are not yet signed, and no SBOM is published yet.

## v1.5.0 - 2026-11-18

### Added
- Smarter outlaw AI: flanking behavior when cornered by law, chase player when no law nearby
- Deputy AI: chases outlaw, flanks to block escape routes
- GUI toolbar inventory count
* TUI: 'u' key uses first item from inventory

### Known limitations
- This release published with no installers. The release workflow failed on a GUI compile error and a duplicate, invalid workflow file. v1.5.1 fixes both.

## v1.4.0 - 2026-11-18

### Added
- Item system: inventory, `Game::use_item(kind)` with arbitrary item kinds
- Loot consumption: +5 score per loot item used
- Horse mount: use horse item to mount (same as 'M' toggle)
- GUI toolbar inventory count display
- TUI: 'u' key uses first item from inventory

### Changed
- Collected items go to inventory instead of auto-scoring
- Use items explicitly with 'u' key (TUI/GUI)

### CI & Release
- CI green: format, clippy, tests, multi-OS release builds (Linux arm64/x86_64, macOS arm64, Windows x86_64)
- Installers published: .deb (arm64/x86_64), .dmg (arm64), .msi (x86_64), SHA256SUMS.txt

## v1.3.0 - 2026-11-18

### Added
- GUI status bar: health bar (HP), wanted level indicator, mount status
- Player mount/dismount toggle (M key in TUI/GUI)
- Game over screen with restart/quit options
- Save format version migration for older save files

### Changed
- GUI toolbar now displays wanted level (from outlaw bounty) with color coding
- GUI toolbar now displays player health (HP/100)
- TUI: 'M' key toggles mounted/dismounted status

### CI & Release
- CI green: format, clippy, tests, multi-OS release builds (Linux arm64/x86_64, macOS arm64, Windows x86_64)
- Installers published: .deb (arm64/x86_64), .dmg (arm64), .msi (x86_64), SHA256SUMS.txt

## v1.2.0 - 2026-11-18

### Added
- Player health (HP) — default 100, damaged by NPC attacks, death = game over
- NPC health (HP) — sheriff 50 HP, deputy 40 HP, outlaw 50 HP, removed from world at 0 HP
- Player attack action (F key in TUI/GUI to attack adjacent NPCs)
- Combat feedback messages in game log
- Death states: player death = game over, NPC death = removed from world
- Combat sound effects (hit, enemy death)

### Changed
- NPC Attack action now damages the player instead of just increasing bounty
- NPC vs NPC combat (e.g., sheriff vs outlaw) with damage and death

### CI & Release
- CI green: format, clippy, tests, multi-OS release builds (Linux arm64/x86_64, macOS arm64, Windows x86_64)
- Installers published: .deb (arm64/x86_64), .dmg (arm64), .msi (x86_64), SHA256SUMS.txt

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

## v0.2.0 - 2026-11-15

### Added
- 3D isometric ASCII rendering with per-cell lighting/shading
- 3D world with height (z) - raised plateau, river, buildings, trees
- GUI-only desktop window (egui/eframe 0.37)
- WASD movement, height-clamped stepping
- Save/load (single slot, 3D state)

## v0.1.0 - 2026-11-15

### Added
- Playable ASCII Western adventure (top-down TUI + GUI)
- Turn-based movement, NPC interaction, loot/horse collection
- Save/load (single slot)
- Crash-report panic hook, update-check stub
- CI: format, clippy, tests, multi-OS release builds