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
| CAP-07 | GUI frontend | met | CI `cargo build --release` (gui is default) |
| CAP-08 | Crash-report panic hook | met | `crates/crash-report` unit tests |
| CAP-10 | Update-check stub | met | `crates/update-check` unit tests |
| CAP-11 | Contrast (4.5:1/3:1, all variants) | met | `theme.rs` contrast tests |
| CAP-12 | Multiple save slots | met | `tests: slot_names_are_validated_and_trimmed`; `tests: save_named_to_load_named_from_roundtrip`; `tests: store_roundtrips_two_slots_through_disk`; CI run 44521541346 |
| CAP-13 | Smarter outlaw AI | met | `tests: outlaw_flanks_player_when_near`; `tests: deputy_chases_outlaw_when_near`; CI run 44521541346 |
| CAP-14 | Procedural map generation | met | `tests: generate_is_deterministic`, `test: generate_different_seeds_differ`, plus 5 connectivity property tests in `domain/procgen.rs`; CI run 44521541346 |
| CAP-15 | In-app release-notes popup | met | `gui/release_notes.rs` parser tests: `parse_releases_plain_v_heading`, `parse_releases_bracket_heading_normalises`, `parse_token_ledger_basic`, `parse_releases_ignores_prose_headings`; PR #25 verification pending |
| CAP-16 | Auto-update download + signature | partial | `crates/update-check` detects a newer release only. Installer download, SHA-256/signature verification, staging, installation, and restart are not implemented. |
