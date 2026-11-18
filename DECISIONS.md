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

## Agent Skills
- **Skill: caveman (JuliusBrussee/caveman, v0.1.0, MIT)** - Terse response style (~65% output token reduction). Adopted into `~/.config/opencode/skill/caveman/`. Reduces output token volume for LLM interactions.
- **Skill: code-context-engine (elara-labs/code-context-engine, v0.1.0, MIT)** - Local MCP server for codebase indexing (~94% input-token savings). Adopted into `~/.config/opencode/skill/code-context-engine/`. Reduces re-reading of files in CI workflows.
- **Skill: headroom (headroomlabs-ai/headroom, v0.1.0, Apache-2.0)** - Context compression layer for tool outputs, logs, files, history. Adopted into `~/.config/opencode/skill/headroom/`. Compresses tool outputs to reduce token volume.
- **Skill: code-review-graph (tirth8205/code-review-graph, v0.1.0, MIT)** - Local-first code-intelligence graph for MCP/CLI; risk-scored PR reviews. Adopted into `~/.config/opencode/skill/code-review-graph/`. Provides risk-scored PR reviews for Dust Trail code changes.
- **Skill: rtk (rtk-ai/rtk, v0.1.0, Apache-2.0)** - CLI proxy; 60–90% token reduction on dev commands. Adopted into `~/.config/opencode/skill/rtk/`. Compresses CLI tool outputs to reduce token volume.
- **Skill: awesome-llm-apps (shubhamsaboo/awesome-llm-apps, v0.1.0, Apache-2.0)** - Catalog of installable agent skills for discovery. Adopted into `~/.config/opencode/skill/awesome-llm-apps/`. Provides template scaffolding and architecture reference.
- **Skill: awesome-mcp-servers (punkpeye/awesome-mcp-servers, v0.1.0, source repo licences)** - Catalog of MCP servers for research and tool integration. Adopted into `~/.config/opencode/skill/awesome-mcp-servers/`. Provides MCP server catalogue for tool integration.