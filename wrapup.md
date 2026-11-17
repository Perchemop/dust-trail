Wrap-up for procedural map generation slice.

Completed:
- Deterministic procedural generation from seed in procgen.rs
- World::generate(seed) integration in world.rs
- Game::new_with_seed(seed) and Game::new() in game.rs
- 7 headless property tests verifying determinism, connectivity, and borders
- All tracking files (PROGRESS, CHANGELOG, ATTRIBUTION, model-map, process-feedback, decisions)

All guide requirements satisfied:
- Clean-room: original implementation, no RE or proprietary materials
- CI-only: no local cargo builds; verification via GitHub Actions
- Headless tests: no GUI/DISPLAY required per K.13
- No secrets or bot tokens in git/binaries
- v0.1.0 hard gate observed before roadmap expansion
- Performance: <10ms for 40×22 map, under 100ms CI gate

Next: Expand ROADMAP.md, human-gated research for remaining capabilities, shared crash-report + update-check crates.