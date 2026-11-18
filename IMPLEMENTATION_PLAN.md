# Implementation Plan
Vertical slices for Dust Trail, each a PR with CI green.

## Slice 1 — Foundation (done, v0.1.0)
* Workspace, shared crates (crash-report, update-check), app crate.
* Domain layer: world, player, game, tile, cell, frame, color, save — headless + tests.
* TUI (crossterm): terminal render, movement, interact, save/load.
* GUI (egui/eframe 0.37, feature-gated): monospace ASCII grid, toolbar, keyboard.
* main.rs: crash-report panic hook, parked-report review, update-check stub.
* CI: fmt, clippy, tests (default + gui), multi-OS builds, secret/personal-data scan, token gate.
* Tracking: README, CHANGELOG, ROADMAP, ATTRIBUTION, AGENTS.md, DECISIONS.md, REQUIREMENTS_TRACEABILITY.md, PR template, issue templates.
* Acceptance: `cargo test --all --locked` green; `cargo test --features gui --locked` green; release builds on Linux/macOS/Windows.

## Slice 2 — v0.2.0 polish (planned)
* Multiple save slots.
* Smarter outlaw AI (seed-based patrol).
* Wanted level / bounty mechanic.
* Accessibility: contrast unit tests (done in theme), keyboard focus, AccessKit labels, shortcuts doc.
* GUI kittest evidence (seam refactor first — extract UI from eframe::CreationContext).
* Sound effects (optional, feature-gated).

## Slice 3 — v1.0.0 stable (planned)
* Stable save format with version migration.
* **Procedural map generation.**
* In-app release-notes popup + bottom-right version display (suite standard).
* Auto-update download + signature verification (minisign).
* Multi-platform installers.

## Slice 4 — Post-v1.0 enhancements
* [TBD per roadmap]