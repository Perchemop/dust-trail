# Decisions - Dust Trail

## Version 1.5.1 — release-infrastructure slice (2026-11-20)
- **Decision:** Ship the release-pipeline fix as `v1.5.1`, and reserve `v2.0.0` for the GUI-only ASCII-3D transformation.
- **Rationale:** Removing the terminal interface is a breaking change and therefore a major version. Bundling it with the pipeline repair would have made an already-broken release harder to diagnose, and would have delayed the first release that actually carries installers.
- **Implementation:** Manifest version moved from `0.2.0` to `1.5.1` (the lockfile, the `.deb` control file, the macOS bundle and the release notes all derive from the same value now). The release pipeline was repaired in the same slice so the next tag ships real artefacts.

### v1.5.1 scope decision
- **Assets:** `.msi` (Windows), `.dmg` (macOS), `.deb` for x86_64, `.deb` for arm64, `SHA256SUMS.txt`, CycloneDX SBOM
- **No AppImage:** AppImage is not produced for Dust Trail. The project ships native platform packages only.
- **Signature method:** GPG detached signatures. The release workflow attaches a `.asc` signature over `SHA256SUMS.txt`.
- **Key status:** **No key is currently provisioned.** The release signing key must be stored in the GitHub secret `GPG_PRIVATE_KEY` (ASCII-armored private key) with its passphrase in `GPG_PASSPHRASE`; pin the public fingerprint in `GPG_KEY_FINGERPRINT`. The signing step fails the release (never publishes unsigned) until these secrets are configured. See `docs/update-signing.md`.
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
- **Decision:** `default = ["tui", "gui"]` in `apps/dust-trail/Cargo.toml`. Builds and `cargo wix` run against the default feature set with no extra flags (`cargo-wix 0.3.9` does not support `--features`/`--target`).
- **Rationale:** The release pipeline built the binary with `--features gui` but packaged it with `cargo wix` against the default feature set, so every shipped installer was terminal-only even though the pipeline appeared to build the GUI. Making the GUI a default feature removes the chance of the two diverging again.
- **Constraint:** The terminal interface is still removed in `v2.0.0`, not before.

## Lockfile gate: manifest drift, not registry movement (2026-11-21)
- **Decision:** The CI `Cargo.lock` gate checks that the committed lockfile satisfies the manifests (`cargo metadata --locked`), not that it matches a fresh registry re-resolution byte-for-byte. The committed lockfile is cargo-generated (recovered from CI run `44734392220` after an earlier hand-edit was found); it is updated deliberately when manifests change, never hand-edited.
- **Rationale:** The byte-for-byte regenerate-and-diff gate also failed whenever an upstream crate published a new version between CI runs — `zcheapstr` went 1.3.0 → 1.4.0 on crates.io within hours of the recovery, and the gate would keep turning upstream releases into red CI (a treadmill, and a flaky gate per the guide's flaky-test policy). Registry movement is not manifest drift. True reproducibility is enforced by the `--locked` build and test steps, which pass with the committed lockfile.
- **Implementation:** `.github/workflows/ci.yml` "Cargo.lock validation (manifest drift)" step.

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
- **Skill: caveman (JuliusBrussee/caveman, v0.1.0, MIT)** - Terse response style (~65% output token reduction). Adopted into `~/.config/dsh/skill/caveman/`. Reduces output token volume for LLM interactions.
- **Skill: code-context-engine (elara-labs/code-context-engine, v0.1.0, MIT)** - Local MCP server for codebase indexing (~94% input-token savings). Adopted into `~/.config/dsh/skill/code-context-engine/`. Reduces re-reading of files in CI workflows.
- **Skill: headroom (headroomlabs-ai/headroom, v0.1.0, Apache-2.0)** - Context compression layer for tool outputs, logs, files, history. Adopted into `~/.config/dsh/skill/headroom/`. Compresses tool outputs to reduce token volume.
- **Skill: code-review-graph (tirth8205/code-review-graph, v0.1.0, MIT)** - Local-first code-intelligence graph for MCP/CLI; risk-scored PR reviews. Adopted into `~/.config/dsh/skill/code-review-graph/`. Provides risk-scored PR reviews for Dust Trail code changes.
- **Skill: rtk (rtk-ai/rtk, v0.1.0, Apache-2.0)** - CLI proxy; 60–90% token reduction on dev commands. Adopted into `~/.config/dsh/skill/rtk/`. Compresses CLI tool outputs to reduce token volume.
- **Skill: awesome-llm-apps (shubhamsaboo/awesome-llm-apps, v0.1.0, Apache-2.0)** - Catalog of installable agent skills for discovery. Adopted into `~/.config/dsh/skill/awesome-llm-apps/`. Provides template scaffolding and architecture reference.
- **Skill: awesome-mcp-servers (punkpeye/awesome-mcp-servers, v0.1.0, source repo licences)** - Catalog of MCP servers for research and tool integration. Adopted into `~/.config/dsh/skill/awesome-mcp-servers/`. Provides MCP server catalogue for tool integration.
| D-? | GPG release signing key | Guide §7.2 requires signed releases; DECISIONS.md recorded a GPG-detached `.asc` deviation from the guide's minisign, but no key existed | Owner directed the agent to create the key (2026-11-22). RSA-3072 sign-only, identity `Dust Trail Dev <dev@example.com>`, fingerprint `E3C70D79A8A8A20960B45FD42A2E09D1B7BCD1A1`, expires 2028-11-21. Private key + passphrase live **only** as GitHub Actions secrets (`GPG_PRIVATE_KEY`/`GPG_PASSPHRASE`/`GPG_KEY_FINGERPRINT` pins it); local copies destroyed after an isolated import→sign→verify test; public key published in `docs/update-signing.md` | 2026-11-22 |
| D-? | DSH verification shim on this machine | Guide §4 names `~/.config/dsh/scripts/ci.sh` as the verification path; the machine only had the legacy `~/.config/opencode/` shim | Created `~/.config/dsh/scripts/ci.sh` as a relative symlink to the legacy shim (single source; no behavioural change). Known gap: the shim has no `merge` subcommand — PR merges go through `gh pr merge --rebase` from the Dev account, recorded here and in `docs/dev.md` | 2026-11-22 |
| D-? | Dependabot PR authorship reconciliation | Guide II.26.7 requires Dev-account authorship for all GitHub actions; dependabot PRs cannot pass `pr-author-is-dev` | Dependabot PRs are rebased by dependabot, then re-authored (cherry-pick + Dev-identity commit) and merged from a Dev branch; the dependabot PR is closed with a reference. Applied to #28 (rodio 0.23, merged as #34) | 2026-11-22 |
| D-? | rand 0.10 deferral | Dependabot PR #29 (rand 0.9→0.10) failed to compile: `StdRng` lost `Clone`, `from_os_rng` removed, range/bool methods moved to `rand::RngExt` | Closed #29 from the Dev account; the migration is a dedicated ticketed slice in `docs/gaps.md` with red/green CI evidence per §5.6.4 — not adopted blind | 2026-11-22 |



## MCP Server Adoption (guide §25.2, §19)

| Decision | Server | Licence | Transport | Rationale | Date |
|----------|--------|---------|-----------|-----------|------|
| D-MCP-001 | GitHub MCP (@modelcontextprotocol/server-github) | MIT | stdio | Replaces gh CLI calls for issues, PRs, releases, Actions. Credential: ' + credRef + '. | 2026-11-23 |
| D-MCP-002 | Rust Analyzer MCP (rust-analyzer) | Apache-2.0 OR MIT | stdio | Semantic code navigation, type info, goto definition — essential for Rust codebase work. | 2026-11-23 |
| D-MCP-003 | Code Review Graph MCP (tirth8205/code-review-graph) | MIT | stdio | Persistent codebase knowledge graph, risk-scored PR reviews, token-efficient review per §19 rule 7. | 2026-11-23 |
| D-MCP-004 | Context7 MCP (@context7/mcp-server) | Apache-2.0 | stdio | Version-specific documentation lookup for clean-room research (§2.4, §25.2). Replaces web browsing for public docs. | 2026-11-23 |

Credential setup: GitHub PAT (classic) with scopes repo, workflow, read:org, read:user. Store in DSH Settings → Credentials as github_token. Referenced as ' + credRef + ' in .dsh/settings.local.json. Never committed to repo.

One context tool rule (§19 rule 6): code-review-graph is the adopted code-intelligence tool; code-context-engine remains available as a skill but is not stacked as an additional MCP server for the same job. headroom is adopted as a skill (token compression) but not as an MCP server to avoid duplicating rtk's CLI-proxy role.

Vetting (§25.2): Each server checked against §II.20.4 — licence confirmed, maintenance signal checked, security posture reviewed. Credentials never in config; always ' + credRef + ' templates.



## Standing Agent Operating Preferences (user directive)

| Preference | Setting | Rationale |
|------------|---------|-----------|
| Auto-enable skills | Enabled — skills auto-loaded where relevant per guide section 19; caveman terse style always active to cut output tokens | Reduces output volume; matches user directive |
| Auto-compact | Enabled — context compacted where relevant (headroom/rtk) to stay inside Smart Zone under 150k tokens per guide section 5.6.1 | Prevents dumb-zone degradation |
| Auto-enable MCP | Enabled — MCP servers auto-enabled where relevant per guide section 25.2; GitHub, Rust Analyzer, Code Review Graph, Context7 active by default | Replaces gh CLI calls and web browsing; enables code navigation and clean-room research |

These preferences are standing instructions for every DSH session on this repository. They do not waive any guide rule: no local cargo builds, no runtime-specific installs, credentials never committed, linear-history rebase merges only, and all changes verified through ci.sh before merge.
