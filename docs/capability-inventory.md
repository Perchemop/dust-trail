# Capability Inventory

Stable capability IDs (never renumbered). Status: not started / partial /
implemented / out of scope. Implementation evidence lives in docs/parity.md.

| ID | Capability | Observable behaviour | Acceptance criteria | Status |
|----|------------|---------------------|---------------------|--------|
| CAP-01 | ASCII world rendering | Grid of tile glyphs (ground, wall, water, tree, building, door, loot, horse) with player avatar | Frame renders correct glyphs/colours per tile | implemented |
| CAP-02 | Turn-based movement | Player moves one tile per input; NPCs act after each turn | `player_can_move_on_ground` test | implemented |
| CAP-03 | Blocked movement | Cannot enter walls or NPC-occupied tiles | `player_cannot_move_into_wall` test | implemented |
| CAP-04 | Item collection | Walking onto loot/horse collects it, updates score/mount | `collecting_loot_raises_score` test | implemented |
| CAP-05 | NPC interaction | Adjacent E shows a dialogue line; outlaw surrenders if mounted | `interacting_with_sheriff_yields_line` test | implemented |
| CAP-06 | Save / load | Game state persists to JSON and reloads identically | `save_and_load_roundtrip` test | implemented |
| CAP-07 | TUI frontend | Playable in terminal | CI release build + manual run | implemented |
| CAP-08 | GUI frontend | Playable in desktop window | CI release build --features gui | implemented |
| CAP-09 | Crash-report panic hook | Scrubbed report parked on panic | crash-report unit tests | implemented |
| CAP-10 | Update-check stub | Quiet fail-soft check against GitHub Releases API | update-check unit tests | implemented |
| CAP-11 | Contrast AA | 4.5:1 body, 3:1 UI/boundary, all variants | theme.rs contrast tests | implemented |
| CAP-12 | Multiple save slots | Named save slots | — | out of scope (v0.2.0) |
| CAP-13 | Smarter outlaw AI | Seed-based patrol | — | out of scope (v0.2.0) |
| CAP-14 | Procedural map generation | Randomised towns | — | out of scope (v1.0.0) |
| CAP-15 | In-app release-notes popup | Version + notes popup | — | out of scope (v1.0.0) |
| CAP-16 | Auto-update download + signature | Verified self-update | — | out of scope (v1.0.0) |
