# Dust Trail Roadmap

Readiness percentages move **only from verified evidence** (CI + local
verification), per guide §6.2.

## Milestones

| Milestone | Target | Readiness | Status |
|-----------|--------|-----------|--------|
| v0.1.0 — First functional test build (top-down ASCII TUI+GUI) | 2026-11 | 100% | Released |
| v0.2.0 — 3D isometric ASCII GUI | 2026-11 | 100% | Released |
| v0.3.0 — Outlaw AI patrol | 2026-11 | 100% | Released |
| v0.4.0 — Wanted Level / Bounty mechanic | 2026-11 | 100% | Released |
| v1.0.0 — Stable release | 2026-11 | 100% | Released |
| v1.1.0 — Multi-save slots | 2026-11 | 100% | Released |
| v1.2.0 — Combat System | TBD | 0% | Planned |

## v0.1.0 (complete — superseded by v0.2.0)
* Playable ASCII Western adventure (top-down TUI + GUI)
* Turn-based movement, NPC interaction, loot/horse collection
* Save / load (single slot)
* Crash-report panic hook, update-check stub
* CI: format, clippy, tests, multi-OS release builds

## v0.2.0 (complete)
* 3D isometric ASCII rendering with per-cell lighting/shading (glyph-density bands)
* 3D world with height (z) — raised plateau, river, buildings, trees
* GUI-only desktop window (egui/eframe 0.37); terminal TUI scrapped
* WASD movement, height-clamped stepping (≤1 level), NPC patrol, loot/horse
* Save / load (single slot, 3D state)
* Crash-report panic hook, update-check stub
* CI: format, clippy, tests, multi-OS release builds, installer artefacts
  (.deb / .dmg / .msi / .AppImage)

## v0.3.0 (complete — 2026-11-16)
* Outlaw AI patrol with Patrol/Chase/Flee states
* Non-deterministic RNG with deterministic seed for testing
- Patrol: random walk
- Chase: move toward player when within 8 tiles
- Flee: move away from sheriff when within 6 tiles
- 20% randomness factor for unpredictability
- RNG state re-seeded from OS entropy on game load
- 4 new AI tests: determinism, flee, chase, seed variance

## v0.4.0 (complete — 2026-11-16)
* Wanted Level / Bounty system
- Bounty field on Npc struct (Outlaws only)
- Crimes increase bounty: killing sheriff (+50), stealing horse (+20), stealing loot (+10), assaulting player (+30)
- Bounty decays 1 per 10 turns, never below 0
- Wanted levels: None (0-10), Low (11-30), Medium (31-60), High (61+)
- Sheriff AI scales with wanted level (pursuit range increases)
- Deputy NPC kind (blue glyph 'D'), spawns at Medium (1) and High (2) wanted levels
- Sheriff AI pursuit range scales with wanted level (6/10/15/25 tiles)
- Deputy AI: coordinated pursuit with flanking behavior
- Bounty collection: player collects bounty on capturing outlaw (mounted)
- Bounty decay: 1 per 10 turns
- Wanted levels: None (0-10), Low (11-30), Medium (31-60), High (61+)
- Deputy spawning: Medium=1, High=2, spawn around sheriff
- Add comprehensive tests: bounty crimes, decay, wanted levels, deputy spawning, bounty collection, bounty only affects outlaws

## v1.0.0 (complete — 2026-11-18)
* Sound effects (optional, feature-gated) — rodio + 10 embedded WAV files
* Accessibility: contrast unit tests (4.5:1 body, 3:1 UI), keyboard focus, AccessKit labels
* GUI screenshot/evidence via kittest — evui_kittest + AccessKit assertions
* Stable save format with version migration — SaveData with version slot field
* Procedural map generation — deterministic world generation from seed, connectivity guarantees
* In-app release-notes popup + version display (suite standard) — CHANGELOG.md + docs/tokens.md embedded
* Auto-update download + signature verification — SHA-256 + minisign, anti-loop marker
* CI green: format, clippy, tests, multi-OS release builds (Linux arm64/x86_64, macOS arm64, Windows x86_64)
* Installers published: .deb (arm64/x86_64), .dmg (arm64), .msi (x86_64), SHA256SUMS.txt

## v1.1.0 (complete — 2026-11-18)
* Multiple named save slots — `Game::save_named_to(slot, path)` with arbitrary slot names
* `Game::list_saves()` returns all available save slots
* GUI slot selection UI — selectable slot list in save/load dialogs
* TUI slot selection — keyboard-navigable slot list (1-9 keys, Esc to cancel)
* Save slot naming — user can name each save slot when saving
* CI green: format, clippy, tests, multi-OS release builds

## v1.2.0 (planned — Combat System)
* Player health (HP) — default 100, damaged by NPC attacks
* NPC health (HP) — can be damaged by player attacks, removed from world at 0 HP
* Player attack action (E key to attack adjacent NPCs)
* Combat feedback messages in game log
* Death states: player death = game over, NPC death = removed from world
* Combat sound effects (hit, death)
* CI green: format, clippy, tests, multi-OS release builds

## Out of scope
* Multiplayer / networking
* Commercial-licence assets or trademarked names
* Pixel-copying of any commercial product
