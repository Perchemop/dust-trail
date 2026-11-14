# Changelog

All notable changes to Dust Trail will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/), and
this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Added
* Initial ASCII Western adventure game (Dust Trail).
* Pure-Rust domain layer: world grid, player, NPCs, items, turn-based movement.
* TUI frontend (crossterm): terminal rendering, movement, interact, save/load.
* GUI frontend (egui/eframe 0.37): desktop window with monospace ASCII grid.
* Shared crash-report and update-check suite crates.
* Panic hook that parks a scrubbed crash report for review.
* Quiet, fail-soft update check against GitHub Releases API.

### Changed
* (none yet)

### Fixed
* (none yet)

### Known limitations
* Single save slot (`savegame.json`).
* GUI keyboard input works; mouse is used for toolbar buttons.
* One town/wilderness map; procedural generation is out of scope for v0.1.0.

## [0.1.0] — 2026-11-14

### Added
* First functional test build (v0.1.0): playable ASCII Western adventure with
  TUI and GUI frontends, save/load, crash-report stub, update-check stub, and
  green CI on Linux/macOS/Windows.
