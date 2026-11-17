# Decisions - Dust Trail

## Procedural Map Generation
- **Decision:** Implement deterministic procedural generation from seed in `procgen.rs`
- **Rationale:** Enables reproducible worlds for testing, demos, and saves; falls within v1.0.0 roadmap
- **Implementation:** `World::generate(seed)` delegating to `procgen::generate(seed)` with BFS connectivity guarantee and retry logic
- **Performance target:** <10ms for 40×22 map, well under 100ms CI gate
- **Tests:** 7 headless property tests verifying determinism, connectivity, and border enforcement

## Clean-room compliance
- **Decision:** All generation code is original; no reverse-engineered content or proprietary algorithms
- **Rationale:** Guide §2.2 forbidden items (decompile, reverse-engineer, copy proprietary source)
- **Compliance:** No commercial product assets, icons, fonts, or algorithms copied

## Headless test philosophy
- **Decision:** All domain/tests run without GUI, no DISPLAY required
- **Rationale:** Guide K.13: "Tests must not need a display"; Guide M.2: v0.1.0 blocked unless headless workflow tests exist
- **Implementation:** All procgen.rs tests use only tile lookups and BFS; no egui/eframe dependencies

## Token metrics
- **Decision:** Track token usage via `docs/tokens.md` schema per guide §24
- **Rationale:** Guide §9.3 requires token metrics in release notes and in-app version popup