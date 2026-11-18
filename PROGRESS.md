# Progress Log - Dust Trail

## v1.1.0 — Multi-save Slots

### Status: CI Green ✅

**CI:** All checks green — format, clippy, tests, multi-OS release builds  
**Ready for tag:** v1.1.0

### Implemented

| Item | Status | Evidence |
|------|--------|----------|
| Multiple named save slots | Complete | `Game::save_named_to(slot, path)`, `Game::list_saves()` |
| GUI slot selection UI | Complete | `apps/dust-trail/src/gui/mod.rs` save/load dialogs |
| TUI slot selection | Complete | `apps/dust-trail/src/tui/mod.rs` key-based slot selection (1-9, Esc) |
| Save slot naming | Complete | User enters slot name when saving |
| Multi-save slot roundtrip test | Complete | `save_named_to_load_named_from_roundtrip` test |

### CI Results (v1.1.0)
- **check** (fmt + clippy): PASSED
- **Build (linux-arm64)**: PASSED
- **Build (linux-x86_64)**: PASSED
- **Build (macos-arm64)**: PASSED
- **Build (windows-x86_64)**: PASSED
- **supply-chain**: PASSED

## v1.0.0 — Stable Release

### Status: Released ✅

**Tag:** `v1.0.0` pushed to origin  
**GitHub Release:** Published with detailed notes  
**CI:** All checks green — format, clippy, tests, multi-OS release builds  
**Installers:** .deb (arm64/x86_64), .dmg (arm64), .msi (x86_64), SHA256SUMS.txt

### Implemented (v1.0.0 roadmap items)

| Item | Status | Evidence |
|------|--------|----------|
| Sound effects (optional, feature-gated) | Complete | `apps/dust-trail/src/domain/sound.rs`, 10 embedded WAV files, `sound` Cargo feature |
| Accessibility: contrast unit tests | Complete | `apps/dust-trail/src/theme/mod.rs` contrast tests, CI merge gate |
| Keyboard focus | Complete | TUI focus visible, Esc dismisses dialogs, Enter activates default |
| AccessKit labels | Complete | Icon-only controls carry `labelled_by` text |
| GUI screenshot/evidence via kittest | Complete | `egui_kittest` + AccessKit assertions, CI artefact |
| Stable save format with version migration | Complete | `SaveData` with `slot` field, version tracking |
| Procedural map generation | Complete | `apps/dust-trail/src/domain/procgen.rs`, `World::generate(seed)` |
| In-app release-notes popup + version display | Complete | `apps/dust-trail/src/gui/release_notes.rs`, CHANGELOG.md + docs/tokens.md embedded |
| Auto-update download + signature verification | Complete | `crates/update-check`, SHA-256 + minisign, anti-loop marker |

### CI Results (v1.0.0)
- **check** (fmt + clippy): PASSED
- **Build (linux-arm64)**: PASSED
- **Build (linux-x86_64)**: PASSED
- **Build (macos-arm64)**: PASSED
- **Build (windows-x86_64)**: PASSED
- **supply-chain**: PASSED

### Token Metrics
- Recorded in `docs/tokens.md` — see release notes for per-release and cumulative totals

## Procedural Map Generation (v0.3.0 era) — Complete

See v1.0.0 section above for full details.
