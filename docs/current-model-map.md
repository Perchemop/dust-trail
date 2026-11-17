# Current Model Map - Dust Trail

## Guide Information
- **Guide version:** 2026-11-16
- **Sections read:** Part I (0–18), Part II (II.18–II.30), Appendix A–O

## Active Free Models
- Orchestrator: `opencode/deepseek-v4.1-flash-free`
- Coder: `opencode/deepseek-v4.1-flash-free`
- Domain-dev: `opencode/deepseek-v4.1-flash-free`
- Reviewer: `opencode/glm-5.3-flash-free`
- UI/UX: `opencode/nemotron-3.5-super-vl-free`

## Model Rotation Record
- Started on OpenCode free models
- Will rotate to NVIDIA free if exhausted
- Per guide §5.2 and II.26.7

## Key Decisions Recorded
- **Procedural map generation** implemented in `procgen.rs` with deterministic seed-based generation
- `World::generate(seed)` integration via domain layer
- `Game::new_with_seed(seed)` and `Game::new()` methods added
- All tests headless, no GUI required
- CI green verified before client try-steps
- No RE, no secrets, no heavy local builds
- Process friction logged to `docs/process-feedback.md`

## Notes
- No `guide.md` or `addendum.md` pushed to repo (local-only)
- Hierarchy: addendum wins on client/Rust/session; guide wins on clean-room/legal
- v0.1.0 hard gate observed before roadmap theatre