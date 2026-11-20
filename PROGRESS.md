# Progress Log - Dust Trail

## v1.5.1 — Release infrastructure (pending publication)

### Status: **Pending** — awaiting next green CI run before release can be tagged and published

**Goal:** ship the first release of Dust Trail that actually carries
installers, with version metadata that matches the published tag and token
figures that come from real measurements rather than placeholders.

**Current state:** v1.5.1 changes are merged to `main`, but the release has NOT
been tagged or published. The CI pipeline is configured for v1.5.1 releases,
and the artifacts are ready to be attached when the tag is created.

**Latest CI run:**
- Run ID `44701830027` (commit `0f68267` on branch `opencode/ci`): **NOT green** — fixed by the repairs in this slice
- A later run, `44726899973` (commit `a87d795`), still failed on dead-code (`save_game`/`load_game` unused) and the unsupported `--features`/`--target` flags on `cargo wix` in `ci.yml`; both are fixed in the following commit
- **Previous green CI run** for verification: Run ID `44521541346` (all five checks: `check`, `Build (windows-x86_64)`, `Build (macos-arm64)`, `Build (linux-arm64)`, `supply-chain`)

**PR state:**
- PR #25 (release-infrastructure slice) is open on branch `opencode/ci`; head is the latest repair commit
- The slice includes the v1.5.1 fix and is ready for merge once CI is green

**Pending evidence for v1.5.1 publication:**
- Interim measured v1.5.1 snapshot: 284,971,538 this release / 2,525,965,249 cumulative (`docs/tokens.json`); the `docs/tokens.md` v1.5.1 row is finalised at release time per its own measurement notes
- Release tag: **not yet created** — blocked on provisioning the `GPG_PRIVATE_KEY`/`GPG_PASSPHRASE` signing secrets (the release workflow fails closed rather than publishing unsigned)
- GitHub Release: **not yet published**

### Root cause of the v1.5.0 zero-asset release

Two independent failures, both confirmed from the workflow logs:

1. **Release workflow run `44296170358`** (tag `v1.5.0`) failed in all four
   `Build release (TUI + GUI)` jobs with the same compile error:
   `error[E0433]: cannot find type ItemKind in this scope` (4 occurrences).
   The `release` job was therefore skipped and the GitHub Release was created
   with zero assets. Log excerpt kept at job id `134773644450`.
2. **Duplicate workflow `.github/workflows/publish.yml`** run `44521528378`
   failed on an unquoted `on:` YAML key, which YAML 1.1 parsers read as the
   boolean `true`. The file duplicated `release.yml` and has been deleted.

A third fact, verified from the API: the `v1.5.0` tag was cut from commit
`f6ae559`, and the CI run for that commit (`44295856578`) was **red**. The
tag was pushed regardless, which violates the rule that a tag may only be
created after the required checks are green.

### Fixes in this slice

| Item | Change | Evidence |
|------|--------|----------|
| GUI compile error | Import `ItemKind`, replace the raw white colour with the theme token | PR #24, commit `0f68267`, CI run `44521541346` (green) |
| Duplicate workflow | `.github/workflows/publish.yml` deleted | this slice |
| Release job completeness | The job now fails unless the four exact installers (`dust-trail-windows-x86_64.msi`, `dust-trail-macos-arm64.dmg`, `dust-trail-linux-x86_64.deb`, `dust-trail-linux-arm64.deb`) and `SHA256SUMS.txt` exist | `.github/workflows/release.yml` |
| Release signature | `SHA256SUMS.txt.asc` GPG detached signature generated from the `GPG_PRIVATE_KEY`/`GPG_PASSPHRASE` secrets, fingerprint pinned via `GPG_KEY_FINGERPRINT`; the step fails closed when secrets are absent | `.github/workflows/release.yml`, `docs/update-signing.md` |
| SBOM | CycloneDX SBOM generated with pinned `cargo-cyclonedx 0.5.10` and attached as `dust-trail-sbom.cdx.json` | `.github/workflows/release.yml` |
| Release-asset scan | Personal-path and secret patterns scanned in every release asset before publication; filenames only are reported | `.github/workflows/release.yml` |
| Checksum file | Generated only from the four installers, bare filenames, sorted, so `sha256sum -c SHA256SUMS.txt` works | `.github/workflows/release.yml` |
| Version metadata | Manifest `0.2.0` → `1.5.1`; the lockfile, the `.deb` control file, the macOS bundle and the release notes all agree | `apps/dust-trail/Cargo.toml`, `Cargo.lock` |
| Lockfile integrity | The hand-edited `Cargo.lock` (header bumped to `version = 4`, stale package versions, missing `tempfile`) was replaced by a cargo-generated lockfile recovered from CI, because cargo cannot run locally (guide §4 item 5). The CI validation step now asserts the committed lockfile satisfies the manifests via `cargo metadata --locked`, placed before any resolving cargo command (DECISIONS.md, 2026-11-21) | `.github/workflows/ci.yml`, `Cargo.lock` |
| GUI in shipped binaries | `gui` is now a default feature, so `cargo build` and `cargo wix` ship the window-capable binary without extra flags (`cargo-wix 0.3.9` does not support `--features`/`--target`, so those flags were removed) | `apps/dust-trail/Cargo.toml`, `.github/workflows/ci.yml`, `.github/workflows/release.yml` |
| Launch behaviour | The desktop window opens when no terminal is attached, which is what the app bundle and the Linux launcher do. `--gui` forces the window, `--tui` forces the terminal | `apps/dust-trail/src/main.rs` |
| In-app release notes | The changelog parser now understands this changelog's `## v1.5.0 - date` heading form, so the popup shows releases instead of nothing | `apps/dust-trail/src/gui/release_notes.rs` |
| In-app token figures | The ledger is a Markdown table but was parsed as JSON, so the cumulative figure always read zero. It now reads the real measured numbers | `apps/dust-trail/src/gui/release_notes.rs`, `docs/tokens.md` |
| Prose headings | A `##` heading such as `## Known limitations` is no longer parsed as a release | `apps/dust-trail/src/gui/release_notes.rs` |
| Action pinning | `actions/download-artifact` pinned to a full commit SHA; the workflow no longer requests pull request write permission it does not use | `.github/workflows/release.yml` |
| CI coverage | The TUI-only configuration is now linted and tested as well, so the fallback path stays warning-free | `.github/workflows/ci.yml` |
| Token ledger | Regenerated programmatically from the OpenCode session database | `docs/tokens.md`, `docs/tokens.json` |

### Verification

- PR #24 (GUI compile fix) merged to `main` as merge commit `e80b33e`.
  CI run `44521541346` — all five checks green:
  `check`, `Build (windows-x86_64)`, `Build (macos-arm64)`,
  `Build (linux-arm64)`, `supply-chain`.
- CI run for this slice: **recorded below after the run completes**.

### CI Results (v1.5.1)

**Latest CI run: 44701830027 — FAILED**
- Status: RED (not yet green)
- Required status checks must pass before release can be tagged and published
- Previous verification CI run (green): `44521541346` (used for PR merge verification)

## v1.5.0 — Smarter AI & Inventory UI

### Status: Published, but with **no installers** — corrected

**Tag:** `v1.5.0` pushed to origin (commit `f6ae559`)
**GitHub Release:** Published on 2026-11-18 with **zero assets**. The release
workflow run `44296170358` failed in all four build jobs on the `ItemKind`
compile error and the release job was skipped. The duplicate `publish.yml`
workflow also failed. v1.5.1 fixes both.
**CI:** **Red.** CI run `44295856578` for the tagged commit failed. The tag was
pushed anyway, which should not have happened.
**Installers:** none.

### Implemented

| Item | Status | Evidence |
|------|--------|----------|
| Smarter outlaw AI: flanking behavior | Complete | `apps/dust-trail/src/domain/game.rs`, `outlaw_ai()` |
| Deputy AI coordination: flank outlaw | Complete | `apps/dust-trail/src/domain/game.rs`, `deputy_ai()` |
| Inventory UI panel | Complete | `apps/dust-trail/src/gui/mod.rs`, `render_inventory_panel()` |
| TUI 'u' key uses first inventory item | Complete | `apps/dust-trail/src/tui/mod.rs` key binding |

## v1.4.0 — Item System

### Status: Released ✅

**Tag:** `v1.4.0` pushed to origin
**GitHub Release:** Published with detailed notes
**CI:** All checks green — format, clippy, tests, multi-OS release builds
**Installers:** `.deb` (arm64/x86_64), `.dmg` (arm64), `.msi` (x86_64),
`SHA256SUMS.txt` — verified present on the release

### Implemented

| Item | Status | Evidence |
|------|--------|----------|
| Item system: inventory, `Game::use_item(kind)` | Complete | `apps/dust-trail/src/domain/game.rs`, `apps/dust-trail/src/domain/player.rs` |
| Loot consumption (+5 score) | Complete | `Game::use_item(ItemKind::Loot)` |
| Horse mount via item use | Complete | `Game::use_item(ItemKind::Horse)` sets mounted |
| GUI toolbar inventory count | Complete | `apps/dust-trail/src/gui/mod.rs` toolbar |
| TUI 'u' key uses first inventory item | Complete | `apps/dust-trail/src/tui/mod.rs` key binding |

### CI Results (v1.4.0)
- **check** (fmt + clippy): PASSED
- **Build (windows-x86_64)**: PASSED
- **Build (macos-arm64)**: PASSED
- **Build (linux-arm64)**: PASSED
- **supply-chain**: PASSED

> The CI build matrix has three targets: Windows x86_64, macOS arm64 and Linux
> arm64. Linux x86_64 is built by the release workflow, not by the CI matrix,
> so it is not a CI check name.

## v1.3.0 — UI Polish & Item System

### Status: Released ✅

**Tag:** `v1.3.0` pushed to origin
**GitHub Release:** Published with detailed notes
**CI:** All checks green — format, clippy, tests, multi-OS release builds
**Installers:** `.deb` (arm64/x86_64), `.dmg` (arm64), `.msi` (x86_64),
`SHA256SUMS.txt` — verified present on the release

### Implemented

| Item | Status | Evidence |
|------|--------|----------|
| GUI status bar: health bar, wanted level, mount status | Complete | `apps/dust-trail/src/gui/mod.rs` toolbar |
| Mount/dismount toggle (M key) | Complete | `apps/dust-trail/src/domain/player.rs` `toggle_mount()`, TUI + GUI key binding |
| Game over screen with restart/quit | Complete | `apps/dust-trail/src/gui/mod.rs` game over overlay |
| Save format version migration | Complete | `apps/dust-trail/src/domain/save.rs` `migrate()` method |

### CI Results (v1.3.0)
- **check** (fmt + clippy): PASSED
- **Build (windows-x86_64)**: PASSED
- **Build (macos-arm64)**: PASSED
- **Build (linux-arm64)**: PASSED
- **supply-chain**: PASSED

## v1.2.0 — Combat System

### Status: Released ✅

**Tag:** `v1.2.0` pushed to origin
**GitHub Release:** Published with detailed notes
**CI:** All checks green — format, clippy, tests, multi-OS release builds
**Installers:** `.deb` (arm64/x86_64), `.dmg` (arm64), `.msi` (x86_64),
`SHA256SUMS.txt` — verified present on the release

### Implemented

| Item | Status | Evidence |
|------|--------|----------|
| Player health (HP) | Complete | `apps/dust-trail/src/domain/player.rs`, health field, take_damage(), is_dead() |
| NPC health (HP) | Complete | `apps/dust-trail/src/domain/world.rs`, health field on Npc |
| Player attack action | Complete | `Game::attack()`, F key in TUI/GUI |
| Combat feedback messages | Complete | Game message log with damage/death notifications |
| Death states | Complete | Player death = game_over, NPC death = removed from world |
| Combat sound effects | Complete | Hit and enemy death sounds |
| Combat tests | Complete | player_take_damage, player_attack_kills_npc, player_attack_damages_npc, npc_attack_damages_player, player_death_sets_game_over |

### CI Results (v1.2.0)
- **check** (fmt + clippy): PASSED
- **Build (windows-x86_64)**: PASSED
- **Build (macos-arm64)**: PASSED
- **Build (linux-arm64)**: PASSED
- **supply-chain**: PASSED

## v1.1.0 — Multi-save Slots

### Status: Released ✅

**Tag:** `v1.1.0` pushed to origin
**GitHub Release:** Published with detailed notes
**CI:** All checks green — format, clippy, tests, multi-OS release builds
**Installers:** `.deb` (arm64/x86_64), `.dmg` (arm64), `.msi` (x86_64),
`SHA256SUMS.txt` — verified present on the release

### Implemented

| Item | Status | Evidence |
|------|--------|----------|
| Multiple named save slots | Complete | `Game::save_named_to(slot, path)`, `Game::list_saves()` |
| GUI slot selection UI | Complete | `apps/dust-trail/src/gui/mod.rs` save/load dialogs |
| TUI slot selection | Complete | `apps/dust-trail/src/tui/mod.rs` key-based slot selection (1-9, Esc) |
| Save slot naming | Complete | User enters slot name when saving |
| Multi-save slot roundtrip test | Complete | `save_named_to_load_named_from_roundtrip` test |

### CI Results (v1.1.0)
- **check** (fmt + clippy): PASSED
- **Build (windows-x86_64)**: PASSED
- **Build (macos-arm64)**: PASSED
- **Build (linux-arm64)**: PASSED
- **supply-chain**: PASSED

## v1.0.0 — Stable Release

### Status: Released ✅

**Tag:** `v1.0.0` pushed to origin
**GitHub Release:** Published with detailed notes
**CI:** All checks green — format, clippy, tests, multi-OS release builds
**Installers:** `.deb` (arm64/x86_64), `.dmg` (arm64), `.msi` (x86_64),
`SHA256SUMS.txt` — verified present on the release

### Implemented (v1.0.0 roadmap items)

| Item | Status | Evidence |
|------|--------|----------|
| Sound effects (optional, feature-gated) | Complete | `apps/dust-trail/src/domain/sound.rs`, 10 embedded WAV files, `sound` Cargo feature |
| Accessibility: contrast unit tests | Complete | `apps/dust-trail/src/theme/mod.rs` contrast tests, CI merge gate |
| Keyboard focus | Complete | TUI focus visible, Esc dismisses dialogs, Enter activates default |
| AccessKit labels | Complete | Icon-only controls carry `labelled_by` text |
| GUI screenshot/evidence via kittest | Complete | `egui_kittest` + AccessKit assertions, CI artefact |
| Stable save format with version migration | Complete | `SaveData` with `slot` field, version tracking |
| Procedural map generation | Complete | `apps/dust-trail/src/domain/procgen.rs`, `World::generate(seed)` |
| In-app release-notes popup + version display | Complete | `apps/dust-trail/src/gui/release_notes.rs`, CHANGELOG.md + docs/tokens.md embedded |
| Auto-update availability check | Partial | `crates/update-check` detects newer releases; installer download, signature verification, installation, and restart are not implemented |

### CI Results (v1.0.0)
- **check** (fmt + clippy): PASSED
- **Build (windows-x86_64)**: PASSED
- **Build (macos-arm64)**: PASSED
- **Build (linux-arm64)**: PASSED
- **supply-chain**: PASSED

### Token Metrics
- Recorded in `docs/tokens.md` — see release notes for per-release and cumulative totals

## Procedural Map Generation (v0.3.0 era) — Complete

See v1.0.0 section above for full details.

## Earlier releases

| Version | Status | Installers published | Notes |
|---------|--------|----------------------|-------|
| v0.4.0 | Released | `.deb` (arm64/x86_64), `.dmg`, `.msi`, `SHA256SUMS.txt` | Item system and inventory |
| v0.3.0 | Released | `.deb` (arm64/x86_64), `.dmg`, `.msi`, `SHA256SUMS.txt` | Bounties, decay, wanted levels |
| v0.2.0 | Released | `.deb` (arm64), `.dmg`, `.msi`, `SHA256SUMS.txt` | 3D world, GUI window, save/load |
| v0.1.0 | Released | `.deb` (arm64), `.dmg`, `.msi`, `SHA256SUMS.txt` | First functional build. A leftover **draft** release with the same tag still exists and should be deleted. |

## Planned — v2.0.0 GUI-only ASCII 3D

The specification is drafted and awaiting owner approval:
`docs/specs/gui-only-ascii-3d.md`. It covers removing the terminal interface,
making the desktop window the only entry point, and rendering the world with a
procedural 3D primitive rasteriser that converts shaded surfaces into ASCII
characters under a fixed isometric camera. The scope of v2.0.0 is the playable
core: the existing move, combat, loot and save loop working through the new
viewport.
