# AGENTS.md

# Dust Trail

Original Western-themed ASCII adventure game, playable as TUI and GUI Rust
application. Adhere to `~/guide.md` (kept outside the repo, never committed) —
the single source of truth.

## Purpose
Build a free, open-source ASCII adventure game inspired by the Western genre
(original mechanics, art, and name — no Rockstar trademarks or trade dress).
Playable as a terminal TUI and an egui/eframe GUI.

## Operating model (guide §5)

Chat (plan) → Task (execute) → Pull Request (review) → Merge

- **Chat (planning)** — orchestrator clarifies the goal, writes a Task brief
  (requirement IDs, files likely touched, acceptance criteria, definition of
  done) and updates `IMPLEMENTATION_PLAN.md`. Never edits application code and
  never runs heavy builds.
- **Task (execution)** — coder makes one small, bounded vertical slice, updates
  tests and tracking files, then commits and pushes. GitHub Actions verifies it.
- **Pull Request (review)** — the change lands as a PR; CI plus the reviewer
  and human review it (guide §9).
- **Merge** — the agent merges once CI is green and review findings addressed
  (owner directive 2026-10-26).
- **PR template** — every PR uses `.github/PULL_REQUEST_TEMPLATE.md`.

## Release Currency (owner directive)

After all remaining work within reason is complete (or awaiting only user
verification), confirm the latest GitHub Release is current with `main`. A
release is **not** current if `main` contains commits not reachable from the
release tag.

- Before declaring a session complete, compare `main` against the latest tag:
  `git merge-base --is-ancestor <latest-tag> origin/main && echo green` or
  `git log --oneline <latest-tag>..origin/main | head`.
- If `main` has unreleased commits, either cut a new release tag and publish
  installable artefacts (per the installable-builds directive below) **or**
  record the pending work explicitly in `PROGRESS.md` / `ROADMAP.md` so the
  next session knows what still needs releasing.
- Never leave `main` ahead of the latest release without either publishing a
  release or documenting the gap.

## "Just Works" Verification Workflow (guide §4)

**This app must always "just work" out of the box.**

### Installable Builds for Every Release

Every release must provide installable build artefacts for every platform the app
supports. For native Rust applications this means producing platform-specific
installers (`.msi`/`.exe` on Windows, `.dmg`/`.pkg` on macOS, `.deb`/`.rpm`/
`.AppImage` on Linux) that users can download and install without needing a Rust
toolchain installed. A release that consists only of source code or only
provides source-built binaries is incomplete — the build artefact must be
consumable directly by end users.

### Step 1: Pre-flight check (local, no cargo needed)

Before running `ci.sh`, fix the most common CI failures locally. These are
patterns the CI's `cargo fmt --check` and `cargo clippy -- -D warnings` will
flag:

| Check | Command | What it catches |
|---|---|---|
| Line length | `awk 'length > 100 {print FILENAME":"FNR}' <file>` | Lines over 100 chars |
| Trailing whitespace | `grep -rn ' $' <file>/` | Whitespace at end of lines |
| Missing final newline | `tail -c 1 <file> | xxd | grep -q '0a'` | File not ending with newline |

**Rustfmt style expectations** (these fail CI if violated):
- Single-line struct literals and match arms stay on one line when they fit
- Long method chains break onto separate lines, one call per line
- Import blocks are alphabetical within each group
- `if let` with a single body stays on one line when it fits

### Step 2: Run CI

```bash
~/.config/opencode/scripts/ci.sh run "<short description of the change>"
```

That script commits the working tree to a branch, pushes it, opens or updates a
pull request so the repository's CI workflow triggers, waits for the checks,
and prints the outcome — including the failing job logs when something breaks.

Read the CI output, fix the code, run the script again. The CI result is the
only authoritative verification; never claim a build or test suite passed
without it.

### Common CI failures and quick fixes

| Failure | Likely cause | Fix |
|---|---|---|
| `rustfmt` diff | Lines >100 cols, multi-line struct literals, unaligned imports | Fix line length, collapse/expand struct literals, alphabetize imports |
| `clippy: unused_imports` | Import not used anywhere | Remove the import |
| `clippy: needless_borrow` | `&field` where `field` suffices | Remove the `&` |
| `clippy: pedantic` | Style violations (needless return, etc.) | Rewrite to match clippy suggestion |
| `test failures` | Hard-coded version numbers, stale test data | Use invariants, not literal version strings |

**Pro tip**: If CI fails on formatting, don't guess — copy the exact diff from the CI log and apply it line-by-line. The diff shows exactly what rustfmt wants.

## Codebase rules
- Rust only, 100% free OSS.
- Domain-first: `src/domain` is pure (no UI), fully serialisable with serde,
  testable headlessly.
- Formatting: rustfmt conventions, checked by CI. No trailing whitespace.
- Linting: `cargo clippy --all-targets -- -D warnings` (warnings as errors).
- Testing: `cargo test` run by CI, never locally. Add/extend tests when
  changing behaviour or fixing bugs.
- Error handling: `thiserror`; avoid panics in logic; prefer `Result`.
- Conventions: snake_case, flat explicit modules, doc comments for public API.
- UI: crossterm (TUI) and egui/eframe 0.37 (GUI, feature-gated).
- Persistence: JSON (serde_json), single save slot `savegame.json`.
- Git: small focused slices; imperative descriptive messages.

## Git identity (guide II.26.7)

Every git action — commit, PR, merge, tag — is authored as
`Dust Trail Dev <dev@example.com>` (repository-local config; `ci.sh` asserts
it). GitHub PR/merge authorship follows the pushing account (platform
limitation), never a reason to use a personal identity in commit metadata.

## Testing strategy
- Unit tests in `src/domain/*` modules.
- Headless tests must not require a display.
- Roundtrip save/load tests.

## Clean-room / legal
- No trademarked names, logos, icons, fonts, string tables, or exact layouts
  from any commercial product (guide Part I §2.2).
- Product name "Dust Trail" is original; pending trademark clearance recorded
  in `DECISIONS.md`.
- Dual licence MIT OR Apache-2.0; `ATTRIBUTION.md` lists every crate.
