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
