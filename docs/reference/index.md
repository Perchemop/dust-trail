# Reference Index

Structured reference generated from `docs/` for team pre-loading (guide
§5.7.2) — lets teammates query system architecture and team patterns without
re-reading raw source files (§5.6.1 lazy context loading).

| Area | File | Contents |
|------|------|----------|
| Product | `docs/project-brief.md`, `docs/scope.md` | What the app is; in/out of scope |
| Architecture | `docs/architecture.md`, `docs/specs/` | System shape; feature specifications |
| Domain | `apps/dust-trail/src/domain/` | Game state, world, entities, save format |
| Rendering | `apps/dust-trail/src/render/`, `docs/specs/gui-only-ascii-3d.md` | 3D ASCII isometric pipeline (T-001..T-020) |
| Process | `AGENTS.md`, `docs/dev.md`, `docs/gaps.md` | Ruleset, operations, ranked gaps |
| History | `CHANGELOG.md`, `PROGRESS.md`, `REQUIREMENTS_TRACEABILITY.md` | Release and requirement records |
| Decisions | `DECISIONS.md` | Every recorded decision incl. the GPG signing deviation |
| Credits | `contributors/people.toml` | Consent-verified human credits (§24.2) |

Regenerate this table when a top-level doc is added or renamed.
