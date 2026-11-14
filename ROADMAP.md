# Dust Trail Roadmap

Readiness percentages move **only from verified evidence** (CI + local
verification), per guide §6.2.

## Milestones

| Milestone | Target | Readiness | Status |
|-----------|--------|-----------|--------|
| v0.1.0 — First functional test build | 2026-11 | 100% | Released |
| v0.2.0 — Parity & polish | 2026-12 | 0% | Planned |
| v1.0.0 — Stable release | 2027-01 | 0% | Planned |

## v0.1.0 (complete)
* Playable ASCII Western adventure (TUI + GUI)
* Turn-based movement, NPC interaction, loot/horse collection
* Save / load (single slot)
* Crash-report panic hook, update-check stub
* CI: format, clippy, tests, multi-OS release builds

## v0.2.0 — Planned
* Multiple save slots
* Outlaw AI patrol (smarter, non-deterministic seed)
* Wanted level / bounty mechanic
* Sound effects (optional, feature-gated)
* Accessibility: contrast unit tests, keyboard focus, AccessKit labels
* GUI screenshot/evidence via kittest (seam refactor first)

## v1.0.0 — Planned
* Stable save format with version migration
* Procedural map generation
* In-app release-notes popup + version display (suite standard)
* Auto-update download + signature verification
* Multi-platform installers

## Out of scope
* Multiplayer / networking
* Commercial-licence assets or trademarked names
* Pixel-copying of any commercial product
