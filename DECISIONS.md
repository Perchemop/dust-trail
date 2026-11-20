# Decisions - Dust Trail

## Version 1.5.1 — release-infrastructure slice (2026-11-20)
- **Decision:** Ship the release-pipeline fix as `v1.5.1`, and reserve `v2.0.0` for the GUI-only ASCII-3D transformation.
- **Rationale:** Removing the terminal interface is a breaking change and therefore a major version. Bundling it with the pipeline repair would have made an already-broken release harder to diagnose, and would have delayed the first release that actually carries installers.
- **Implementation:** Manifest version moved from `0.2.0` to `1.5.1` (the lockfile, the `.deb` control file, the macOS bundle and the release notes all derive from the same value now). The release pipeline was repaired in the same slice so the next tag ships real artefacts.

### v1.5.1 scope decision
- **Assets:** `.msi` (Windows), `.dmg` (macOS), `.deb` for x86_64, `.deb` for arm64, `SHA256SUMS.txt`, CycloneDX SBOM
- **No AppImage:** AppImage is not produced for Dust Trail. The project ships native platform packages only.
- **Signature method:** GPG detached signatures (minisign-compatible format). The release workflow attaches a `.asc` signature over `SHA256SUMS.txt`.
- **Key status:** **No key is currently provisioned.** The release signing key must be stored in the GitHub secret `RELEASE_SIGNING_KEY`. Signature verification is not functional until this secret is configured.
- **Platform installation:**
  - **Windows:** Download the `.msi` from the GitHub Release and run it with standard Windows installation privileges.
  - **macOS:** Download the `.dmg`, open it, and drag Dust Trail to the Applications folder.
  - **Linux (Debian):** Download the `.deb` and install with `sudo apt install ./dust-trail_*.deb`.

### v2.0.0 exclusion
- GUI-only ASCII-3D rendering and TUI removal are explicitly reserved for v2.0.0. These are not part of the v1.5.1 release-infrastructure slice.

## Frontend selection (2026-11-20)
- **Decision:** The app opens the desktop window when it is started without a terminal attached, and the terminal interface when a terminal is attached. `--gui` forces the window, `--tui` forces the terminal.
- **Rationale:** The macOS app bundle and the Linux launcher both start the app with no terminal attached. Under the previous behaviour they opened a terminal interface inside a window with no terminal, which looked broken. The terminal interface remains reachable for anyone who wants it.
- **Implementation:** `apps/dust-trail/src/main.rs`, using `std::io::IsTerminal` (stable since Rust 1.70; the crate floor is 1.82).

## GUI is a default feature (2026-11-20)
- **Decision:** `default = ["tui", "gui"]` in `apps/dust-trail/Cargo.toml`, and both `cargo build` and `cargo wix` pass `--features gui` explicitly.
- **Rationale:** The release pipeline built the binary with `--features gui` but packaged it with `cargo wix` against the default feature set, so every shipped installer was terminal-only even though the pipeline appeared to build the GUI. Making the GUI a default feature removes the chance of the two diverging again, and the explicit flag keeps the intent visible.
- **Constraint:** The terminal interface is still removed in `v2.0.0`, not before.

## v2.0.0 renderer and camera (2026-11-20)
- **Decision:** The v2.0.0 world renderer is a **procedural 3D primitive rasteriser**. Each object (tree, shrub, character, prop) is a set of 3D primitives rendered into a depth buffer and a luminance buffer, then converted to characters through a glyph ramp. The camera is a **fixed isometric** camera, matching the existing dimetric projection.
- **Rationale:** The owner chose this over hand-authored sprite sheets and over a true perspective camera. The rasteriser produces objects that read as shaded 3D models and respond to light, stays deterministic and headless-testable, and reuses the projection the game already has.
- **Constraint:** The v2.0.0 milestone is the **playable core**: terminal interface removed, desktop window the only entry point, the new renderer live for terrain, props and characters, and the existing move/combat/loot/save loop working through the new viewport. Full content re-authoring, day/night lighting, signed installers, an SBOM, the accessibility rendered-evidence pass and README screenshots are explicitly out of scope for v2.0.0.
- **Specification:** `docs/specs/gui-only-ascii-3d.md` (draft, awaiting owner approval).

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