# Capability Inventory

Stable capability IDs (never renumbered). Status: not started / partial /
implemented / out of scope. Implementation evidence lives in docs/parity.md.

| ID | Capability | Observable behaviour | Acceptance criteria | Status |
|----|------------|---------------------|---------------------|--------|
| CAP-01 | ASCII world rendering | Grid of tile glyphs (ground, wall, water, tree, building, door, loot, horse) with player avatar, laid out in an **isometric diamond projection** | Frame renders correct glyphs/colours per tile at isometric screen positions | implemented |
| CAP-02 | Turn-based movement | Player moves one tile per input; NPCs act after each turn | `player_can_move_on_ground` test | implemented |
| CAP-03 | Blocked movement | Cannot enter walls or NPC-occupied tiles | `player_cannot_move_into_wall` test | implemented |
| CAP-04 | Item collection | Walking onto loot/horse collects it, updates score/mount | `collecting_loot_raises_score` test | implemented |
| CAP-05 | NPC interaction | Adjacent E shows a dialogue line; outlaw surrenders if mounted | `interacting_with_sheriff_yields_line` test | implemented |
| CAP-06 | Save / load | Game state persists to JSON and reloads identically | `save_and_load_roundtrip` test | implemented |
| CAP-07 | TUI frontend | Playable in terminal | CI release build + manual run | implemented |
| CAP-08 | GUI frontend | Playable in desktop window | CI release build --features gui | implemented |
| CAP-09 | Crash-report panic hook | Scrubbed report parked on panic | crash-report unit tests | implemented |
| CAP-10 | Update-check stub | Quiet fail-soft check against GitHub Releases API | update-check unit tests | implemented |
| CAP-11 | Contrast AA | 4.5:1 body, 3:1 UI/boundary, all variants | `src/theme/mod.rs` contrast tests | implemented |
| CAP-12 | Multiple save slots | Named save slots | `slots.rs` tests: `slot_names_are_validated_and_trimmed`, `save_named_to_load_named_from_roundtrip`, `store_roundtrips_two_slots_through_disk` | implemented |
| CAP-13 | Smarter outlaw AI | Outlaw flanks player, coordinates with deputies | `game.rs` tests: `outlaw_flanks_player_when_near`, `deputy_chases_outlaw_when_near` | implemented |
| CAP-14 | Procedural map generation | Randomised towns | `procgen.rs` tests: `generate_is_deterministic`, `generate_different_seeds_differ`, plus 5 connectivity property tests | implemented |
| CAP-15 | In-app release-notes popup | Version + notes popup | `gui/release_notes.rs` parser tests: `parse_releases_plain_v_heading`, `parse_releases_bracket_heading_normalises`, `parse_token_ledger_basic`, `parse_releases_ignores_prose_headings` | implemented |
| CAP-16 | Auto-update download + signature | Verified self-update | **partial** — app detects updates; download, SHA-256/signature verification, staging, installation, and restart not implemented | partial |
