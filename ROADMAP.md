# Dust Trail Roadmap

Readiness percentages move **only from verified evidence** (CI + local
verification), per guide §6.2.

## Milestones

| Milestone | Target | Readiness | Status |
|-----------|--------|-----------|--------|
| v0.1.0 — First functional test build (top-down ASCII TUI+GUI) | 2026-11 | 100% | Released |
| v0.2.0 — 3D isometric ASCII GUI | 2026-11 | 100% | Released |
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

## v1.0.0 — Planned
* Multiple save slots
* Outlaw AI patrol (smarter, non-deterministic seed)
* Wanted level / bounty mechanic
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
