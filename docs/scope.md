# Scope

## In scope (v0.1.0)
* Pure-Rust domain layer: world grid, player, NPCs, items, turn-based movement.
* ASCII rendering of a frontier town + wilderness in an **isometric
  (diamond) projection**, not top-down.
* GUI frontend (egui/eframe 0.37, feature-gated): desktop window, monospace ASCII grid.
* Save / load (single slot).
* Crash-report panic hook (parks a scrubbed report for review).
* Quiet, fail-soft update-check stub (GitHub Releases API).
* CI: fmt, clippy, tests, multi-OS builds, secret/personal-data scan.

## Out of scope (v0.1.0)
* Multiple save slots (planned v0.2.0).
* Smarter outlaw AI / wanted level (planned v0.2.0).
* Procedural map generation (planned v1.0.0).
* Sound / music.
* Multiplayer or networking.
* Commercial trademarks, logos, icons, fonts, or exact layouts from any
  commercial product (clean-room rule, guide Part I §2.2).
* Pixel-copying of any commercial product.

## TUI Frontend (Previously in scope, removed in v2.0.0)
* **Removed:** Terminal interface (crossterm) — no longer supported.
