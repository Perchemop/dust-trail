# Changelog - Dust Trail

## v2.0.0 — In Progress (GUI-only ASCII 3D)

### Added
- Render module: 3D primitive types (Box, Cone, Ellipsoid, Cylinder, Capsule) with Transform and Material
- ObjectModel and ModelRegistry mapping EntityKind to procedural models
- Fixed isometric Camera with world→screen projection and screen_depth for painter's algorithm
- Luminance-to-glyph ramp (8-step ASCII gradient) for shaded surfaces
- Sprite struct with cells, depth buffer, anchor, and key
- SpriteCache: bounded LRU cache keyed by model, light, size, orientation
- Rasteriser: software projection, z-buffer, Lambertian shading per primitive kind, renders to Sprite
- Depth-aware compositing (composite_sprite, composite_sprites_back_to_front) with per-cell z-compare
- Frame depth buffer (z per cell, set_at_depth, clear_depth)
- Tile.to_sprite() — single-cell flat ground sprites for terrain
- Game::frame() integration: new render pipeline when `gui` feature enabled, flat fallback for headless tests; SpriteCache held by Game, invalidated on world state changes

### Changed
- CI: clippy -D warnings now passes with zero warnings across all targets
- Build: TUI and GUI configurations both lint and test clean

### Removed (planned for v2.0.0)
- Terminal interface (TUI feature flag) — removed in v2.0.0 final

### Known limitations
- The terminal interface is still present in this development state. v2.0.0 will remove it entirely.
- egui/eframe GUI frontend not yet wired (T-012 through T-020)

## v1.5.1 - 2026-11-20

### Fixed
- Build: 19 compile errors in the library test target fixed — `Rgb` struct literals in render tests, `&'static str` model-key lifetimes in the sprite-cache tests, a clone-after-move in the model-registry test, unused imports, and redundant integer casts. CI run `44965810348` is the first fully green run of this branch.
- Build: the sprite-cache test helpers now take `&'static str` keys, matching the `ModelKey` type.
- Render tests: the Lambertian shading test now exercises two separated primitives with different fixed normals (side-facing box, top-facing ellipsoid), matching the documented per-primitive-kind shading model. The previous fixture required per-pixel normals, which the rasteriser does not implement yet.
- Agent ruleset: `AGENTS.md` re-synced with build guide 2026-11-22 — runtime renamed to DeepSeek Harness (DSH), the DSH workflow protocol (§5.6), multi-agent protocol (§5.7), Dev-identity, personal-data scan, Rust GUI/UX, skills, and catalog sections added.
- Release pipeline: removed the duplicate `publish.yml` workflow. Its unquoted `on:` YAML key made the whole workflow invalid, and it was the reason the v1.5.0 release published with no installers at all.
- Release pipeline: fixed the GUI compile error that failed every one of the four release build jobs for v1.5.0.
- Release pipeline: the release job now fails if any installer is missing instead of publishing a partial release, and `SHA256SUMS.txt` is generated only from the installers, using bare filenames so `sha256sum -c SHA256SUMS.txt` works.
- Application version: the manifest said `0.2.0` while the published tags said `v1.5.0`. The version is now `1.5.1` and the manifest, the lockfile, the installer package metadata and the release notes all agree.
- Linux package metadata: the `.deb` version is derived from the release tag instead of a hardcoded `0.1.0`.
- In-app release notes popup: the changelog parser only understood bracketed headings, so the popup showed no releases at all against this changelog's `## v1.5.0 - date` format.
- In-app release notes popup: the token ledger is a Markdown table but was parsed as JSON, so the cumulative token figure always read zero. It now reads the real measured numbers.
- Changelog parsing: a prose `##` heading such as `## Known limitations` is no longer mistaken for a release.

### Changed
- Dependencies: `rodio` 0.19 → 0.23 (optional `sound` feature). Adopted from dependabot PR #28, rebased onto current `main` and re-authored from the Dev account per guide II.26.7; CI run verified all eight required checks green.
- CI: the personal-data/secret scan moved out of the `check` job into two named required checks — `personal-data-scan` (guide II.29.2 patterns incl. Windows profile paths) and `secret-scan` (token/secret patterns) — and a `pr-author-is-dev` check verifies every PR is opened by the Dev account (guide II.26.7). All three are in `.ci-required.txt`.
- Repository: guide-mandated scaffolding added — `.dsh/` layout (7 command files, 5 agent definitions, `skill/`), `.gitleaks.toml`, `contributors/people.toml` (schema only, no real names), `docs/team-log/`, `docs/reference/index.md`, `docs/design/brand.md`; the stale `.opencode/` tree removed (guide: DSH is the only supported runtime).
- Repository: `main` branch protection enabled per guide §9.3 and verified via API — required status checks, PR-only merging, linear history, no force-pushes or deletions, admin enforcement, conversation resolution; settings and date recorded in `docs/dev.md`.
- The GUI is now a default feature. Every build of the package, including the installers produced by the release pipeline, ships a window-capable binary.
- Launch behaviour: the desktop window opens when the app is started without a terminal attached, which is what the macOS app bundle and the Linux launcher do. `--gui` forces the window and `--tui` forces the terminal interface.
- Release workflow: the artifact download action is pinned to a full commit SHA, and the workflow no longer requests pull request write permission it does not use.
- CI builds the TUI-only configuration as well as the default one, so the fallback path stays warning-free.

### Known limitations
- The terminal interface and the desktop window are both still present in this release. The terminal interface is removed in v2.0.0.
- **Release artifacts published.** v1.5.1 ships `.msi` (Windows), `.dmg` (macOS), `.deb` (x86_64 and arm64 Linux), `SHA256SUMS.txt`, `SHA256SUMS.txt.asc` (GPG detached signature), and a CycloneDX SBOM on the GitHub Release.
- **Release signing key rotated.** v1.5.1's `SHA256SUMS.txt.asc` was signed with GPG key `9D07 07AF 32A5 F249 F7F8 2EA2 9829 4016 C6C9 D993` (key provisioned before this session; public key not recovered locally). The key was rotated at owner direction on 2026-11-22 to `E3C7 0D79 A8A8 A209 60B4 5FD4 2A2E 09D1 B7BC D1A1` (identity `Dust Trail Dev <dev@example.com>`, RSA-3072 sign-only, expires 2028-11-21). Private key + passphrase live only as GitHub Actions secrets (`GPG_PRIVATE_KEY`/`GPG_PASSPHRASE`, pinned by `GPG_KEY_FINGERPRINT`); see `docs/update-signing.md`.

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
- Auto-update availability check against the GitHub Releases API. Installer download, signature verification, installation, and restart were not implemented in this release.
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