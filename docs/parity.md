# Parity

Per-guide II.27.7 parity reporting against `docs/capability-inventory.md`.
Each capability ID reports met / partial / missing / deliberately out of scope
with evidence (test name, CI run).

| ID | Capability | Status | Evidence |
|----|------------|--------|----------|
| CAP-01 | ASCII world rendering | met | domain unit tests, CI `cargo test` |
| CAP-02 | Turn-based movement | met | `tests: player_can_move_on_ground` |
| CAP-03 | Blocked movement (walls/NPCs) | met | `tests: player_cannot_move_into_wall` |
| CAP-04 | Item collection (loot/horse) | met | `tests: collecting_loot_raises_score` |
| CAP-05 | NPC interaction (sheriff/outlaw) | met | `tests: interacting_with_sheriff_yields_line` |
| CAP-06 | Save / load | met | `tests: save_and_load_roundtrip` |
| CAP-07 | TUI frontend | met | CI `cargo build --release` |
| CAP-08 | GUI frontend | met | CI `cargo build --release --features gui` |
| CAP-09 | Crash-report panic hook | met | `crates/crash-report` unit tests |
| CAP-10 | Update-check stub | met | `crates/update-check` unit tests |
| CAP-11 | Contrast (4.5:1/3:1, all variants) | met | `theme.rs` contrast tests |
| CAP-12 | Multiple save slots | out of scope v0.1.0 | planned v0.2.0 |
| CAP-13 | Smarter outlaw AI | out of scope v0.1.0 | planned v0.2.0 |
| CAP-14 | Procedural map generation | out of scope v0.1.0 | planned v1.0.0 |
| CAP-15 | In-app release-notes popup | out of scope v0.1.0 | planned v1.0.0 |
| CAP-16 | Auto-update download + signature | out of scope v0.1.0 | planned v1.0.0 |
