# Requirements Traceability

Each requirement ID is traced to an implementation and a CI-verified test.

| ID | Requirement | Source | Status | Implementation | Evidence |
|----|-------------|--------|--------|----------------|----------|
| RT-02 | App runs as a GUI desktop window | User request | Done | `src/gui/mod.rs` (feature `gui`) | CI `cargo test --features gui`, `cargo build --release --features gui` |
| RT-03 | ASCII art rendering of a Western world | User request | Done | `src/domain/frame.rs`, `src/domain/tile.rs` | domain unit tests |
| RT-04 | Turn-based player movement | User request | Done | `src/domain/game.rs::move_player` | `tests: player_can_move_on_ground` |
| RT-05 | Blocked movement into walls / NPCs | guide II.24.1 (edge cases) | Done | `src/domain/game.rs` | `tests: player_cannot_move_into_wall` |
| RT-06 | Collect loot and horse items | User request | Done | `src/domain/game.rs::collect_item` | `tests: collecting_loot_raises_score` |
| RT-07 | Interact with NPCs (sheriff/outlaw) | User request | Done | `src/domain/game.rs::interact` | `tests: interacting_with_sheriff_yields_line` |
| RT-08 | Save and load game state | User request | Done | `src/domain/save.rs`, `src/domain/game.rs` | `tests: save_and_load_roundtrip` |
| RT-09 | Crash-report panic hook parks a scrubbed report | guide §7.1 | Done | `crates/crash-report/src/lib.rs`, `main.rs` | unit tests in crash-report |
| RT-10 | Quiet, fail-soft update-check stub | guide §7.2 | Done | `crates/update-check/src/lib.rs`, `main.rs` | `tests: status_defaults_to_idle` |
| RT-11 | Original product name, no trademarks | guide Part I §2.2 | Done | `DECISIONS.md` (Dust Trail) | N/A (decision record) |
| RT-12 | CI green: fmt, clippy, tests, builds | guide §4 | Done | `.github/workflows/ci.yml` | CI run `44965810348` (commit `fec2013`) — all five required checks green |
| RT-13 | Dual licence MIT OR Apache-2.0 | guide Part I §2.2 | Done | `LICENSE`, `ATTRIBUTION.md` | N/A |
| RT-14 | Headless domain tests (no display) | guide AGENTS.md | Done | `src/domain/*` tests | CI `cargo test --all --locked` |
