# Changelog

All notable changes to Dust Trail will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/), and
this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Added
### Changed
### Fixed
### Known limitations

## [0.3.0] — 2026-11-16

### Added
- Multiple save slots with 3D state persistence (domain layer)
- Save/Load dialog modals for managing multiple save slots in GUI
- Design Guidelines documentation in AGENTS.md (XI.1 anti-patterns, XI.2 high-quality UI patterns, XI.3 sync rules)
- Design Guidelines synchronization tracking in current-model-map.md
- Semantic danger color from theme module for error states

### Changed
- GUI module restructured for rustfmt compatibility (nested closures extracted to helper methods)
- eframe::App implementation uses `ui` method with direct ui parameter
- Token discipline: raw Color32::from_rgb removed from gui, using semantic theme roles
- AGENTS.md: removed duplicate II.31 section, added XI.1–XI.3 Design Guidelines

### Fixed
- GUI module parse error: `render_load_dialog` moved inside `impl Gui`
- Borrow checker issues with window open state (local `is_open` variables)
- `Game::list_saves()` called as associated function, not method
- `slot_clone` scope in `render_save_dialog_buttons`
- ui.columns closure syntax (pass closure calling helper with uis argument)
- Save/load dialog modals now work correctly with multiple save slots

### CI
- CI passes on all platforms (linux-arm64, macos-arm64, windows-x86_64)
- Supply-chain check validates dist-workspace.toml
- Secret/personal data scan excludes AGENTS.md appropriately

## [0.2.0] — 2026-11-14

### Added
- 3D isometric ASCII rendering with per-cell lighting/shading (glyph-density bands)
- 3D world with height (z) — raised plateau, river, buildings, trees
- GUI-only desktop window (egui/eframe 0.37); terminal TUI removed
- WASD movement, height-clamped stepping (≤1 level), NPC patrol, loot/horse
- Save / load (single slot, 3D state)

### Changed
- Terminal TUI scrapped; GUI is now the primary frontend

## [0.1.0] — 2026-11-14

### Added
- First functional test build (v0.1.0): playable ASCII Western adventure with
  TUI and GUI frontends, save/load, crash-report stub, update-check stub, and
  green CI on Linux/macOS/Windows.

