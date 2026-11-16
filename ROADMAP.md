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
| v1.0.0 — Stable release | 2027-01 | 0% | Planned |

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

## v1.0.0 — Planned
* Sound effects (optional, feature-gated)
* Accessibility: contrast unit tests, keyboard focus, AccessKit labels
* GUI screenshot/evidence via kittest (seam refactor first)
* Stable save format with version migration
* Procedural map generation
* In-app release-notes popup + version display (suite standard)
* Auto-update download + signature verification

## Out of scope
* Multiplayer / networking
* Commercial-licence assets or trademarked names
* Pixel-copying of any commercial product
