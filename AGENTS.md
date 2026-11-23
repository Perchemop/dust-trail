# AGENTS.md

# Dust Trail

Original Western-themed ASCII adventure game, playable as TUI and GUI Rust
application. Adhere to `~/guide.md` (kept outside the repo, never committed) —
the single source of truth.

## Standing rule: Read guide.md every section in complete detail

Before any work that touches a capability area governed by guide.md, read **every**
section of `~/guide.md` in full: Part I (mission, clean-room rules, technology
stack, operating model, progressive releases, suite infrastructure, path to a
finished app, quality control, UI/UX, source of truth, architectural stress
test, execution sequence) **and** Part II (naming/branding, legal depth,
security audit, accessibility, marketing, maintainer sustainability, definition
of done, launch readiness, open questions, designer role, clean-room research,
design vision, implementation, perceived performance, parity review, naming
conventions, personal data/secrets, Rust GUI/UX) **and** §19–§25 (DSH agent
skills, design anti-patterns, UI patterns, pstack, config examples, token schemas,
external resource catalogs). Do not rely on memory or partial
recall from previous sessions; the guide is the single source of truth and every
section can impose obligations on the current slice. Record the sections read
and the guide's version/date in `docs/current-model-map.md`.

## 1. Mission — Fully Finished Application (guide §1)

Build a free, open-source application that delivers the capabilities in `docs/project-brief.md` and `docs/scope.md`.

**Definition of finished:**
- All in-scope features implemented and verified
- Main workflows work end-to-end on the primary local platform
- Real persistence, validation, error handling (no mock-ups or fake success states)
- UI polished to the level agreed in discovery
- Tests and CI pass
- Progressive multi-platform builds after local success
- Public roadmap and release history accurate
- In-app crash/bug reporting and update-check operational (when network allowed)
- Documentation accurate

## 2. Clean-Room Rules & Legal Boundaries (Absolute) (guide §2)

### 2.1 Allowed — Public Black-Box Research

Agents may use **public** materials to understand *what* features exist and *how users interact*:
- Official public documentation and help pages
- Publicly published manuals / getting-started guides
- Public tutorials, blogs, talks
- Public marketing screenshots and demo videos (for workflow understanding only — not for pixel-copying)
- Public API specifications and open standards
- Curated public directories (§25) for discovery only

**On GitHub (public repo):** store only **original behavioural summaries**, acceptance criteria, and **URLs** to public sources.
Do **not** commit large verbatim copyrighted manual text, full scraped HTML, or third-party screenshots as project assets.

### 2.2 Forbidden

1. Decompile, disassemble, or reverse-engineer proprietary binaries or installers.
2. Copy proprietary source, icons, fonts, exact layouts, string tables, presets, or internal algorithms.
3. Use prior reverse-engineering reports (e.g. REA output) as implementation input.
4. Reproduce trademarked names, logos, or distinctive trade dress as the product identity.
5. Paste large copyrighted excerpts into the codebase or public docs.
6. Implementation may be inspired only by public behaviour, public specs, and `docs/inspiration.md` / `docs/scope.md`.
7. All data models, commands, UI, and interaction design must be **original**.
8. Third-party assets must be permissively licensed and listed in `ATTRIBUTION.md`.
9. Prefer dual licence **MIT OR Apache-2.0**.
10. Do not claim affiliation with, endorsement by, or compatibility certification from commercial vendors unless true and authorised.

### 2.3 Research After v0.1.0 + Human Gate

Once **v0.1.0** (first functional test build) exists:

1. Expand `ROADMAP.md` into an exhaustive completion plan.
2. For each remaining capability, draft `docs/research/<capability>.md` (behaviour, acceptance criteria, edge cases, **URLs only**).
3. **Human gate (mandatory):** human reviews research notes before large implementation Tasks that depend on them. Reject notes that paste copyrighted text or describe pixel-perfect UI cloning.
4. Commit approved summaries to the public repository.
5. Convert approved research into staged semver milestones.

### 2.4 Practical Web Research (Least Human Input)

1. **Preferred:** free model/provider or MCP that can browse/fetch public pages; Orchestrator assigns it to research Tasks.
2. **CI / script assist:** Action or script fetches **text** from listed public URLs; agent summarises into original notes (no bulk copyright dump in git).
3. **Human assist (last resort):** human pastes URLs or short excerpts; agent writes behavioural notes only.

Document the method in `docs/current-model-map.md`.

### 2.5 Exhaustive Legal Checklist (Every Release)

Before each public release, confirm:

| Area | Check |
|------|--------|
| Copyright | No proprietary source, assets, or substantial copied text in tree |
| Clean-room | No RE tools/output used as implementation input |
| Trademarks | Product name/logo original; no vendor marks in UI or installers |
| Trade dress | UI is original expression; not a slavish visual copy of a commercial product |
| Patents | Avoid implementing known patented techniques when a public non-infringing alternative exists; prefer public standards |
| Licence | App licence clear (MIT OR Apache-2.0 recommended); all third-party NOTICE/ATTRIBUTION complete |
| Secrets | No API keys, tokens, or credentials in git or release binaries |
| Privacy | Crash reports send only what user consents to; documented in privacy note or README |
| GitHub API | User-supplied token only; no embedded privileged bot token in client |
| Research files | Public research = summaries + URLs; no scraped manual corpora |
| Claims | README/roadmap do not state false certification, affiliation, or "drop-in replacement" unless carefully qualified |
| Export/crypto | If crypto or export-controlled code appears, document and comply |
| Contributors | CLA or DCO optional but CONTRIBUTING.md states licence terms for PRs |

**This guide is not legal advice.** When stakes are high (commercial distribution, trademark-adjacent naming, format reverse-engineering at scale), obtain qualified counsel.

### 2.6 Never Crash Discipline

- **Deny lints on production crate roots:** `unwrap`, `expect`, `panic!`, `unreachable!`, `todo!`, and `unimplemented!` are forbidden at the crate root and in public API functions. CI runs `cargo deny` with these lints enforced.
- **Isolated `unsafe` only.** A single, well-documented crate may use `unsafe` if it has `SAFETY:` comments at every raw pointer dereference and the crate is excluded from the `deny` lint for `unsafe_code`. All other crates have `unsafe_code = "forbid"`.
- **Last-resort panic guard at command dispatch.** The top-level command handler catches any unhandled error and presents a minimal recoverable dialog rather than aborting the process.
- **Hostile input numbers checked/saturating.** All input-derived integers use checked arithmetic, `saturating_*`, or `wrapping_*` operators. Bounded recursion depth is enforced.
- **Crash fix ⇒ regression test.** Every crash fix must ship with a regression test that originally panicked before the fix was applied. The test must stay green; if it ever fails, the fix is reverted.

---

## Authority — absolute and non-negotiable (guide preamble, 2026-11-18)

This ruleset implements guide.md, the single source of truth for every agent
on a suite repository. Every rule in it is mandatory unless a later, stricter
rule supersedes it. Where two rules conflict, **the stricter one wins**. Where
the guide conflicts with a skill, plugin, command, addendum, or external
protocol, **the guide wins** — except where the addendum is explicitly named
as binding on client/Rust/session (§18).

**No agent may waive, weaken, or "interpret around" a rule. No agent may edit
guide.md to remove a rule** — guide changes go through the human-owned private
maintenance repository (§18).

If a rule cannot be followed due to a genuine blocker, the agent must:
(1) stop, (2) state the blocker in plain language (§5.5 item 11), (3) append a
note to `docs/process-feedback.md`, and (4) wait for a human decision.
**Silent deviation is the only unforgivable failure.**

**Runtime (guide preamble, 2026-11-22).** This guide targets **DeepSeek Harness (DSH)** as the agent runtime. The only supported tool path is DSH plus the suite's `.dsh/` layout (§12) and the `ci.sh` verification shim (§4). Other agent runtimes (Claude Code, Cursor, Codex, Aider, OpenCode, etc.) are **not supported**: a reference to one of them appears only when the guide is describing material for adaptation, never as an installation target.

**User direction (owner directive, 2026-11-19).** All user-directed input is
handled by the agent end-to-end: the human directs in goals and priorities
("does this work / feel right?"); the agent executes every downstream step —
specification, tickets, implementation, CI, PR, merge, release, tagging,
install verification — without returning mechanical work to the human. The
human's remaining touchpoints are exactly the guide's gates: scope/research
approval (§2.3), release honesty, and "does this work / feel right?" on a
downloaded artefact.

---

## 3. Technology Stack (guide §3)

Defaults when the human has no strong preference:
- **Language:** Pure Rust when suitable
- **GUI:** egui + eframe + wgpu, or iced
- **Platform priority:** (1) human's local OS first (2) all practical platforms via CI
- **Persistence:** SQLite or lightweight local store
- **Architecture:** Command-centric
- **Suite shared crates:**
  - `crash-report` — panic hook, diagnostics, optional GitHub Issue create
  - `update-check` — GitHub Releases API, version compare
- **CI:** Multi-OS builds, tests, release artefacts on tag
- **Free-tier infrastructure discovery:** see §25.6 for GitHub Actions alternatives, CI/CD, monitoring, database services with permanent free tiers.

---

## 4. Local-First Testing + GitHub Actions for Heavy Work

1. Produce a binary that runs on the human's local OS as early as practical.
2. Prefer GitHub Actions for full workspace builds, large tests, Clippy, multi-target packaging.
3. Pattern: push → CI builds → download artefact / run binary for UX testing.
4. Never launch unbounded local builds that risk freezing the PC.
5. **HARD BUILD RULE — NEVER REVERT:** No model, agent, or session may run `cargo` or any other resource-intensive toolchain locally. All Rust/Node/test/build commands run **only** in GitHub Actions via `~/.config/dsh/scripts/ci.sh`. This is enforced globally by the `dsh-ci-only` plugin and is non-negotiable for every app in this suite. Local work is edit/read/git only.
   **Bootstrap fallback (if the plugin is not installed):** a git `pre-commit` hook rejects any staged file containing `cargo build`, `cargo test`, `cargo run`, `cargo clippy`, `cargo fmt`, `npm run`, `pnpm`, or `yarn` in an invocation context; a CI check fails any PR that commits `target/`, `dist/`, `node_modules/`, or `*.pdb`/`*.dSYM`. The plugin is the fast path; the hook + CI check is the guarantee.
6. Repository **must be public** for Actions and Releases.

### Required CI & Release Workflows (Early)

**From day one** (even before features):

`.github/workflows/ci.yml`:
- Format, lint/Clippy, tests
- Build primary local OS target
- Caching + artefacts

`.github/workflows/release.yml`:
- Trigger on semver tags (`v*.*.*`)
- Build release artefacts
- Publish **GitHub Release** with body from `CHANGELOG` or generated notes
- Attach binaries + checksums when practical

Do not wait until the app is "done" to learn the release path.

### ci.sh usage and result interpretation

Heavy verification (builds, tests, clippy, packaging) runs only through `~/.config/dsh/scripts/ci.sh`:

- `ci.sh run "<description>"` — commits the working tree to a branch, pushes it, opens or updates a pull request so CI triggers, **waits for all checks to complete**, prints the outcome including failing job logs.
- `ci.sh status` — current check state.
- `ci.sh logs` — failing job logs.
- `ci.sh diff` — what would be committed.
- `ci.sh merge` — merges the current PR from the Dev account once checks are green.

**Agent rule — never report back until CI is complete.** When an agent invokes `ci.sh run`, it must wait for the command to finish and report the final `=== CI RESULT: ... ===` line before reporting any outcome to the human. No interim "CI triggered" or "checks running" updates — only the final verified result.

`ci.sh` asserts, before committing or pushing:
- the git identity `"<App Name> Dev" <dev@example.com>` (author and committer),
- the authenticated GitHub login (`gh api user --jq .login`) equals the Dev account login recorded in `DECISIONS.md`.

If any assertion fails, `ci.sh` refuses and prints the concrete reason (see §5.5 item 11). No new GitHub accounts are created for authoring; the existing Dev account is the machine identity (see II.26.7).

`ci.sh` ends with one of these result lines:

| Line | Meaning |
|---|---|
| `=== CI RESULT: SUCCESS ===` | Every check passed. |
| `=== CI RESULT: FAILURE ===` | A check failed. Fix the code and re-run. |
| `=== CI RESULT: TIMEOUT ===` | Checks were still running when the timeout expired. Re-run to pick up the result. |
| `=== CI RESULT: NO CHECKS ===` | GitHub registered no check runs — investigate `.github/workflows`. |

A repository may declare which checks are authoritative in a `.ci-required.txt` at its root (one check name per line, exactly as GitHub reports it). Checks not listed are still reported but do not decide the verdict — use this when a workflow needs repository secrets that a fork cannot have. A check missing from `.ci-required.txt` is non-blocking by design; red checks that appear in the file block the merge.

Do not wait until the app is "done" to learn the release path.

## 5. Operating Model — Chat, Task, and Pull Request (guide §5)

```
Chat (Plan + Research)  →  Task (Execute)  →  Pull Request (Review)  →  Local Test  →  Merge
→  Semver Release + Release Notes  →  Update ROADMAP
```

### 5.1 Modes

**Chat (Orchestrator):** discovery, public research, architecture, Task briefs, roadmap. No large code dumps; no heavy local builds.

**Task (Coder):** one bounded vertical slice; implement; test; commit; push; PR.

### 5.2 Free Model Policy

1. **DSH free models** first.
2. When exhausted → **NVIDIA free models** via API.
3. Process unchanged across providers.
4. No Ollama / local model hosting.
5. Orchestrator discovers available free models and assigns best per role. Record in `docs/current-model-map.md`.

### 5.3 Canonical Agent Roles

`.dsh/agents/`:

- **Orchestrator** — plan and research only; no implementation. May manage a sub-agent dispatch (§5.6.3 `/implement-spec`) or an agent team (§5.7).
- **Coder** — execute one Task.
- **Explorer/Researcher** — gather public context.
- **Reviewer** — check PR vs acceptance criteria + clean-room rules. Read-only. May serve as the plan-approval gate in §5.7.4 or as the `/code-review` executor in §5.6.3.
- **Designer** — owns design vision, capability inventory, and parity review (II.27).

No other role names are used elsewhere in this guide. Read-only roles: deny edit/bash. Multi-agent team execution uses these same roles plus specialisations (front-end Coder, back-end Coder, QA Reviewer) — see §5.7.3.

### 5.4 Working Protocol

1. Read local guide + repo tracking files.
2. Discover free models; assign roles.
3. Chat vs Task deliberately.
4. Commit + push; CI; PR; local UX test when binary available.
5. Never proprietary material; never freeze the PC.

### 5.5 House Rules (Storytold / Suite Standard)

These **14** rules apply in every suite repository. They are enforced through the shared CI house-rules check (see §4) and the `craft-contrib` standards repo. A later rule may be stricter, not weaker.

1. **A person is accountable.** Whoever runs an agent answers for what it opens and for its review comments.
2. **Check, then claim.** Before writing code, read the issue comments, search pull requests for `#N` and the keywords, and check recent commits on `main`. Comment on the issue first. Release a claim you will not finish.
3. **Verify, don't guess.** Open a definition before using it. Confirm the cause in the code, even when someone names a location.
4. **Keep fixes narrow and tested.** Fix what the issue describes, where the bug is. Add a test that fails without the fix.
5. **Review the diff before opening a pull request.** Identifiers exist as used, the test fails without the fix, nothing unrelated changed, and the description is true.
6. **Look for a better pull request.** If another PR fixes the same problem and is better or equal in scope and quality, close yours and say why.
7. **Report only what ran.** In CI status summaries, report only the jobs that actually executed. A job skipped because a secret was unavailable, cancelled by concurrency, or whose test was `#[ignore]`d is reported as **not run**, never as pass or fail.
8. **Main `main` through PR only.** No direct pushes to `main`; every change must land via a verified pull request merge. Branch protection with `enforce_admins: true` and the required checks in `.ci-required.txt` enforces this.
9. **Secrets are never committed.** No API keys, tokens, or credentials in git or release binaries. Crash reports and update checks use user-supplied tokens only; no embedded privileged bot token ever ships in a client binary.
10. **CI stays shared.** Workflow definitions live in a shared suite repo; feature PRs must not alter the shared CI definitions — make a new issue/PR in the shared repo instead.
11. **Guards say why.** Any agent or tool that blocks or gates work must state the concrete reason in plain language — not just "blocked" or "failed."
12. **Instructions must be readable by everyone.** No notes about one machine's private configuration; all steps must be runnable by any suite contributor.
13. **PR movement SLA.** Open PRs receive a first-response comment within 7 days and a decision (merge, revise, or close) within 14 days; stale PRs (30 days unmodified) are automatically candidates for closure.
14. **GitHub actions are authored by the Dev identity — never personal, never fabricated human attribution.** PRs, issues, comments, releases and tags are created by the existing Dev account (II.26.7). No PR description, merge commit, or issue comment claims an agent or model as a person. Models are credited separately (About ▸ Models), never as people and never as the personal account.

CI integrity check: A CI job verifies that the repository's `AGENTS.md` house-rules block matches the current version stored in the shared `craft-contrib` standards repo. If the local guide.md is updated, the agent must also update the repo's `AGENTS.md` and commit the change via `ci.sh`.

---

### 5.6 DSH Agent Workflow Protocol

**Authority.** This section is binding on every agent. It refines how the Orchestrator, Coder, Explorer/Researcher, Reviewer, and Designer roles operate inside DSH. Where it conflicts with the Authority preamble, the HARD BUILD RULE (§4 item 5), the Dev-identity rule (II.26.7), the clean-room rules (§2), the secrets rule (II.29), or branch protection (§9.3), those rules win — the reconciliation is stated explicitly in §5.6.5.

**Deployment.** The protocol lives in the project root `AGENTS.md` (or the global `~/.config/dsh/AGENTS.md`). It is synced with `craft-contrib` per the §5.5 CI integrity check: if this guide changes, `AGENTS.md` changes with it. Slash commands map to `.dsh/command/*.md`; reusable skills map to `~/.config/dsh/skill/`.

**Session lifecycle.** DSH sessions are ephemeral. Durable state lives on disk (specifications, tickets, decisions, progress) — never in session history. `/new` starts a fresh session; every phase boundary is a `/new`.

#### 5.6.1 Context management rules

- **Smart zone (< 150,000 tokens).** Keep every DSH session strictly below 150,000 tokens. Reasoning accuracy and instruction adherence are preserved only inside this zone.
- **Dumb zone (≥ 150,000 tokens).** Stop work immediately. Reasoning degrades, `AGENTS.md` constraints are forgotten, file paths hallucinate. Do not "push through" a long session — `/new` and re-read the disk state.
- **Session reset protocol.** Run `/new` between distinct workflow phases: interrogation → specification → ticket generation → implementation. Never carry interrogation context into implementation.
- **Externalise state to disk.** Specifications in `docs/specs/`, tickets in `docs/tickets/`, decisions in `DECISIONS.md`, progress in `PROGRESS.md`. Session history is never a source of truth.
- **Lazy context loading.** Reference external rule files via `@` paths in `AGENTS.md` so DSH loads them only when needed. Never inline a large rules file into the initial context.

#### 5.6.2 Workflow selection matrix

Select the execution path by scope and clarity **before** modifying files.

| Case | When | Path |
|------|------|------|
| **1. Tiny Work** | Single function, bug fix, or isolated behaviour with known inputs/outputs | Skip planning and specification. `/new` → `/tdd`. |
| **2. One-Session Work** | Small feature fitting within a single 150k-token window | `/grill-me` to clarify edge cases, then build in the same session — no persistent Markdown files. |
| **3. Multi-Session Main Flow** | Medium-to-large features requiring multiple context windows | `/grill-me` → `/to-spec` → `/new` → `/to-tickets` → `/new` → `/implement` (one ticket per fresh session). |
| **4. Parallel Multi-Session Work** | Large features with independent, non-blocking components | `/implement-spec` builds the dependency graph and spawns isolated `task` sub-agents across separate Git worktrees. |
| **5. Huge and Foggy Work** | Ambiguous initiatives, unfamiliar codebases, or architectural migrations | **Wayfinder** skill (`~/.config/dsh/skill/wayfinder`) first to explore and map architectures, *then* `/grill-me`. |

**Misclassification is a defect.** A task routed as Case 1 that turns out to need a spec must be stopped, reclassified, and routed as Case 3. Do not improvise a multi-session flow inside a tiny-work session.

#### 5.6.3 Custom command specifications

All seven commands live in `.dsh/command/*.md`. Each has frontmatter that fixes its DSH agent mode.

**`/grill-me` — `.dsh/command/grill-me.md`**
- **Frontmatter:** `agent: plan` — DSH enforces read-only exploration; the agent cannot edit files.
- **Rule:** Do not write code or implementation plans until every ambiguity is resolved.
- **Action:** Ask targeted questions one by one, providing a recommended answer for each based on existing codebase patterns.
- **Goal:** Alignment on edge cases, data structures, and failure modes.
- **Clean-room check:** If the answer would come from a proprietary source, stop and route through §2.3 human gate.

**`/to-spec` — `.dsh/command/to-spec.md`**
- **Rule:** Compile all decisions from `/grill-me` into a single Markdown specification file on disk (`docs/specs/<feature>.md`).
- **Required sections:** System architecture, interface contracts, non-goals, explicit constraints.
- **Clean-room check:** A specification may cite public URLs only; no proprietary source, no pixel-perfect UI descriptions (§2.2).

**`/to-tickets` — `.dsh/command/to-tickets.md`**
- **Rule:** Parse the specification and split it into atomic, sequential work units sized to fit inside the Smart Zone (< 150k tokens each).
- **Ticket schema (saved to `docs/tickets/<id>.md`):** Objective, target file paths, blocking dependencies, verifiable acceptance criteria.
- **Ready-for-work:** a ticket is ready only when its dependencies are merged to `main` and its acceptance criteria are objectively checkable by CI.

**`/tdd` — `.dsh/command/tdd.md`**
- **Rule:** Never write production code without a failing test. Never alter tests to pass buggy implementation code.
- **Red Phase:** Write a unit or integration test for a single behaviour, then verify the test **fails** via `ci.sh run "TDD red: <behaviour>"`. The CI run must show the new test failing for the expected reason. A red run that does not fail means the test does not exercise the new behaviour — rewrite it.
- **Green Phase:** Write the minimum production code required to pass the failing test, then verify via `ci.sh run "TDD green: <behaviour>"`. CI must be green.
- **Refactor Phase:** Remove duplication and refine types while keeping all tests passing. Verify via `ci.sh run "TDD refactor: <behaviour>"`. CI must remain green.
- **Reconciliation:** the protocol's original "run the test runner via bash" is redirected through `ci.sh` per §4 item 5 — see §5.6.4 for the batching rule that keeps this affordable.

**`/implement` — `.dsh/command/implement.md`**
- **Rule:** Execute in a `/new`, single-ticket DSH session.
- **Sequence:**
  1. Read the assigned ticket file from `docs/tickets/`.
  2. Execute the `/tdd` loop for each acceptance criterion (each phase verified through `ci.sh`).
  3. Run linters, type-checkers, and the full test suite via `ci.sh run "implement: <ticket>"` — not via raw bash.
  4. Run `/code-review`.
  5. Stage and create a clean commit via `ci.sh` (which asserts the Dev identity per II.26.7). Direct `git commit` outside `ci.sh` is forbidden.
  6. Open or update the PR via `ci.sh run`; merge only via `ci.sh merge` (Dev account) once required checks are green.

**`/implement-spec` — `.dsh/command/implement-spec.md`**
- **Rule:** Prevent file collisions across parallel sub-agents.
- **Sequence:**
  1. Parse the ticket dependency graph from `docs/tickets/`.
  2. Create isolated temporary Git worktrees (`git worktree add`) for non-blocking tickets. *This is a git operation, not a build command, so it is allowed locally.*
  3. Dispatch one DSH `task` sub-agent per worktree. Each sub-agent obeys the HARD BUILD RULE — it may not run `cargo` locally; it verifies through `ci.sh` like any other session.
  4. Merge completed branches sequentially into the feature branch **only after** `ci.sh` reports green for that worktree, and **only via the Dev account** (II.26.7). No direct local merges into `main`; the feature branch itself merges to `main` through the normal PR + §9.3 path.

**`/code-review` — `.dsh/command/code-review.md`**
- **Frontmatter:** `agent: plan`, `subtask: true` — DSH runs a read-only audit in an isolated sub-agent context. This is the Reviewer role (§5.3) in DSH terms.
- **Rule:** Audit all code changes before committing or merging.
- **Axis 1 — Architectural alignment:** compliance with the specification document, `AGENTS.md` conventions, and separation of concerns.
- **Axis 2 — Tactical correctness:** off-by-one errors, unhandled null states, race conditions, missing test coverage.
- **Clean-room axis (added by this guide):** no proprietary source, assets, or RE material; public research notes are original summaries + URLs only (§2.2).
- **Secrets axis (added by this guide):** no tokens, keys, or credentials in the diff, the PR text, or the commit metadata (II.29).

#### 5.6.4 Verification mandate — reconciled with the HARD BUILD RULE

- **Never mark a task complete on assumption.** Every phase ends with a verified outcome.
- **Automated verification only.** Linters, type-checkers, and test runners are the source of truth — not the agent's reading of the diff.
- **Verification runs through `ci.sh`, never raw bash.** The protocol's original "run the test runner via DSH's bash tool" is **overridden** by §4 item 5: on this suite, no agent runs `cargo test`, `cargo clippy`, `cargo fmt`, `npm test`, or any equivalent locally. All verification is `ci.sh run "<phase>: <behaviour>"`.
- **Batching rule for affordability.** To keep a TDD cluster affordable across the two CI round-trips per phase:
  - A **red run** may batch up to 5 new failing tests if they belong to the same ticket and the CI log shows each test failing for its expected reason.
  - A **green run** may batch the production code that makes all of them pass.
  - A **refactor run** may cover the whole ticket.
  - A batched "TDD" claim without a recorded red-run failure is a defect: the CHANGELOG and PROGRESS entries must cite the red run's CI run ID.
- **What counts as verified.** A task is verified when: (a) `ci.sh` reports `=== CI RESULT: SUCCESS ===`, (b) the run ID is recorded in `PROGRESS.md`, and (c) the PR checklist (§8.3) is fully ticked.
- **What does not count.** A local edit "looks right"; a reviewer said so; the diff is short; the test "probably passes." None of these are verification.
- **Agent must wait for CI completion.** When invoking `ci.sh run`, the agent must block until the command returns the final `=== CI RESULT: ... ===` line. The agent must not report "CI triggered" or "checks running" as a completion signal — only the final verified CI result constitutes a completed verification phase.

#### 5.6.5 Reconciliation with the rest of the guide (binding)

Where the DSH protocol's original wording conflicts with a rule of this guide, the following table resolves it. The table is not optional.

| DSH protocol says | This guide requires | Resolution |
|---|---|---|
| "Run the project test runner via DSH's bash tool" | HARD BUILD RULE (§4 item 5) | Verify via `ci.sh run`, not raw bash (§5.6.4). |
| "Run automated linters, type-checkers, and test suites via bash" | HARD BUILD RULE | Same — all through `ci.sh`. |
| "Stage and create a clean Git commit" | Dev-identity rule (II.26.7) | Commit only via `ci.sh`, which asserts the Dev account. |
| "Merge completed branches sequentially" | Branch protection (§9.3), PR-only main | Merge via `ci.sh merge` from the Dev account after required checks pass. |
| `/implement-spec` sub-agents "execute parallel tickets" | HARD BUILD RULE | Sub-agents edit files in their worktree and verify through `ci.sh` — no local cargo. |
| `/tdd` "confirm failure" via bash | HARD BUILD RULE | Confirm failure via the CI log of `ci.sh run "TDD red: …"`. |
| `/code-review` "audit all code changes" | Clean-room (§2), secrets (II.29) | The Reviewer also checks the clean-room and secrets axes. |
| Any workflow phase reusing session context | Context management (§5.6.1) | `/new` between phases; state lives on disk. |

**Deviation is a process defect.** If a command file (`.dsh/command/*.md`) instructs the agent to bypass any rule in this table, the command file is wrong and must be fixed — not the guide.

#### 5.6.6 What this protocol adds to the existing workflow

The existing §5 flow (`Chat → Task → PR → Local Test → Merge → Semver Release`) is unchanged. §5.6 adds the **inside-the-session discipline** that makes each phase reliable:

- **Interrogation is a phase, not a preamble.** `/grill-me` is read-only and ends with explicit alignment, not with code.
- **Specifications and tickets are files, not memories.** `docs/specs/` and `docs/tickets/` survive `/new` and survive session crashes.
- **TDD is verified through CI, not asserted.** The Red phase has a recorded CI run ID or it did not happen.
- **Review has two axes (architecture + tactics) plus clean-room and secrets.** The Reviewer role (§5.3) is the executor; `/code-review` is the mechanism.
- **Large work decomposes safely.** Parallel worktrees and `task` sub-agents are bounded by the HARD BUILD RULE and merged by the Dev identity.

#### 5.6.7 Failure modes this protocol is designed to prevent

| Failure mode | Prevention |
|---|---|
| Agent "finishes" a task that does not build | §5.6.4 — green CI run ID required. |
| Agent writes a test that passes for the wrong reason | §5.6.3 `/tdd` Red phase — red run must fail for the expected reason. |
| Long session degrades and silently skips rules | §5.6.1 — Smart Zone hard limit + `/new` between phases. |
| Specification drifts from code | §5.6.3 `/to-spec` + `/code-review` Axis 1. |
| Parallel agents collide on files | §5.6.3 `/implement-spec` — isolated worktrees, dependency-graph dispatch. |
| Agent claims TDD without a red run | §5.6.4 — red run CI ID recorded in PROGRESS and CHANGELOG. |
| Command file contradicts the guide | §5.6.5 — command files are fixed, not the guide. |
| Agent reports "done" before CI checks complete | §5.6.4 — agent must block on `ci.sh run` until final `=== CI RESULT: ... ===` line. |

### 5.7 Comprehensive Multi-Agent Architecture and Execution Protocol

**Authority.** This section is binding on every agent that operates within a coordinated team. It complements §5.6 (the single-session DSH protocol): §5.6 governs how one agent works through one workflow, §5.7 governs how *multiple* agents work together on one project. Where the two conflict, §5.6 wins on session mechanics; §5.7 wins on team topology. Where either conflicts with the Authority preamble, the HARD BUILD RULE (§4 item 5), the Dev-identity rule (II.26.7), the clean-room rules (§2), the secrets rule (II.29), or branch protection (§9.3), those rules win — the reconciliation is stated explicitly in §5.7.5.

**Deployment.** The protocol lives in `AGENTS.md` at the project root (or `~/.config/dsh/AGENTS.md` for global defaults). It is synced with `craft-contrib` per the §5.5 CI integrity check. Team definitions live in `.dsh/agents/*.yaml` (one file per teammate role); the team's shared task list lives on disk (`docs/tickets/`), not in session memory.

**When this protocol applies.** A multi-agent team is warranted only when §5.6.2 selects **Case 4 (Parallel Multi-Session Work)**. Case 1–3 and Case 5 are single-agent paths; do not spawn a team for them. A team that is spawned without a Case 4 classification is a process defect.

**Source of the concepts.** The team topology below is adapted from public multi-agent workflow patterns. It is not an installation of any specific course, video, or third-party product; it is original prose for this guide, reconciled with the rules above.

#### 5.7.1 Architectural foundations: agent teams vs. sub-agents

- **Sub-agents** are isolated workers that execute a single bounded task and return final results to a parent agent. They have no peer communication channels. In this guide, sub-agents are the `/implement-spec` workers (§5.6.3) and any `task`-tool invocation.
- **Agent teams** are coordinated groups of independent, specialised teammates (e.g. front-end developer, back-end engineer, QA agent) managed by a central orchestrator over a shared task list. In this guide, the orchestrator is the **Orchestrator** role (§5.3); teammates are named roles in `.dsh/agents/*.yaml`.
- **Inter-agent communication.** Teammates may communicate directly, pass work streams back and forth, and trigger loops (e.g. a QA agent rejecting code and returning it to developers for revision) **without** routing every message through the human operator. Every such exchange is logged to disk (`docs/team-log/<ticket-id>.md`) so it survives `/new` and is auditable.

**One orchestrator, many teammates.** A team has exactly one Orchestrator. A team with two orchestrators is not a team — it is two conflicting agendas. If the human needs to split a goal across two orchestrators, split the goal first into two independent goals, each with its own team.

#### 5.7.2 Project setup and configuration

- **Feature activation.** Agent teams are experimental. Enable them explicitly in the local settings file:
  - **DSH:** the feature-flag is set in `.dsh/settings.local.json` (or the user's global `~/.config/dsh/settings.local.json`). Do not invent a flag name; check the current DSH documentation and record the exact key in `DECISIONS.md`.
  - The flag is per-project, never per-machine. Committing a machine-specific flag to a public repo violates §5.5 item 12 (readable instructions). Only the *documented* flag goes into the template; the *value* stays local.
- **Documentation pre-loading.** Import the project's official documentation into `docs/` and generate a structured Markdown reference (`docs/reference/index.md`) so teammates can query system architecture and team patterns locally during execution. This is how the team avoids re-reading raw source files and burning context (see §5.6.1, lazy context loading).
- **Permission inheritance.** Sub-agents and teammates inherit permissions from the Orchestrator session. Pre-approve **only** the read-only and git commands that are safe (git status, git diff, git log, `gh api user`, read-only shell inspection). Pre-approval of `cargo test`, `cargo build`, `cargo run`, `cargo clippy`, `cargo fmt`, `npm test`, `npm run`, `pnpm`, `yarn`, or any equivalent build/test command is **forbidden** — §4 item 5 forbids them locally regardless of what a settings file says. Pre-approved commands must be listed in `docs/dev.md` with the date and the reason.

**Why "bypass permissions" is refused by this guide.** The "bypass permissions" pattern bypasses every guard the suite's house rules depend on (§5.5 item 11 "guards say why", §5.5 item 9 secrets, §4 item 5 HARD BUILD RULE). The suite's equivalent is: pre-approve a **specific allowlist** of read-only and git commands, listed in `docs/dev.md`, and refuse every other command unless the Orchestrator (not the teammate) explicitly approves it for that ticket. See §5.7.5.

#### 5.7.3 Prompt engineering and team structuring

- **Natural-language invocation pattern.** When spawning a team, state:
  ```
  Create a team of [X] teammates using [model id from docs/current-model-map.md] for [Goal].
  Teammate 1 is [Role] responsible for [Scope].
  Teammate 2 is [Role] responsible for [Scope].
  Teammate 3 is [Role] responsible for [Scope].
  ```
  The exact prompt is written to `docs/specs/<feature>.md` (Appendix: Team Definition) before the team is spawned, so the team structure survives `/new`.
- **Goal establishment.** A clear global goal is stated at the start. Every teammate understands both its individual scope and the broader mission. "The goal" is one paragraph, quoted verbatim from the spec, not paraphrased per-teammate.
- **File ownership boundaries.** Each agent is assigned specific file ownership. Two agents must not modify the same files simultaneously. Enforce with the §12 parallel-agent hygiene rules: each agent may only edit files it "owns", ownership recorded in an `OWNERS` file or agreed convention, and file-ownership conflicts fail the ticket at review.
- **Team sizing.** Restrict team size to **2 to 5** specialised agents. Do not spawn swarms of 10+. The 2–5 range is chosen to control token consumption and execution cost; it is not a starting suggestion that may be expanded "if it seems helpful." A team of 6+ requires a written justification in `DECISIONS.md` and Orchestrator approval per §5.7.5.
- **Explicit deliverables.** Each team defines concrete end outputs:
  1. A running local application (verified by CI artefact download, §6.2 — never by a local build).
  2. A comprehensive test report (pass/fail) recorded in `docs/qc/team-<feature>-report.md`.
  3. An architectural documentation file (`docs/specs/<feature>-decisions.md`) detailing key decisions and execution steps.

#### 5.7.4 Execution, monitoring, and shutdown protocols

- **Visibility via Tmux.** Running DSH inside a terminal multiplexer (Tmux) enables split-pane monitoring — real-time visibility into agent reasoning, file changes, and inter-agent dialogue. This is recommended for interactive sessions but is not a substitute for the on-disk logs: `docs/team-log/<ticket-id>.md` is the audit trail.
- **Visibility via DSH task panes.** DSH's `task` tool surfaces parallel sub-agents in its own UI. If a UI pane is not available (headless CI), the `docs/team-log/<ticket-id>.md` file is the sole source of truth and must be complete enough to reconstruct what happened without the pane.
- **Plan approval mode.** Teammates propose execution plans; the Orchestrator (or a dedicated lead reviewer agent) approves them **before** code is written. This is the multi-agent counterpart of `/grill-me` (§5.6.3): the plan is written to `docs/tickets/<id>.md` (Plan section), reviewed by the Orchestrator, and only then moves to Implementation. A teammate that writes code without an approved plan is out of scope.
- **Graceful shutdown.** At task completion, the Orchestrator issues shutdown requests to teammates. Each teammate:
  1. Confirms completion (or states incompleteness in plain language, §5.5 item 11).
  2. Flushes temporary state to disk (`docs/team-log/<ticket-id>.md`).
  3. Saves work (commits via `ci.sh`, not raw `git commit`).
  4. Cleans up workspace artefacts (temporary worktrees removed, staged files unstaged).
  The Orchestrator records shutdown in `PROGRESS.md` with the CI run ID of the final green run.

- **Teammates must wait for CI completion.** When a teammate invokes `ci.sh run` for verification, it must block until the command returns the final `=== CI RESULT: ... ===` line. No teammate may report a task as "done" or "verified" based on a triggered CI run — only the final verified CI result counts.

#### 5.7.5 Reconciliation with the rest of the guide (binding)

Where the protocol's original wording conflicts with a rule of this guide, the following table resolves it. The table is not optional.

| Team protocol says | This guide requires | Resolution |
|---|---|---|
| "Bypass permissions" | §5.5 item 11, §4 item 5 | Refused. Pre-approve only a documented read-only + git allowlist in `docs/dev.md`; everything else needs Orchestrator approval per ticket. |
| "Pre-approve standard tools" | §4 item 5 (HARD BUILD RULE) | Pre-approval of `cargo*`, `npm*`, `pnpm`, `yarn`, or any build/test command is **forbidden** — they run only via `ci.sh`. |
| Teammates "run tests" locally | §4 item 5 | Teammates verify through `ci.sh run` — same rule as §5.6.4. |
| Teammates "stage and commit" | II.26.7 (Dev identity) | Commit only via `ci.sh`; every commit carries the Dev identity. |
| Feature flag JSON in `.dsh/settings.local.json` | §5.5 item 12 (readable everywhere) | The flag *name* is documented in `DECISIONS.md`; the flag *value* stays local, never committed. |
| "Allow agents to loop without routing through the human" | §5.5 item 1 (a person is accountable) | The loop is logged to `docs/team-log/<ticket-id>.md`; the human can audit it. Autonomy is bounded by the log, not by absence of oversight. |
| "Team size 2 to 5" | This guide's token-cost discipline | Binding. 6+ requires `DECISIONS.md` justification and Orchestrator approval. |
| "Explicit deliverables include a running local application" | §6.2 (local build forbidden) | "Running local application" means the **CI artefact** downloaded and run, never a local build. |
| Plan approval "by the Orchestrator" | §5.6.3 `/grill-me` discipline | The Orchestrator's approval is recorded in `docs/tickets/<id>.md` (Plan section) before Implementation begins. |

**Deviation is a process defect.** If a teammate configuration (`.dsh/agents/*.yaml`) instructs the agent to bypass any rule in this table, the configuration is wrong and must be fixed — not the guide.

#### 5.7.6 What this protocol adds to §5.6

§5.6 governs one agent through one workflow. §5.7 adds the topology and the safety rails that let multiple agents work together without breaking the suite's guarantees:

- **A team has a classification.** Case 4 in §5.6.2 is the only case that warrants a team. Others do not.
- **A team has one Orchestrator.** No dual-orchestrator setups.
- **A team is 2–5 agents.** Not a swarm.
- **A team has a shared task list on disk.** Not in session memory.
- **A team logs inter-agent dialogue to disk.** Not only to a pane.
- **A team's teammates never bypass permissions.** They operate inside a documented allowlist; everything else is Orchestrator-approved per ticket.
- **A team shuts down gracefully.** Every teammate confirms completion, flushes state, commits via `ci.sh`, cleans up.

#### 5.7.7 Failure modes this protocol is designed to prevent

| Failure mode | Prevention |
|---|---|
| Team spawned for a task that only needed one agent | §5.7 "When this protocol applies" — only Case 4 warrants a team. |
| Two agents edit the same file, one overwrites the other | §5.7.3 file ownership + §12 parallel-agent hygiene. |
| Token cost explosion from a 10+ agent swarm | §5.7.3 team sizing 2–5; justification required for more. |
| Teammate bypasses permissions and runs `cargo` locally | §5.7.5 reconciliation — pre-approval of build commands is forbidden. |
| Inter-agent loop unobservable to the human | §5.7.4 Tmux / task pane + on-disk log (§5.7.1). |
| Teammate writes code without an approved plan | §5.7.4 plan approval mode + `docs/tickets/<id>.md` Plan section. |
| Team shuts down with work lost in memory | §5.7.4 graceful shutdown protocol. |
| "Bypass permissions" setting silently weakens suite guarantees | §5.7.5 — refused; allowlist documented in `docs/dev.md`. |
| Teammate reports "done" before CI checks complete | §5.7.4 — teammate must block on `ci.sh run` until final `=== CI RESULT: ... ===` line. |

## 6. Progressive Releases & Roadmap (guide §6)

### 6.1 Versioning Policy

- Follow **Semantic Versioning 2.0.0** (MAJOR.MINOR.PATCH).
- While the major version is `0` (`0.x.y`), the public API and on-disk formats may still change; treat every minor bump as potentially breaking for early adopters.
- **Pre-releases** use `0.x.y-alpha.N`, `0.x.y-beta.N`, or `0.x.y-rc.N`. The in-app updater **does not** offer pre-releases unless the running build is itself a pre-release of the same line.
- Tags must be exact semver strings (`v0.1.0`, `v0.2.0-rc.1`, etc.). The release workflow triggers only on such tags.
- A tag may be created only after the corresponding commit is on `main` and the required CI checks are green.

### 6.2 v0.1.0 — First Functional Test Build (Hard Gate)

**v0.1.0 is not "hello world."** It is the first *useful* installable build. It must include:

| Requirement | Detail |
|-------------|--------|
| Installable / runnable | CI artefact for the primary local OS, downloaded and run. Local build is never the path to a working binary. |
| One real workflow | At least one end-to-end user task with **real** data model + persistence (not mock UI) |
| App identity | Original name, window title, basic theme |
| Crash stub | Panic hook **captures and logs locally** (a minimal dialog is acceptable). Network submission may slip to 0.2.x; the §9 gate applies to the full flow. |
| Update-check stub | Reads current version; optional "check later" placeholder OK if API wired in 0.2.x |
| CI green | Format, lint, tests, primary OS build |
| Docs | README with build/run instructions; CHANGELOG entry for 0.1.0 |
| Legal baseline | LICENSE, ATTRIBUTION started, no secrets, no proprietary assets |
| Tag | `v0.1.0` published as GitHub Release with short honest notes ("first functional test build; limited scope") |

Until v0.1.0 exists, do **not** expand into exhaustive multi-feature research theatre. Ship the vertical slice first.

### 6.3 After v0.1.0

1. Expand **`ROADMAP.md`**: stages, feature breadth %, ready-for-work %, semver milestones.
2. Public research notes (`docs/research/`) with **human gate** before large Tasks.
3. Frequent semver releases (`0.1.1`, `0.2.0`, …) with **detailed** GitHub Release notes (Added / Changed / Fixed / Known limitations / Token metrics — see §9).
4. Update readiness % only from evidence (CI + local verification).

### 6.4 Release Notes Standard

Each GitHub Release body includes:
- Summary (1–3 sentences, honest)
- Added / Changed / Fixed
- Known limitations
- Roadmap progress (links or % if useful)
- Artefact list + checksums when available
- **Token usage metrics** — total tokens consumed since project start, and tokens consumed in this release (see §9 and `docs/tokens.md`)

`CHANGELOG.md` (Keep a Changelog style) is the source of truth. The GitHub Release body is derived from the corresponding CHANGELOG section.

### 6.5 Emergency / Security Releases

When a critical vulnerability or data-loss bug is confirmed:
1. Open a private advisory / issue (do not discuss exploit details publicly until a fix is ready).
2. Prepare a minimal patch on a private branch; keep the change as small as possible.
3. After internal verification, merge to `main`, tag a new patch version (e.g. `v0.3.1`), and publish the GitHub Release with a clear "Security" or "Critical" note.
4. The in-app updater will surface the new version through the normal channel. If the bug is severe, a short-lived banner or forced-update path may be added for that release only.
5. After the fixed release is public, publish the advisory and credit reporters as appropriate.
6. Never ship a security fix that re-introduces proprietary material or weakens the clean-room / secrets rules.

---

## 7. Suite Infrastructure — Crash Reporting & Update Check (guide §7)

### 7.1 Crash Detection & Bug Reporting

- Panic hook / error boundary: app version, OS, backtrace (scrub secrets/paths as needed), optional user notes.
- UI preview before send; user can cancel.
- **GitHub Issues:** optional user token or device flow to create issues (`crash`, `bug`, `user-report`).
- Fallback: pre-filled New Issue URL if no token.
- **Never** embed a privileged bot token in client binaries.
- Document privacy in README.

### 7.2 Update Check — Standard Flow

Every suite app embeds the shared `crates/update-check` crate and drives **one Updater instance per app**. The crate owns the whole flow; the app calls `tick()` / `show()` each frame and acts on the returned action, so every app handles failures, dropped workers, repaints and restart identically.

#### Check mechanics

- Query GitHub Releases API (`/repos/<owner>/<repo>/releases?per_page=30`) on a background thread; compare semver to the running version.
- **Quiet startup check, once per run:** a failure is logged, never shown as a modal; the app starts normally.
- **Help ▸ Check for Updates (explicit):** reports a visible status — Checking / Up to date / Update available / Failed (with the error). A check already running becomes the requested one.
- Fail soft offline; conditional GETs (ETag / If-None-Match); brief cache; handle GitHub's rate limit gracefully (tell the user, stop checking until reset). Unauthenticated GitHub API limits are low (~60 req/h); prefer conditional requests and short caching.
- Semver rule: a dev/integration build (e.g. `0.17.0-integration.20261029.f5b3f123`) is **not** offered the release it already contains; a release is offered only when strictly newer. Pre-releases are offered only to other pre-release builds of the same line.

#### Primary in-app update UX (canonical)

Every suite app provides an **integrated update path entirely within the application GUI**. The user is never *required* to leave the app, open a browser, or run commands.

1. **Background check** (once per run). If up to date → no UI.
2. **Subtle non-blocking banner/toast** when a newer release is found: "New version available: X.Y.Z — Select to update."
3. Selecting the banner opens an **inline dialog** (not a blocking system modal) containing:
   - Current version → new version
   - Short release highlights (Added / Changed / Fixed, derived from release notes)
   - Primary button: **Update & Restart**
   - Secondary button: **Later**
4. **Update & Restart** runs the full pipeline inside the app:
   - Download the platform-specific artefact to a temporary location
   - Verify SHA-256 against `SHA256SUMS.txt` **and** the minisign signature
   - Run stale-version remediation (remove old app-drawer / Start Menu entries)
   - Stage under a hidden name → rename into place → restart through the normal close path
5. **Later** dismisses the banner; any in-progress download continues and reports through the updater.

#### Secondary / fallback actions

These appear only when full self-update is not possible or the user explicitly requests them:

- **Download & Install** — offered when the running copy cannot be safely replaced in-place (unsupported format, missing signature, package-manager-owned install). Downloads, verifies, then asks the user to confirm opening the installer. Never auto-runs the installer without confirmation.
- **Open release page** — last-resort link that opens only a safe `https` GitHub Releases URL. Used when even the installer path is unavailable. Prefer keeping the user inside the app whenever possible.

#### Signature verification (required)

- Every release ships `SHA256SUMS.txt` and a **minisign** signature over that file (or over the individual artefacts).
- The public key is **pinned in the update-check crate** (or loaded from a compile-time constant). Key rotation requires a new app release that contains the new public key.
- Verification failure (missing signature, bad signature, hash mismatch) aborts the update and surfaces a clear error. The running version is left untouched.
- Document the current public key fingerprint in `docs/update-signing.md` and in the release notes of any key-rotation release.

#### Signing-key handling and compromise procedure

- The minisign private key lives in a hardware token or a scoped secret accessible **only** to the release workflow. It is never on a developer machine.
- `SHA256SUMS.txt` is signed **after** artefact upload and its signature is attached to the release.
- If the private key is suspected compromised:
  1. Publish a new app release whose bundled public key is a newly generated pair, before any further artefact is signed with the old key.
  2. Mark the affected release (or releases) with a clear "Security" banner in the release notes and in the in-app release-notes popup.
  3. Update `docs/update-signing.md` with the old fingerprint, the new fingerprint, and the reason.
  4. Rotate the secret in the release workflow.
- Never silently re-sign existing artefacts with a new key; that creates ambiguity and hides the incident.

#### Install-type detection & applicability

- Detect how the running copy was installed: macOS `.app`, Windows MSI / portable zip / standalone, Linux AppImage / deb / rpm / pacman / Flatpak / standalone.
- **Development builds never self-update.** A `cargo run` / build-tree copy receives only the "Open release page" fallback.
- **Package-manager-owned copies** (Homebrew, Nix, Flatpak, scoop, winget, etc.) defer to their package manager; the in-app updater only notifies.
- Only a packaged copy in a supported format performs full self-update.
- **Never replace a running binary.** A restart-helper waits for the app to exit, finishes OS-locked operations, then starts the new version.
- **Staged → verify → rename → activate.** Any failed step removes the staged copy and leaves the running version untouched.
- **Unsaved-work gating:** an update restart waits while the app holds unsaved work.
- **Anti-loop marker:** a version just installed is recorded with a timestamp and not re-offered for 24 h.
- **Failure recovery:** a failed self-update writes its error to a log shown once at next start; the update remains on offer.
- An env var / build flag **disables update checks entirely** for distro packaging.

#### Token & authentication for update-check / crash-report

- **No privileged bot token** is ever embedded in client binaries.
- Optional user-supplied GitHub token (or device-flow) may be used for higher rate limits or for creating crash/bug issues.
- **Minimal scopes** recommended: `public_repo` (or finer-grained equivalents) only.
- **Persistence:** if a user token is stored, it must use the platform credential store / keychain (or an encrypted file with a machine-bound key). Never store tokens in plain text or in the app's normal config directory without encryption.
- Document the privacy implications in the README / privacy note.

#### Localization

All updater messages follow the app's interface language. Network error text may pass through as received.

## 8. Path to a Finished App + PR Discipline (guide §8)

### 8.1 Phased Tasks

1. Foundation — skeleton, CI, launch, theme, PR template, release workflow
2. **v0.1.0** — first functional test build (Section 6.2)
3. Roadmap expansion + public research (human-gated)
4. Shared crash-report + update-check crates
5. Remaining workflows — one Task / release slice at a time
6. Polish, a11y, empty/error states
7. Multi-platform CI + installers
8. Semver releases until Section 9 gates pass
9. Declare finished only when Section 9 is satisfied

### 8.2 Definition of Done (Every Task)

- Acceptance criteria met with real behaviour — no fake UI, no mock data.
- Unit tests cover happy path, boundary, and failure/edge cases; headless domain tests need no display.
- Code is formatter- and linter-clean in the project's own toolchain (for Rust, CI runs `cargo fmt --all -- --check` and clippy with `-D warnings`; no toolchain command is run locally).
- CI green on GitHub Actions: the project's format → lint → test → release-build chain passes on every supported platform.
- No fake success states, dead controls, or placeholder-only data.
- **No personal data and no secret in the change.** The scan in II.29 is clean across the working tree, the history and the commit metadata. Placeholder, seed, sample and fixture values are obviously synthetic (`Jane Doe`, `dev@example.com`, `example.com`), never a real person's.
- `CHANGELOG.md`, `PROGRESS.md`, and `REQUIREMENTS_TRACEABILITY.md` updated with exact CI results.
- Change is small and reviewable; PR opened using the template, **opened from the Dev account** (II.26.7).
- GitHub hygiene gate before any major push to main or version release: Agent must review the GitHub repository health — verify README is accurate and matches current build, clear stale or broken CI checks, close or triage open PRs/issues that are obsolete, remove draft PRs and stale issue labels, ensure the repo description topics/badges are correct, and confirm Releases page is clean. No release is published with lingering errors, stale PRs, or an out-of-date README.

### 8.3 Pull Request Template (Required)

Create `.github/PULL_REQUEST_TEMPLATE.md`:

```markdown
## Summary
<!-- What this PR does in 1–3 sentences -->

## Type
- [ ] Feature
- [ ] Fix
- [ ] Docs / research (summaries + URLs only)
- [ ] Infra (CI, release, shared crates)
- [ ] Refactor

## Checklist
- [ ] PR opened by the Dev account (II.26.7) — not the personal account
- [ ] Acceptance criteria for the Task are met with **real** behaviour (no fake UI)
- [ ] Tests added or updated
- [ ] CI is green (or will be after merge of required base)
- [ ] No unbounded local full builds used to "verify" on a constrained machine
- [ ] No proprietary source, assets, or RE material introduced
- [ ] Public research notes are original summaries + URLs only (if applicable)
- [ ] ROADMAP / CHANGELOG / parity updated if this completes a tracked item
- [ ] Secrets and tokens not committed
- [ ] User-facing strings and UI are original (no copied trade dress)

## Local verification
<!-- How a human can try this on the primary OS -->

## Notes / risks
```

Reviewers reject PRs that fail the checklist.

### 8.4 Human Gate on Research

- Research PRs or Chat-produced `docs/research/*` require explicit human approval before dependent feature implementation.
- Approval criteria: original wording, URLs present, no substantial copyrighted paste, no instruction to clone a vendor's exact UI.

---

## 9. Quality Control — Finished Product Gates (guide §9)

### 9.1 Feature and behaviour gates

- Every in-scope capability Implemented and Verified (local first where possible)
- Main workflows end-to-end
- Persistence and integrity tested
- No fake functionality
- CI green; primary platform hands-on tested via CI artefact
- Additional platforms as scoped
- Crash reporting and update-check work as designed (full network submission; not stub)
- ROADMAP and CHANGELOG accurate
- No secrets; licences listed; README accurate and user-friendly for a non-technical average user — professional paid-app feel, not developer docs. README must be written in plain language, explain what the app does and why it matters, show how to install/run with minimal steps, and include screenshots of key features when the app is polished enough to warrant it. No code-centric jargon, no build instructions for end users.
- Section 2.5 legal checklist clean
- Clean-room rules respected

### 9.2 GitHub hygiene gate on every major push to main or version release

- README is accurate and matches current build
- Clear stale or broken CI checks
- Close or triage open PRs/issues that are obsolete
- Remove draft PRs and stale issue labels
- Ensure the repo description topics/badges are correct
- Confirm Releases page is clean
- No release is published with lingering errors, stale PRs, or an out-of-date README.

### 9.3 Branch protection checklist (`main`)

- **Require a pull request before merging.**
- **Require status checks to pass** — every name in `.ci-required.txt`, including the PR-author check from II.26.7 and the secret + personal-data scan (II.29.3).
- **Require linear history.**
- **Do not allow force-pushes.**
- **Do not allow deletions.**
- **`enforce_admins: true`** — no admin bypass.
- **Restrict who can push** to the Dev account where the plan allows; otherwise CI check enforces.
- **Signed commits recommended** (`require_signed_commits: true`) — accept if the CI signing path is reliable; skip otherwise and record the reason.
- **Require conversation resolution before merging.**

Record the exact settings in `docs/dev.md` with the date.

### 9.4 Release and in-app gates

- **Release notes and token metrics:** Every semver release includes detailed release notes (app usage / feature changes) plus token usage metrics: total tokens consumed since beginning development and tokens used in this release. Schema in §24. Release notes published to GitHub Releases and embedded in-app.
- **In-app version & release notes:** App displays version number accurately but subtly in a UI-appropriate non-intrusive corner. Selecting the version opens a dedicated release-notes pop-up showing the full release history, detailed app usage per release, and cumulative / per-release token usage metrics. The pop-up reads the same token file bundled at build time.
- **Auto-update check:** Every app checks for a newer release, notifies the user in a simple user-friendly way, and offers one-click auto-update from the latest GitHub Release. The flow is simple, consistent across all suite apps, and non-intrusive (§7.2).
- **Contributor credits system:** The app includes a credits screen (About ▸ Contributors) with consent-verified entries from `contributors/people.toml` (schema in §24). AI agents and models are credited separately under About ▸ Models, never as people. Pull requests must never claim agent or bot authorship.
- **Build attestations (optional):** The release pipeline may include an SLSA provenance and Sigstore attestation step (`gh attestation verify`). Optional but recommended for formal security requirements.

## 10. UI / UX & Visual Assets (guide §10)

- Clean, modern, information-dense but not cluttered; consistent tokens; strong keyboard support
- **Original** visual design — no commercial look-alike branding

### Logos, Icons, Illustrations

1. Describe marks in `docs/design/brand.md` (meaning, shapes, colours — original).
2. Prefer agent- or hand-authored **SVG** under `assets/`; record licence in `ATTRIBUTION.md`.
3. Marketing rasters only if licence-clear; prefer SVG in-app.
4. Never copy competitor icons or trademarks.
5. Generate platform icons from SVG in CI when needed.
6. **UI verification evidence:** before opening a PR, attach an offscreen snapshot render or control-channel screenshot (headless, no display needed) showing the UI change. Include before/after screenshots in the PR body. Colours, radii, and spacing must come from design tokens — never hard-coded values.

---

## 11. Source of Truth and Tracking Files (guide §11)

- `docs/project-brief.md`, `scope.md`, `inspiration.md`, `product-name.md`, `architecture.md`
- `docs/inspiration/<product>.md` — one file per inspiration product (public behaviour, URLs only)
- `docs/research/<capability>.md` — one file per capability (acceptance criteria, edge cases, URLs only)
- `docs/specs/<feature>.md` — one per feature, written by `/to-spec` (§5.6.3; e.g. `specs/takeoff-quantities.spec.md`)
- `docs/tickets/<id>.md` — one per atomic ticket, written by `/to-tickets` (§5.6.3)
- `docs/team-log/<ticket-id>.md` — one per multi-agent ticket: the inter-agent audit trail (§5.7.1)
- `docs/reference/index.md` — structured reference generated from `docs/` for team pre-loading (§5.7.2)
- `docs/parity.md`
- `docs/capability-inventory.md` — ID / Product / Capability / Observable behaviour / Acceptance criteria / Source URLs / Status
- `docs/process-feedback.md` (append-only agent friction log; see addendum N)
- `docs/current-model-map.md`
- `docs/design/brand.md`
- `docs/tokens.md` — token usage metrics per release (§24; supersedes the older `docs/tokens.json` ledger)
- `docs/dev.md` — branch-protection settings, Dev account login, release-workflow notes, allowlisted pre-approved commands (§5.7.2)
- `docs/update-signing.md` — current public key fingerprint, rotation history
- `docs/resource-catalog.md` — projects adopted from the §25 catalogs, with licence, version pin, and the decision recorded
- `ROADMAP.md`
- `IMPLEMENTATION_PLAN.md`, `REQUIREMENTS_TRACEABILITY.md`
- `DECISIONS.md`, `PROGRESS.md`, `CHANGELOG.md`, `ATTRIBUTION.md`
- `docs/qc/`, `docs/development.md`
- `.github/PULL_REQUEST_TEMPLATE.md`
- `docs/gaps.md` (ranked worklist maintained per release)
- `docs/devlog.md` — **local-only**, gitignored. One entry per session. `PROGRESS.md` is the durable record; `devlog.md` is for continuity when sessions end abruptly and never for source-of-truth claims.
- `contributors/people.toml` (schema in §24 — consent-verified human credits only; AI models are credited under About ▸ Models, never here)

Separate **Facts / Assumptions / Decisions**.

---

## 12. Suggested Repository Structure

```
{product-or-suite}/
├── .github/workflows/ci.yml
├── .github/workflows/release.yml
├── .github/PULL_REQUEST_TEMPLATE.md
├── .dsh/
│   ├── agents/
│   ├── command/
│   │   ├── grill-me.md
│   │   ├── to-spec.md
│   │   ├── to-tickets.md
│   │   ├── tdd.md
│   │   ├── implement.md
│   │   ├── implement-spec.md
│   │   └── code-review.md
│   ├── skill/
│   └── settings.local.json    # local only, gitignored
├── crates/
│   ├── crash-report/
│   └── update-check/
├── apps/{app-name}/
├── contributors/people.toml
├── docs/
│   ├── project-brief.md
│   ├── scope.md
│   ├── inspiration.md
│   ├── inspiration/
│   ├── research/
│   ├── specs/
│   ├── tickets/
│   ├── team-log/
│   ├── reference/
│   │   └── index.md
│   ├── capability-inventory.md
│   ├── tokens.md
│   ├── dev.md
│   ├── update-signing.md
│   ├── resource-catalog.md
│   ├── design/
│   └── ...
├── ROADMAP.md
├── CHANGELOG.md
├── IMPLEMENTATION_PLAN.md
├── REQUIREMENTS_TRACEABILITY.md
├── DECISIONS.md
├── PROGRESS.md
├── ATTRIBUTION.md
├── AGENTS.md
├── rust-toolchain.toml
├── deny.toml
├── .ci-required.txt
├── .gitleaks.toml
├── LICENSE-MIT
├── LICENSE-APACHE
└── README.md
```

**Parallel-agent build hygiene.** When multiple agents work in the same workspace:
- Each agent uses a dedicated `CARGO_TARGET_DIR=target/agent-<name>` to avoid compiled artifact conflicts.
- New commands and logic go into **new modules** — never edit an existing module's code to add your feature.
- File ownership is strict: an agent may only edit files it "owns" (marked by a `OWNERS` file or agreed convention). This rule is binding for §5.7 teams too.
- `Cargo.toml` writes are atomic: modify a copy, validate, then replace.
- Each agent's target directory is capped at ~10 GB; clean or archive before continuing.

## 13. Architectural Stress Test — Gaps Found & Mitigations

| Gap | Risk | Mitigation |
|-----|------|------------|
| Vague "first functional build" | Endless foundation without a shippable slice | **Hard v0.1.0 gate** (§6.2) |
| No PR discipline | Agents merge incomplete or dirty work | **PR template + checklist** (§8.3) |
| Release path learned too late | Broken tags/artefacts at the worst time | **CI + release.yml from day one** (§4) |
| Research → code without review | Copyright bleed, UI cloning | **Human gate on research** (§2.3, §8.4) |
| Roadmap optimism | False "ready for work %" | Update % only from verified evidence |
| PR/issue authorship drifts to personal account | Inconsistent audit trail | **Dev-account assertion in `ci.sh` + PR-author CI check** (II.26.7) |
| Suite without shared crates | Inconsistent crash/update behaviour | Shared crates required |
| Free model web search missing | Stalled research | Browse model/MCP → CI fetch → human ladder |
| Heavy local builds | PC freeze | **Non-negotiable Actions-only** (§4 item 5) |
| Meta-guide in git | Process secrets / confusion | **Never push this file** |
| Trademark-adjacent naming | Legal demand letters | Discovery forces original name; legal checklist |
| "Drop-in replacement" claims | Misrepresentation | Qualified claims only |
| Crash payload leaks | Secret / path / token exposure | **Mandatory scrub** (§7.1, II.29.2) |
| Signing key compromise | Malicious update to every user | **Rotation procedure** (§7.2) |
| Token metrics promised, not measured | Requirement no one can keep | **Schema in §24, file in `docs/tokens.md`** |
| Token-reduction tools used as a substitute for real verification | False confidence; "the proxy said it saved 90%" | **Measured-only rule** in the §19 token-efficiency subsection (rule 3); `docs/tokens.md` is the only source of token numbers |
| External catalogs treated as implementation sources | Copying a catalog's examples, or trusting a catalog entry blindly | **§25 use rules**: catalogs are discovery-only, every adopted entry is verified at its own source, licence and API terms checked |
| Long session degrades and silently skips rules | Rules forgotten, paths hallucinated | **§5.6.1 Smart Zone + `/new` between phases** |
| "TDD" claimed without a red run | Tests that pass for the wrong reason | **§5.6.4 red-run CI ID required** |
| Agent completes a task that does not build | False green state | **§5.6.4 green CI run ID required** |
| Command file contradicts the guide | Silent rule erosion | **§5.6.5 reconciliation table binds**; command file is fixed, not the guide |
| Team spawned for a single-agent task | Token waste, coordination overhead | **§5.7 only for Case 4** |
| Team swarm of 10+ agents | Token cost explosion | **§5.7.3 team size 2–5**; 6+ needs justification |
| Teammates bypass permissions and run builds locally | HARD BUILD RULE broken silently | **§5.7.5 reconciliation** — pre-approved build commands forbidden |
| Inter-agent loop unobservable to the human | Accountability gap (§5.5 item 1) | **§5.7.4 Tmux / task pane + on-disk log** (§5.7.1). |
| Teammate writes code without approved plan | Scope creep, unreviewed design | **§5.7.4 plan approval mode** |
| "Bypass permissions" flag weakens suite guarantees | Guards silently ignored | **§5.7.5** — refused; allowlist documented in `docs/dev.md`. |
| Reference material from another agent runtime treated as drop-in | Installing Claude Code/Cursor tooling in a DSH project | **Authority preamble** — DSH is the only supported runtime; other runtimes are reference only |

**Residual risks (accepted, monitored):** free-model capability variance; human required for research approval and release honesty; patent landscape cannot be fully automated; a manual personal-account PR is caught by CI, not prevented at the branch level on plans without "restrict who can push."

## 14. Start-to-End Execution Sequence

1. Discovery → human confirmation.
2. Name locked; public repo created (**never push this guide**).
3. Scaffold + CI + **release.yml** + **PR template** + `.dsh/command/*.md` (§5.6.3) + `.dsh/agents/*.yaml` (§5.7) + `AGENTS.md` (§5.6, §5.7, §5.5).
4. Tracking files + stub ROADMAP + `DECISIONS.md` entry for the Dev author identity.
5. Discover free models; assign roles.
6. Tasks until **v0.1.0** criteria met → tag Release.
7. Expand ROADMAP; human-gated research; shared crates.
8. Chat → Task → PR (as Dev) → local test → merge (as Dev) → semver Release loop.
9. Multi-platform expansion; polish; QC.
10. Section 9 gates + section 2.5 legal checklist → finished.
11. DSH free exhausted → NVIDIA free; process unchanged.

## 15. Session prompt

**Canonical session prompt:** the block below (§15, inline). No separate `addendum.md` file exists — the guide and any addendum are local-only, never committed to a public suite repo, and versioned in the human's private maintenance repository (§18). Until a human supplies an addendum, this in-guide prompt is operative.

If only this guide is available, use:

> Resume clean-room build via Chat → Task → PR. Read guide + addendum if present (never commit them). Read docs/*, ROADMAP, PROGRESS, process-feedback, capability-inventory, resource-catalog, specs/, tickets/, team-log/. Discover free models. If no v0.1.0, only §6.2 (Rust: testable core + headless tests + CI). Apply §5.6 workflow: pick case from matrix, `/grill-me` → `/to-spec` → `/new` → `/to-tickets` → `/new` → `/implement` per ticket. For Case 4 only, apply §5.7 multi-agent protocol: 2–5 teammates, one Orchestrator, shared task list, plan approval, graceful shutdown. Verify through `ci.sh` only — no raw bash. CI green before client try-steps. Research = summaries + URLs; human gate. No RE, no secrets, no heavy local builds. Author every GitHub action as the Dev identity (II.26.7). Log process friction to docs/process-feedback.md without waiving safety. End with: shipped, client try-steps, next, any process feedback logged.

## 16. Process feedback

Agents may append `docs/process-feedback.md` when the guide blocks productive work.
Template and non-negotiables: **addendum section N**. Human promotes changes into the local guide/addendum at milestones; agents do not self-edit these meta-documents by default.

## 17. Rust note

For Rust desktop apps, the binding requirements below (§17, inline) stand in for addendum sections K and M: `*-core` crate, headless workflow tests, `rust-toolchain.toml`, CI — not the client laptop — for heavy work, v0.1.0 structural checklist M.2. No separate `addendum.md` file exists (§18); until a human supplies one, this in-guide summary is binding.

- **Crate layout.** Split the app into `<app>-core` (headless, pure logic, testable with `cargo test` on CI without a display), `<app>-commands` (command definitions), `<app>-persistence` (storage), `<app>-ui` (rendering), and `<app>-app` (wiring). The core crate must not depend on the GUI toolkit.
- **Headless workflow tests.** Every primary workflow has a test that exercises the command handler end-to-end without launching a window.
- **`rust-toolchain.toml`.** Pin the toolchain version; CI uses it.
- **CI-only heavy work.** All `cargo` commands run in GitHub Actions (§4 item 5). The client machine is edit/read/git only.
- **v0.1.0 structural checklist.** `.github/workflows/ci.yml` present and green; `rust-toolchain.toml`; `<app>-core` split with headless tests; `CHANGELOG.md` entry; tag `v0.1.0` published.

## 18. Final Reminders

- `guide.md` and `addendum.md` stay **local forever** — never pushed to a public suite repo. They are versioned in a private maintenance repository owned by the human. When the guide changes, `AGENTS.md` in each suite repo is synced per §5.5.
- **Hierarchy:** addendum wins on client/Rust/session; guide wins on clean-room/legal.
- **All GitHub actions are authored by the Dev identity (II.26.7)** — PRs, issues, comments, releases, tags. Personal-account authorship of any of these is a process defect caught by the required CI check.
- **v0.1.0 hard gate** before roadmap theatre.
- PR template, early release workflow, human research gate are mandatory.
- Public research = summaries + URLs only; legal checklist every release.
- Process feedback via `docs/process-feedback.md` only; no silent rule deletion.
- **§5.6 DSH protocol is binding**: pick the workflow case from the matrix, run `/new` between phases, verify through `ci.sh`, and never claim TDD without a recorded red-run CI ID.
- **§5.7 multi-agent protocol is binding for Case 4 only**: 2–5 teammates, one Orchestrator, shared task list on disk, plan approval before code, graceful shutdown. No "bypass permissions," no pre-approved build commands.
- **DSH is the only supported runtime.** References to other agent runtimes (Claude Code, Cursor, Codex, Aider, OpenCode, etc.) in §19 and §22 are **reference material for adaptation**, never installation targets. Do not install Claude Code plugins, Cursor plugins, or Codex tooling on a machine that runs DSH.

---

## II.18 Naming, branding and message (extends Part I section 0)

- **Check for name conflicts** before committing to a name: search existing projects, package registries, and the [WIPO Global Brand Database](http://www.wipo.int/branddb/en/) for trademarks.
- **Reserve the handles** (domain, org/user name, package name) even if unused.
- **Choose clarity over cleverness** in the name.
- **Write a one-paragraph message**: what it does, who it is for, why it is different.
- **Your writing style is part of the brand.** Plain language and inclusive wording ("they" for a single person); many readers are not native English speakers.

## II.19 Legal depth (extends Part I section 2)

### II.19.1 Copyright and originality

Every line of code, every string, every icon, every layout and every algorithm must be original: derived from public standards, the project's own spec, and capability-level inspiration recorded in `docs/inspiration.md` / `docs/inspiration/`. The test is not "does it look similar?" but "could this have been written without access to the inspiration product's internals?" If the answer is no, it does not ship.

The designer maintains the capability inventory (`docs/capability-inventory.md`) with a source URL for every claimed behaviour. No capability may be implemented until its source URL is recorded and its wording is original.

### II.19.2 Trademarks and naming

The product name, window title, and visual identity must be original. Search existing projects, package registries, and the WIPO Global Brand Database for conflicts. Reserve handles. Choose clarity over cleverness. Record the decision in `DECISIONS.md` once; do not revisit without a new decision entry.

### II.19.3 Licence

Prefer dual licence **MIT OR Apache-2.0**. Before the first public release, `ATTRIBUTION.md` lists every third-party crate with its licence, and `cargo deny` (`deny.toml`) passes in CI. Report the licence accurately in any SBOM.

### II.19.4 Secrets and tokens

No API keys, tokens, or credentials in git or release binaries. Crash reports and update checks use user-supplied tokens only; no embedded privileged bot token ever ships in a client binary.

- Prefer the platform credential store / keychain for any persisted user token.
- Request only minimal scopes (`public_repo` or finer).
- Secret scanning (gitleaks) runs in CI on every PR and is a required status check.
- See also §7.2 "Token & authentication" for updater-specific rules.

### II.19.5 Release legal checklist

The checklist in §2.5 runs before **every** public release, not just v1.0.0. The release is not cut until every row is checked and recorded in `PROGRESS.md` with the CI run ID as evidence.

## II.20 Security-audit skill

### II.20.1 What is in scope

- The application on every supported platform.
- Parsing and loading of the app's own document and library files, CSV import and JSON backups (atomic write → rename, no path traversal).
- The optional update check (GitHub Releases API, soft-fail offline) and the crash-report submission path (user token, pre-filled issue URL fallback, II.29.2 scrub).
- The CI and release pipeline in `.github/workflows/` (pinned action SHAs, Dependabot for Cargo and Actions, gitleaks, CodeQL, cargo-deny).

Out of scope: third-party services the user connects to voluntarily, the operating system itself, and physical access to the user's machine.

### II.20.2 Severity scale

| Severity | Definition | Response |
|----------|-----------|----------|
| Critical | Remote code execution, secret exfiltration, or crash-report data leak without user consent | Fix before next release; private advisory |
| High | Local privilege escalation on the user's machine, or silent data corruption | Fix before next release; private advisory |
| Medium | Information disclosure (logs, paths, crash contents) or weak validation | Fix in next release cycle |
| Low | Hardening opportunity, no direct exploit path | Track in backlog |

### II.20.3 Reporting and disclosure

GitHub private vulnerability reporting. Acknowledge within seven days. Coordinated disclosure. Reporters credited unless they ask not to.

### II.20.4 Scanner gates

Every PR runs secret scanning, SAST, and dependency/licence check (`cargo-deny` for Rust). SBOM (CycloneDX) generated and attached to every release. Required status checks on `main` via `.ci-required.txt`; direct pushes disabled.

**Universal branch rule:** All changes land on `main` via a verified pull request merge, never by direct push: push branch → CI must pass → PR (authored by the Dev account, II.26.7) → merge after required checks (from the Dev account). The git identity is `"<App Name> Dev" <dev@example.com>` and the authenticated GitHub login is the Dev account. No new accounts.

## II.21 Accessibility

### II.21.1 Commitment

Accessibility is a correctness property of the UI. Target **WCAG 2.2 Level AA where feasible** for a native desktop application. Where a criterion cannot be met (immediate-mode toolkit, native renderer), record the limitation in `ACCESSIBILITY.md` rather than glossing over it.

### II.21.2 Primary priorities

1. **Keyboard support.** Every flow keyboard-completable. Focus order sensible and visible.
2. **Screen-reader semantics.** egui AccessKit on supported platforms. Meaningful roles, names, values.
3. **Colour contrast.** WCAG AA in dark and light; colour never the only signal.
4. **Reduced motion.** Platform preference honoured; see II.30.7.7 for the egui limitation.

### II.21.3 Severity scale

| Severity | Definition |
|----------|-----------|
| Blocker | Core flow cannot be completed from the keyboard, or a screen reader cannot announce a required control |
| Critical | Focus trapped, invisible, or lost; state conveyed by colour alone |
| Serious | Contrast below AA; dialog cannot be dismissed with Esc |
| Moderate | Reflow at 200% or narrow window clips or overlaps a control |
| Minor | Cosmetic contrast or focus-ring inconsistency |

### II.21.4 Testing method

Run the accessibility smoke test in `docs/qc/checklist.md` whenever UI is touched: keyboard-only walk-through, Esc dismissal, screen-reader spot check, 200% zoom / narrow window, high-contrast in both themes. Record in QC run log. Screen-reader behaviour is a manual spot check, not an automated CI suite — record that honestly in `ACCESSIBILITY.md`.

### II.21.5 Reporting path

Accessibility issues use `.github/ISSUE_TEMPLATE/accessibility.yml`, triaged with the same seriousness as other bugs.

## II.22 Marketing

### II.22.1 Message

Write one paragraph: what the app does, who it is for, why it is different. Becomes README opening, repo description, announcement. Plain language, inclusive wording.

### II.22.2 Handles and presence

Reserve handles even if unused. Writing style consistent across README, repo description, release notes, issue templates.

### II.22.3 What marketing is not

No false affiliation, no "drop-in replacement" without careful qualification, no false certification claims. Release notes honest about scope and limitations.

## II.23 Maintainer sustainability

### II.23.1 Release cadence

One person, a few hours a week. Seven-day response expectation published and honest. Scope small enough to sustain: one slice, one release.

### II.23.2 Burnout guardrails

- Scope creep reflected back: "That's a new milestone; approve to add to ROADMAP?"
- Definition of Done (II.24) applied per slice.
- Progressive releases. No big-bang launch.
- Maintainer does not write code the client could write; the client's role is goals, priorities, and "does this work / feel right?"

### II.23.3 Human gate

Research notes (`docs/research/*.md`) require explicit human approval before dependent implementation. The agent proposes; the human approves or rejects.

## II.24 Definition of Done

### II.24.1 Per slice (every Task)

- Acceptance criteria met with real behaviour.
- Unit tests cover happy path, boundary, and failure/edge cases; headless domain tests need no display.
- Formatter- and linter-clean in the project's own toolchain (CI runs `cargo fmt --all -- --check` and clippy with `-D warnings`; no toolchain command locally).
- CI green on GitHub Actions: format → lint → test → release-build on every supported platform.
- No fake success states, dead controls, or placeholder-only data.
- **No personal data and no secret in the change.** II.29 scan clean across working tree, history, commit metadata, **and the PR text**. Placeholder values obviously synthetic.
- `CHANGELOG.md`, `PROGRESS.md`, `REQUIREMENTS_TRACEABILITY.md` updated with exact CI results.
- PR opened using the template, **by the Dev account** (II.26.7).

### II.24.2 Per phase

- All phase requirement IDs traced to `DONE` with CI evidence.
- `REQUIREMENTS_TRACEABILITY.md` ↔ `IMPLEMENTATION_PLAN.md` consistent.
- Open assumptions reviewed in `docs/assumptions-and-questions.md` (answered or marked still-open with Q-IDs).
- `DECISIONS.md` has an entry for every user directive this phase.

### II.24.3 Release / packaging

- Cross-platform build artefacts verified on every supported platform: each CI run attaches binaries plus the lockfile.
- App launches on a clean machine; save → reopen → identical state.
- Version bumped; `CHANGELOG.md` has a release entry; `docs/tokens.md` has the release token entry.
- No secrets/keys committed; no local-only files in tree.
- `LICENSE` present; `ATTRIBUTION.md` complete and accurate.
- Release assets scanned for secrets and personal data (II.29.6); SBOM and installers carry no maintainer home path or personal identifier.
- `SHA256SUMS.txt` + minisign signature attached; fingerprint matches `docs/update-signing.md`.

## II.25 Launch readiness

Checked before a public launch or major release. Items already in §9 are not repeated.

- README answers: what this does, why it is useful, how to get started, where to get help.
- `SECURITY.md`, `CODE_OF_CONDUCT.md`, `CONTRIBUTING.md`, `GOVERNANCE.md`, `VISION.md`, `ACCESSIBILITY.md` present and current.
- Issue forms (`bug_report`, `feature_request`, `accessibility`) and PR template render correctly.
- Branch protection on `main` matches §9.3; direct pushes disabled.
- Seven-day response expectation published.
- Supported platforms and privacy claims match built binaries and code.
- No false affiliation or "drop-in replacement" claims.
- Scanner gates active and green: secret scanning, SAST, dependency/licence audit, SBOM attached.
- Dependabot enabled for Cargo and Actions.
- No open PR older than 14 days without a decision.

## II.26 Open questions for the human

### II.26.1 "Local tests must be runnable" vs "no heavy work locally" — resolved by documentation

- **Status (2026-11-17):** Adopted. `CONTRIBUTING.md` documents the exact commands to build and test locally for contributors. The `main`-branch rule set in §9.3 and the "no local `cargo`" rule in §4 item 5 apply to the maintainer's machine, not to contributors. The wording in `CONTRIBUTING.md` makes this distinction explicit: "CI is authoritative on this repo because the maintainer's machine is constrained. Contributors can and should run tests locally; see the commands below." Adoption across existing repos is an edit, not a decision — bring every repo to the same wording when its `CONTRIBUTING.md` is next touched.

### II.26.6 Licence choice: "MIT OR Apache-2.0" default

- Confirmed. The "OR" is intentional per project. `ATTRIBUTION.md` and any SBOM report it accurately.

### II.26.7 GitHub authorship — every action authored by the existing Dev identity

**Directive (owner, 2026-11-17).** Everything actioned in GitHub and every user-facing change in a suite repository — pull requests, issues, comments, releases, tags, merge commits — is authored by the **existing Dev identity**. No new GitHub account is created. No GitHub App is installed. No machine account is created. The Dev identity is:

- **Display name:** `<App Name> Dev`
- **Email:** `dev@example.com`
- **GitHub login:** recorded once in `DECISIONS.md` (the login of the Dev account already used for pushes and merges to `main`)
- **Existing credential:** the personal access token or SSH key already used for pushes and merges to `main`

**Why this satisfies the rule.** GitHub attributes PRs, issues, comments, releases and tag pushes to whichever account holds the credential that performs the action. Commits already carry `<App Name> Dev` metadata (via `ci.sh`). When the credential that pushes the branch and the credential that calls `gh pr create` / `gh pr merge` / `gh release create` both belong to the Dev account, every GitHub surface shows the Dev account. This is the mechanism.

**Required changes once in force:**

1. **`ci.sh` asserts three things before any commit, push, or PR:**
   - `git config user.name` == `"<App Name> Dev"`
   - `git config user.email` == `dev@example.com`
   - `gh api user --jq .login` == `<dev-account-login>` from `DECISIONS.md`
   Any mismatch → `ci.sh` refuses with a plain-language reason (§5.5 item 11).
2. **Required CI check:** `pull_request.user.login == '<dev-account-login>'`. Add to `.ci-required.txt`. This is the enforcement GitHub actually provides.
3. **Branch protection** per §9.3. The human's personal account keeps admin + review; the Dev account keeps push + merge. Merges happen from the Dev account (via `ci.sh merge` or by being authenticated as Dev in the browser).
4. **§5.5 item 14** (the authorship house rule) references this section.
5. **PR template** carries the "opened by the Dev account" checkbox.
6. **`DECISIONS.md`** records the Dev account login.

**Manual personal-account actions are a process defect.** If a PR is opened, an issue filed, or a release published from the personal account, the required CI check fails it (for PRs) and the process-feedback log records it. Corrective action: re-open from the Dev account, close the personal-account item, note it in the feedback log. No silent exceptions.

**Second case — a fork with a dead remote.** If a repository is a fork of an upstream the maintainer cannot push to, and the configured fork remote has disappeared, there is no writable remote and no authorship rule can be satisfied. Remediate by (a) restoring the fork, (b) pushing to a suite-owned remote, or (c) marking the repo read-only in `docs/gaps.md` and removing it from the active suite. Do not push to the upstream by mistake.

**Status (2026-11-17): in force.** No new accounts. The Dev account is the machine identity.

## II.27 Designer role and standing responsibilities

The designer is a standing role — not a final polish phase. Four responsibilities run continuously:

**(A) Clean-room analysis of the inspiration product.** Study what reference products do from public sources only, recording the URL for every claim: official docs, manuals, tutorials, blogs, talks, public screenshots/videos (for workflow understanding only — never pixel-copying), public specs, open standards. The method is black box. Forbidden: decompiling or RE; copying icons, fonts, exact layouts, string tables, presets or internal algorithms; using prior RE output; reproducing trademarked names, logos or trade dress; committing verbatim copyrighted text or third-party screenshots; pixel-perfect UI cloning. Write original behavioural summaries plus URLs under `docs/inspiration/` (index in `docs/inspiration.md`). New research requires owner approval before implementation depends on it.

**Discovery accelerators.** §25.3 (`open-design`) provides an Apache 2.0 design workspace with **skills, design templates, design systems, and plugins** that may be studied as *public behavioural references* for the designer role. It is not a source of UI code to copy; it is a public gallery of interaction patterns and design-system structures. Any pattern adopted must pass the same clean-room test as any other inspiration product. Record the decision in `docs/resource-catalog.md`.

**(B) The capability inventory.** `docs/capability-inventory.md` defines "all the features and abilities of the app we are designing." One row per capability: ID (`CAP-014`), Product, Capability, Observable behaviour, Acceptance criteria, Source URL(s), Status (not started / partial / implemented / out of scope). Enumerate until a pass over the public surface yields nothing new. Record implementation evidence (test name, CI run) in `docs/parity.md`. Move readiness percentages only from verified evidence. If a capability cannot be delivered within the clean-room rules, mark it `out of scope` with the reason and raise via the `question` tool.

**(C) Design vision and its implementation.** The designer owns the design and builds it: IA, interaction model, visual system, iconography, voice and tone. May edit source to realise design, working with `ui-dev` where one exists; domain logic stays out of UI. Every state is designed: empty, loading, error, disabled, permission-denied, long content, narrow window, high-contrast, reduced motion. No default controls, no placeholder text, no clipped text, no mismatched iconography, no dead controls, no cluttered toolbars. When a tool set grows beyond a handful, use a drop-down menu grouped by family. Outside the app: logo, wordmark, app icon set (all platforms), splash/installer art, illustration style, docs diagrams, README/social graphics, `docs/design/brand.md`.

**(D) Parity review.** Before any slice, milestone or release is called done, check the app against the inventory and report per capability ID: met / partial / missing / deliberately out of scope (with reason), each with evidence. The application is not complete while an in-scope capability is unmet. Judge the rendered product, not the intention. Where we exceed the inspiration product, record it — parity is a floor, not a ceiling. Report honestly.

## II.28 Naming conventions inside a repository

The shape is `<app>-<role>` / `<app>-<os>-<arch>`. In Rust source the hyphen becomes an underscore.

### II.28.1 What carries the app prefix

| Kind | Shape | Rule |
|------|-------|------|
| Workspace crates (package names) | `<app>-core`, `<app>-commands`, `<app>-persistence`, `<app>-ui`, `<app>-app` | `<app>-<role>` — unambiguous in `Cargo.lock`, a log line or a `cargo test -p` argument. |
| Crate identifiers in Rust | `<app>_core`, `<app>_app` | Hyphenated package name with `_`. |
| The binary | `<app>` | The user runs the app, not `<app>-app`. |
| Release artefacts | `<app>-linux-x86_64`, `<app>-linux-aarch64`, `<app>-windows-x86_64`, `<app>-macos-aarch64` | `<app>-<os>-<arch>`. |
| Checksum file | `SHA256SUMS.txt` | Fixed name. |

### II.28.2 What does **not** carry the prefix, and must not

| Kind | Correct | Why a prefix breaks it |
|------|---------|------------------------|
| Manifest | `Cargo.toml` | Cargo discovers crates by exact filename. |
| Crate roots | `src/main.rs`, `src/lib.rs` | Fixed entry points. |
| Module files inside a crate | `engine.rs`, `report.rs`, `update_check.rs` | Module path is the name. |
| Integration tests | `tests/properties.rs` | Discovered by path. |
| GitHub community files | `README.md`, `LICENSE`, `LICENSE-MIT`, `LICENSE-APACHE`, `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, `SECURITY.md`, `CHANGELOG.md`, `GOVERNANCE.md` | GitHub recognises by exact name. |
| Tooling configs | `.gitignore`, `.ci-required.txt`, `deny.toml`, `dist-workspace.toml`, `rust-toolchain.toml`, `.gitleaks.toml`, `.github/workflows/*.yml` | Read by name. |
| Already-scoped directories | `crates/core/`, `docs/architecture.md` | Directory already answers which app and which layer. |

**Rule: prefix what leaves the repository or must be globally unique; never prefix what a tool finds by name.**

### II.28.3 Deviations to check

- **Version-first artefact names.** An app may name artefacts `<app>-<version>-<target-triple>` with a versioned checksum file. Its update-check code parses those names, so changing them is a code change plus a release change, not a rename. Record the deviation in `DECISIONS.md`.
- **Architecture spelling.** Suite-wide default is `aarch64`. A repo that ships `arm64` records the deviation in `DECISIONS.md`.

## II.29 Personal data and secrets: the mandatory scan gate

A repository publishes three things at once: files, history, artefacts. A real person's name, email, phone, address, account name or secret is a defect, never content. This section makes the check mandatory for every agent, before every commit and before every release.

**Owner directive (2026-10-27): every agent runs a full scan, and nothing coded or released may contain this information.**

### II.29.1 What must never appear

- A real person's **name**, **email**, **phone**, **address**, or company/government identifier.
- A **personal value used as a default**: hard-coded demo/seed account, form placeholder, fallback identity, test fixture, sample/example data, populated commit/issue template.
- **The maintainer's OS account name or absolute home path** — `/home/<os-user>/...`, `C:\Users\<os-user>\...`. Use `~` or relative paths.
- **Secrets**: tokens, keys, passwords, `.env` contents, private URLs, webhook IDs, session cookies.
- Any **quoted copy** of the above in docs, audit notes, `CHANGELOG`, issue or PR text.

Obvious-synthetic values are fine: `dev@example.com`, `example.com`, `Jane Doe`, `+1 555 0100`, `ACME Demo JV`. Do not fix a leak by substituting another plausible real name.

### II.29.2 The scan is mandatory — four surfaces

1. **The whole working tree** — every file type, dotfiles and dot-directories (`.github/`, `.dsh/`, `.env*`), lockfiles.
2. **The whole history, not just `HEAD`.** `git log -S"<value>" --all` and `git grep` across `origin/main` and every branch.
3. **The published surface** — repo contents, issues, PRs, release bodies, and every **release asset** (tarball, AppImage, installer, SBOM). Build artefacts can embed absolute build paths.
4. **The metadata** — commit author/committer name/email, tag messages, repository description, homepage, topics.

**Fifth surface — the crash-report payload.** Before sending, the crash-report crate scrubs absolute paths, env-var values matching the patterns, and any `Authorization` header or token. See §7.1 for the mandatory scrub rules. A crash report is a publication and is subject to this section.

**Sixth surface — command frontmatter, team logs, and session transcripts.** `.dsh/command/*.md` frontmatter, `.dsh/agents/*.yaml`, `docs/specs/*`, `docs/tickets/*`, `docs/team-log/*`, and any chat/session log that is ever committed. The §5.6 and §5.7 protocols externalise state to disk; that state is scanned.

Pattern set:

```
(?i)/home/[a-z0-9_-]+
(?i)C:\\Users\\[A-Za-z0-9_.-]+
[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\\.[A-Za-z]{2,}
(?i)(api[_-]?key|secret|token|password|passwd|bearer)\\s*[:=]\\s*['"][^'"]+
```

A hit is a finding unless obviously synthetic or third-party upstream content. Classify every hit; never just count.

### II.29.3 Wire it into the gates

- Secret scan (gitleaks) and personal-data pattern scan on every PR, **required** status checks.
- Same scan in the **release job** against built artefacts, before assets attach.
- Both in `.ci-required.txt`.
- Record the scan result in `PROGRESS.md` with the CI run ID.

### II.29.4 When a value is already published

1. Remove from working tree and every current file (normal PR).
2. Redact quoted copies in docs — keep the finding, delete the value.
3. Purge history when the value is a personal identifier or secret: `git filter-repo --replace-text`, force-push, all clones re-fetch. Branch protection with `enforce_admins` blocks this; enable `allow_force_pushes` temporarily and restore. **Owner-approved action**, never a reflex.
4. Ask GitHub Support when history cannot be rewritten.
5. Rotate any secret regardless of history.

Record the incident, fix and residual risk in `CHANGELOG.md` or `SECURITY.md` — describing the defect, never repeating the value.

### II.29.5 Assignment of responsibility

- **Every agent** before committing/pushing: scan for its own change, stop if not clean.
- **Reviewer**: reject any change adding a plausible real identifier or secret, even in a comment, fixture, placeholder, or PR text.
- **Designer**: no real name, photo, address or personal datum in mockups, sample content or copy.
- **Orchestrator**: whole-history and release-artefact scan before any release (II.24.3); raise history-purging decisions with the owner.

### II.29.6 Release-asset scan

```
# tree + history
git grep -n -I -i -E '/home/[a-z0-9_-]+|@(gmail|outlook|hotmail|proton|yahoo)\\.'
git log --all -S"<personal value>" --oneline
# published artefacts
grep -rIn -i -E '/home/[a-z0-9_-]+' <extracted-asset-dir>
```

Only `/home/runner` (CI) and obviously synthetic values are acceptable hits.

## II.30 Rust GUI/UX — stack matching, tokens and evidence

### II.30.1 Hard rule — stack matching

| Source | Stack | Use for a native Rust desktop GUI? |
|--------|-------|-----------------------------------|
| Astryx | React 19 + StyleX | **No** as a dependency. Optional inspiration only. |
| shadcn/ui, Material web, any CSS design system | JS/CSS | **No** as code. Patterns only. |
| Open Design (nexu-io) | Web-first design workspace (Apache 2.0) | **No** as a dependency. **Copy pattern only** — its design systems and templates are public behavioural references, not egui code. |
| egui / eframe / iced / Slint / GPUI | Rust | **Yes** — primary sources. |

1. Detect the actual GUI crate and its resolved version before recommending.
2. Match every recommendation to that crate.
3. Never propose React, StyleX, Astryx, Tailwind, Open Design, or a CSS framework for a pure Rust GUI unless the repo already ships a web frontend.
4. Never answer "improve the design" with prose alone. A UX claim needs tokens, contrast, keyboard paths, or rendered evidence.

### II.30.2 Detect the stack before recommending

Read-only commands only (allowed on the client; `cargo` is not):

```bash
grep -nE '^(eframe|egui|egui_extras|egui-wgpu|iced|slint|gpui|tauri|dioxus|leptos)[[:space:]]*=' \
  Cargo.toml apps/*/Cargo.toml crates/*/Cargo.toml 2>/dev/null
grep -A2 -E '^name = "(egui|eframe|iced|slint|gpui)"' Cargo.lock
grep -rln 'egui::' --include='*.rs' . | head -20
```

Record in the PR or `docs/design/gui-stack.md`: toolkit, resolved version, shell shape, theme module path, whether an a11y/snapshot harness exists, window/shell layout model.

### II.30.3 Version discipline

- Pin recommendations to the `Cargo.lock` version, never the range in `Cargo.toml`.
- **A UX slice never bumps the toolkit.** Dependency moves are their own slice.
- Prefer crates documenting support for the project's egui version.
- Adopting a design crate behind the current toolkit is a recorded risk in `DECISIONS.md`.
- Confirm the API shape from current documentation before adopting.

### II.30.4 egui/eframe resource map

Tags: **adopt dependency** | **copy pattern** | **process only**.

**Design systems, theming and components:**

| Crate / project | Role | Guidance |
|-----------------|------|----------|
| shadcn-egui | shadcn-style presets on egui | Strong visual reference. Copy pattern by default; adopt only if brand and licence allow. |
| egui-pop (+ egui-pop-tokens) | DTCG tokens, visual kits, workbench, commands | Copy pattern for token structure and command/keymap architecture. Full `App` host is adopt only for greenfield. |
| spaceui | Semantic themes, SVG icons, custom shell | Copy pattern for shell, rail, status-bar structure. |
| modern-egui | Tokens plus UI helpers | Adopt when its egui version matches; otherwise copy token helpers. |
| egui_sauge | WCAG-AA palette, density, workbench chrome | Copy pattern. |
| egui_elegance | Opinionated buttons, inputs, tabs, cards | Adopt or copy for polish. Not a whole-app framework. |
| theme-egui | Semantic roles → `egui::Visuals` | Copy pattern. |
| egui_aesthetix | Trait-based themes | Alternative theming. Do not stack. |
| egui_material3 | Material 3 | Adopt only if Material is a product decision. |
| fluent_egui | Fluent / WinUI | Adopt only if Fluent is a product decision. |
| kylig | Animation / FX, reduced-motion aware | Adopt for motion only, with II.30.7.7. |

**Layout and docking:**

| Crate | Role | Guidance |
|-------|------|----------|
| egui_dock | Tab docking | Adopt only when user-rearrangeable panels are a goal. |
| egui_tiles | Flexible tiling | Adopt for genuinely tiled workspaces; plain panels for fixed three-pane. |

**Official and testing:**

| Resource | Role | Guidance |
|----------|------|----------|
| egui + egui_extras | Core widgets | First-party baseline. |
| egui accessibility docs | AccessKit | Process only. |
| egui_kittest | AccessKit + optional snapshots in CI | Adopt where the seam allows. |

**Hard limit.** One theming/design path per app. One primary typeface plus one monospaced face if needed, unless the brand owns a licensed family. Do not stack `shadcn-egui` + `spaceui` + `egui_aesthetix` in one binary. Open Design's design systems are **reference material** (see §25.3), not a second theming path.

### II.30.5 Other Rust GUI stacks

| Stack | UX sources | Rule |
|-------|-----------|------|
| iced | iced theme/widget ecosystem; Elm-style messages | Do not mix egui crates. |
| Slint | `.slint` components; Slint design tooling | Design in Slint's model. |
| GPUI | Retained-mode GPUI patterns | No egui design crates. |
| Tauri / Dioxus / Leptos (web UI) | Web design systems | Rust side is backend; web layer keeps token discipline. |

### II.30.6 One token source — semantic roles, not hex literals

**Placement.** `apps/<app>/src/theme/` or a `crates/<app>-theme` crate with no egui dependency for palette maths.

**Semantic roles (minimum set):**

| Role | Paints | Requirement |
|------|--------|-------------|
| `surface.base` / `.raised` / `.sunken` | panels, popups, wells | Distinguishable without borders. |
| `text.primary` / `.secondary` / `.muted` / `.disabled` | body, labels, hints, disabled | AA against the surface each sits on. |
| `accent` / `accent.hover` / `accent.on` | primary action, selection | `on` meets AA on `accent`. |
| `danger` / `warning` / `success` (+ `.on`) | validation, status | Never the only state signal. |
| `border.subtle` / `.strong` | separators, input outlines | `strong` ≥ 3:1 against adjacent surface. |
| `focus.ring` | keyboard focus | ≥ 3:1 against control and surface. |
| `overlay.scrim` | modal dimming | Dialog text readable at composite. |
| `selection.bg` / `selection.text` | text selection, list highlight | Contrast as text-on-fill. |

**Scales.** Spacing 2-4-8-12-16-24-32-48; radii 2/4/8/12; elevation three levels maximum; type scale one ratio (1.125 or 1.2) with roles `display`, `h1`, `h2`, `body`, `small`, `mono`.

**Mapping.** `Tokens` exposes `fn visuals(&self) -> egui::Visuals` and `fn style(&self) -> egui::Style`.

**Variants.** Light, dark, high contrast. System-follow default; user choice persisted. `Visuals::high_contrast()` may seed the third variant but must pass the II.30.7.1 tests.

**Enforcement (process only, cheap).** CI grep gate:

```bash
! grep -rn --include='*.rs' -E 'Color32::from_(rgb|rgba|gray|black|white)' \
    apps/*/src crates/*/src | grep -v 'src/theme/'
```

### II.30.7 Accessibility and input as code

1. **Contrast is a test, not an opinion.**

```
contrast(a, b) = (L1 + 0.05) / (L2 + 0.05), with L = 0.2126R + 0.7152G + 0.0722B
after sRGB linearisation
```

Required: **4.5:1** body text, **3:1** large text (≥ 24 px regular or ≥ 18.66 px bold), **3:1** meaningful UI boundaries and focus rings, **3:1** for every state colour against its surface. Assert both themes and high contrast in one `#[test]`.

2. **Colour is never the only signal.**
3. **Keyboard completes every flow.** Tab order reading order; Esc dismisses topmost dialog; Enter activates default; focus never trapped; `focus.ring` visible on every interactive widget. Shortcut table in `docs/qc/shortcuts.md`.
4. **Icon-only controls are named.** Tooltips are not names; set `WidgetInfo`/`labelled_by`.
5. **Target size.** At least **24 logical points** on the shortest side for interactive targets (WCAG 2.2 AA 2.5.8). Primary and destructive actions at least 32. Density variants never below 24. *(The 44–48 pt figure is iOS HIG and applies only to a mobile/web surface, not to this native desktop rule.)*
6. **Reflow.** At narrow window and 200% zoom no control clips or overlaps; the shell degrades by collapsing the side panel before shrinking the canvas.
7. **Reduced motion — honest limitation.** egui has no cross-platform system-level reduced-motion query. Provide an in-app preference; default animations **off** when the platform signal is unavailable; keep transitions under ~120 ms and non-blocking; record the limitation in `ACCESSIBILITY.md`.

### II.30.8 Evidence — rendered, not intended

**Minimum evidence per UX slice.** Shell chrome in light and dark, one primary panel in its empty state, one error state, one dialog, and the narrow-window layout — captured from the built app and reviewed by the designer. Artefacts in the PR description or review record, never committed.

**`egui_kittest` gates where a harness exists.** AccessKit assertions are real tests: named control with the right role, dialog dismisses on Esc, focus moves, empty state renders its CTA. Snapshot PNGs are for the eye; the assertions are the gate.

**No pixel baselines.** A diff that fails on font-metric change teaches the team to ignore the gate.

### II.30.9 Workflows the agent runs

| Workflow | Outcome | Gate |
|----------|---------|------|
| Token → toolkit visuals | One token table drives light, dark, high contrast | Contrast test green |
| Component recipe sheet | Primary/secondary/ghost/danger plus sizes documented | Review: no ad-hoc buttons |
| Empty/error/loading states | Every primary panel has a real, non-dead state | Rendered evidence |
| Keyboard and focus pass | Shortcuts listed, focus visible, icon controls named | AccessKit test or manual QC log |
| Contrast CI | Unit tests for text and UI contrast | Merge gate |
| Snapshot/kittest | Shell and critical dialogs baselined | Artefact reviewed |
| Dependency slice | Toolkit and design-helper bumps isolated | CI green, evidence re-reviewed |

### II.30.10 Component recipe sheet

| Role | Use | Rules |
|------|-----|-------|
| Primary | The one action a view exists for | At most one per view; accent fill; `accent.on` text. |
| Secondary | Supporting actions | Outline or subtle fill. |
| Ghost | Toolbar, inline actions | No container until hover; ≥ 24 pt target. |
| Danger | Destructive, irreversible | Separated from primary cluster; confirmation names the consequence. |

States every recipe specifies: default, hover, active, focus, disabled, loading. Loading keeps the control's size stable. Disabled explains why on hover/focus.

### II.30.11 States every primary panel owns

| State | Requirement | Copy rule |
|-------|-------------|-----------|
| Empty | Says what the panel is for and offers the first action | What, why empty, what next — never "No data". |
| Loading | Skeleton or spinner; no layout shift | No fake content; no spinner under 400 ms perceived work. |
| Error | Names what failed, keeps input, offers retry | What happened, what to do, no blame, no error code alone. |
| Disabled | Explains the precondition | Reason plus the enabling step. |
| Permission denied | Says who can grant access | No dead end. |
| Long content | Scrolls or truncates with a way to see all | Truncation never hides the only copy. |
| Narrow window | Collapses predictably | Canvas shrinks last. |
| High contrast / reduced motion | Renders and animates per II.30.6 and II.30.7.7 | No state conveyed by motion alone. |

### II.30.12 Decision tree

```text
Is the UI web (Tauri / Dioxus / Leptos frontend)?
  YES -> Web design systems apply to the frontend.
  NO  -> Native Rust GUI path.
         Is the toolkit egui?
           YES -> Token discipline + modern-egui / shadcn-egui / sauge / elegance
                  patterns; egui_dock or egui_tiles only if docking is a goal;
                  egui_kittest + AccessKit for evidence.
           NO  -> Use that toolkit's official theming and widget ecosystem only.
         Is the task "polish the existing app"?
           YES -> Selective widgets + tokens + a11y + split oversized UI modules.
           NO  -> Greenfield: a fuller kit may be evaluated.
```

### II.30.13 Deliverables for a GUI/UX request

1. Stack detection — crate, resolved version, shell shape.
2. Gap list — tokens, contrast, keyboard, empty states, density, consistency, module size.
3. Recommendations, each tagged adopt / copy pattern / process-CI.
4. Non-goals.
5. Implementation slices — one PR each, test and a11y note per slice.
6. Evidence plan.

### II.30.14 Non-goals

- Astryx, React, StyleX, Open Design, or any CSS design system inside a native Rust GUI.
- Two competing egui design systems in one binary.
- Rebuilding the shell around a framework without an explicit migration task.
- Copying the inspiration product's pixels, icons, fonts, string tables, presets.
- An emoji set standing in for an icon system.
- A toolkit or design-crate version bump hidden inside a UX slice.
- "Design improved" without contrast, keyboard, or rendered evidence.
- Any heavy build, test, or `cargo` command on the client machine.

### II.30.15 Definition of done for a UX slice

- [ ] Stack and resolved version recorded; no crate outside its supported egui version.
- [ ] Every colour, space, radius and text size from the token module; raw-colour grep gate passes.
- [ ] Contrast tests assert light, dark, high-contrast palettes (4.5:1 text, 3:1 large and UI boundaries).
- [ ] Every primary panel has empty, loading, error states with real copy.
- [ ] Keyboard completes the flow; focus visible; icon-only controls carry AccessKit labels; shortcuts documented.
- [ ] Targets ≥ 24 pt; narrow window and 200% zoom do not clip a control.
- [ ] Rendered evidence captured and reviewed (II.30.8); AccessKit assertions where a harness exists.
- [ ] Adopted crates' licences in `ATTRIBUTION.md` and allowed by `cargo deny`.
- [ ] `CHANGELOG.md`, `PROGRESS.md` and PR design section updated with the evidence, not just the claim.
- [ ] Verification is from the CI artefact, not a local build (§6.2).

### II.30.16 Licence and clean-room hygiene for GUI crates

Every adopted crate passes II.20.4 and M.4: SPDX checked, licence compatible, recorded in `ATTRIBUTION.md`, allowed by `cargo deny`. Design galleries and demo apps are study material: copy the pattern, reimplement the look, never lift icons, fonts, copy or layouts. A bundled font needs its licence in `ATTRIBUTION.md`; the default egui font set suffices unless the brand owns a licensed face.

### II.30.17 Quick card

1. Detect the toolkit and its `Cargo.lock` version before recommending.
2. Match the stack: no React/CSS design system in a native Rust GUI.
3. One design path per app; no multi-system mashup; one typeface unless the brand owns one.
4. Tokens first: semantic roles, one theme module, no scattered hex.
5. Contrast is a unit test, not an opinion.
6. Every panel has empty, loading, error states with real copy.
7. Keyboard completes the flow; icon-only controls are named.
8. Evidence is rendered or asserted, never asserted from prose.
9. Toolkit bumps are their own slice.
10. No `cargo` locally — verify with `ci.sh` and CI artefacts.

---

## 19. DSH Agent Skills

**Snapshot (~early October 2026) — will be stale by design. Discover current skill patterns via the DSH skill ecosystem and the §25 catalogs before relying on this table.**

**What this section is, and is not.**

- **It is** a reference catalog of *skill patterns* curated from the broader agent ecosystem, most of which were originally indexed for other runtimes (Claude Code in particular).
- **It is not** a list of drop-in DSH packages. None of these skills is installed by copying a command from the column below.

**How to use a skill from this table with DSH:**

1. Read the skill's source repository and its `SKILL.md` / instruction file.
2. Adapt the instruction set into `~/.config/dsh/skill/<name>/` following DSH's skill layout (§5.6 deployment, §12 repo structure).
3. Record the licence in `ATTRIBUTION.md` and the capability + version in `DECISIONS.md` (§19 "Skill integration rules" below).
4. Verify the adapted skill does not bypass §5.6, §5.7, or the reconciliation tables in §5.6.5 and §5.7.5. If it does, the guide wins and the skill is rewritten or dropped.
5. Never run a runtime-specific install command (Claude Code's `npx skills add … -a claude-code`, the Claude Code plugin marketplace, Cursor's plugin CLI) on a machine that runs DSH. The commands below are shown for provenance, not for execution.

### Top 10 Reference Skills (ranked by stars in the upstream catalog)

| Rank | Skill | Approx. stars | Description | Provenance (not install) |
|------|-------|---------------|-------------|--------------------------|
| 1 | **obra/superpowers** | ~295k | Plans, writes tests, and debugs before writing code. The most-starred skills framework. | Upstream install targets Claude Code; adapt per §19 preamble. |
| 2 | **mattpocock/skills** | ~275k | Small, composable engineering skills straight from Matt Pocock's own setup. | Upstream install targets Claude Code; adapt per §19 preamble. |
| 3 | **multica-ai/andrej-karpathy-skills** | ~217k | One CLAUDE.md derived from Karpathy's notes that improves how an agent writes code. | Rename/rewrite for `AGENTS.md`; adapt per §19 preamble. |
| 4 | **anthropics/skills** | ~180k | Anthropic's official set (docs, PDFs, spreadsheets, slides, design, skill-creator, etc.). | Upstream install targets Claude Code; adapt per §19 preamble. |
| 5 | **DietrichGebert/ponytail** | ~154k | Makes the agent code like the "laziest senior dev": less code, fewer bugs. | Upstream install targets Claude Code; adapt per §19 preamble. |
| 6 | **nextlevelbuilder/ui-ux-pro-max-skill** | ~133k | Design intelligence (styles, palettes, UX rules) so UI stops looking AI-generated. | Upstream install targets Claude Code; adapt per §19 preamble. |
| 7 | **Graphify-Labs/graphify** | ~124k | Turns a codebase + docs + schemas into a queryable knowledge graph. | Upstream install targets Claude Code; adapt per §19 preamble. |
| 8 | **JuliusBrussee/caveman** | ~110k | Terse "caveman" response style; ~65% token cut; 30+ agents. See **Token-efficiency and context-engineering skills** below. | Upstream install targets Claude Code; adapt per §19 preamble. |
| 9 | **addyosmani/agent-skills** | ~101k | Production engineering workflows from Addy Osmani (spec → build → test → ship). | Upstream install targets Claude Code; adapt per §19 preamble. |
| 10 | **Leonxlnx/taste-skill** | ~116k | Gives coding agents real design taste / art direction + reference images + browser check so UI doesn't look generic. | Upstream install targets Claude Code plugin marketplace; adapt per §19 preamble. |

### Remaining Skills from the Full 320-List

The upstream page (a snapshot ~October 3, 2026) lists all 320 skill repos with categories and search. Star counts grow; treat this table as **stale by design**. Use `npx skills explore` only as a **catalog discovery tool** to see what exists — never as an install command for DSH. For DSH usage, follow the §19 preamble (adapt into `~/.config/dsh/skill/<name>/`).

### Token-efficiency and context-engineering skills

These tools reduce the *volume* of tokens an agent consumes. They complement §5.2 (free-model policy) and §6.4 (token-metrics release notes). Each is local-first: no code leaves the user's machine except to the model provider already in use.

| Tool | Approx. stars | What it does | Install (DSH-compatible) | Licence |
|------|---------------|--------------|--------------------------|---------|
| **rtk-ai/rtk** | ~68.5k | CLI proxy; 60–90% token reduction on 100+ dev commands; single Rust binary, <10 ms overhead. | `brew install rtk-ai/tap/rtk` or `cargo install --git https://github.com/rtk-ai/rtk` (build output, not a toolchain run on the client — §4 item 5 still applies to *this project's* build). | Apache-2.0 |
| **elara-labs/code-context-engine** | ~450+ | Local MCP server; indexes the codebase so agents search instead of re-reading files; 94% input-token savings; works with any agent that speaks MCP. | `uvx --from "code-context-engine[local]" cce init` | MIT (core); one optional component is dual AGPL-3.0/commercial — record the distinction in `ATTRIBUTION.md` |
| **headroomlabs-ai/headroom** | ~69.8k | Context compression layer; library, proxy, MCP server, or `headroom wrap <agent>`. Compresses tool outputs, logs, RAG chunks, files, history; 60–95% fewer tokens for JSON, 15–20% for coding agents; reversible (CCR). | `headroom wrap dsh` — confirm the exact DSH CLI name in the current headroom docs; if headroom has no DSH wrapper yet, use its proxy/MCP mode instead. | Apache-2.0 |
| **tirth8205/code-review-graph** | ~3.7k lines Python, active | Local-first code-intelligence graph for MCP/CLI; persistent codebase map; risk-scored PR reviews; 6.8× fewer tokens on reviews, up to 49× on daily tasks. | `code-review-graph install` (MCP server; wire it into DSH's MCP config) | MIT |
| **JuliusBrussee/caveman** (also #8 in the Top-10) | ~116k | Terse "caveman" response style; ~65% token cut; 30+ agents. | Adapt into `~/.config/dsh/skill/caveman/` per §19 preamble. | MIT |

**Mandatory rules for all five** (extends the general skill rules below):

1. **No local heavy builds.** None of these may be used to justify running `cargo` or any resource-intensive toolchain on the client machine (§4 item 5). Their own installers and proxies are fine; building them from source locally is not.
2. **Record licence and capability in `DECISIONS.md` and `ATTRIBUTION.md`.** For code-context-engine, record the MIT / dual-AGPL split explicitly (§II.20.4 and M.4).
3. **Token-savings claims must be measured, not assumed.** Any number that appears in `docs/tokens.md` or a release note is the measured result from the project's own runs, never the vendor's marketing figure (II.29.4 honesty rule, §6.4).
4. **No privileged token.** These tools may use the user's own provider credential; they must never embed or ship a privileged bot token (§7.2, II.19.4).
5. **Clean-room unaffected.** These are development-time efficiency tools. They do not relax the clean-room rules (§2.1–2.3) — code reviewed or generated with their help is still subject to the same clean-room test.
6. **One context tool at a time, by default.** rtk (command-output filtering), code-context-engine (codebase indexing), and headroom (payload compression) can be combined deliberately, but each must be recorded in `DECISIONS.md` with the reason. Do not stack two tools that do the same job (e.g. two codebase indexers) in one project.
7. **code-review-graph's `fail-on-risk` gate is optional.** If enabled, it runs in CI, not locally, and its threshold is recorded in `DECISIONS.md`. A failed gate is a review signal, not an automatic revert.
8. **No runtime-specific install.** No `-a claude-code` flag, no Claude Code plugin marketplace command, no Cursor plugin command. If a tool documents only a Claude Code or Cursor install path, adapt it to DSH (MCP config, skill directory) or skip the tool. Never install another runtime's tooling on a DSH machine.

### Agent-skill libraries (curated catalogs, §25)

Two of the §25 catalogs are **directories of installable agent skills** in other ecosystems. They are discovery surfaces, not endorsement:

- **shubhamsaboo/awesome-llm-apps** (§25.4) — 100+ agents, RAG apps, and installable agent skills. Each skill ships real code and passes a security + eval CI gate in its own repo. Use it to discover patterns (Project Graveyard, First Reader, Scope Creep Detector, Commit Archaeologist, Dependency Doctor, Advisor Orchestrator Worker), then apply §19's preamble (adapt into `~/.config/dsh/skill/<name>/`, record licence in `ATTRIBUTION.md`, capability + version in `DECISIONS.md`). The upstream `npx skills add <repo>/tree/main/<path>` command is Claude Code-specific and is **not** a DSH install command.
- **punkpeye/awesome-mcp-servers** (§25.2) — 4,000+ MCP servers. Not a skill library, but the MCP-server layer that many DSH skills depend on. Use it to find servers to wire into DSH's MCP configuration; vet every server against §II.20.4 before adoption.

### Skill integration rules

- **Skill selection** follows the guide's role routing; the Orchestrator assigns skills per task, never heavy local builds.
- **Skills are discovered via free model browsing** (DSH → NVIDIA fallback) and MCP/Catalog lookups. The §25 catalogs are the canonical discovery surfaces for skills, MCP servers, and design references.
- **Installed skills** live in `~/.config/dsh/skill/<name>/`, must have their licence recorded in `ATTRIBUTION.md`, and their capability + version in `DECISIONS.md`.
- **No skill** that requires `cargo` or resource-intensive builds on the client machine (§4 item 5).
- **No skill** that conflicts with clean-room rules (§2.2).
- **No skill** that instructs the agent to bypass §5.6, §5.7, or the reconciliation tables in §5.6.5 and §5.7.5.
- **No skill** that ships an install command for another runtime (Claude Code `-a claude-code`, Claude Code plugin marketplace, Cursor plugin, Codex install). If a skill has no DSH install path, adapt it or skip it — do not install another runtime's tooling on a DSH machine.
- **No agent fixes an issue by unplanned new code.** Every fix must have a plan, integrate with the existing code, pass CI, and leave no dead, unfinished, or placeholder code in the codebase. New code is expected; **unplanned** new code is forbidden. Fallbacks must be fully implemented and documented. All contributions pass CI, satisfy the dependency hygiene gates, and are verified end-to-end before merge.

---

## 20. Design Anti-Patterns — "Vibecoded" Signals & Intentional Alternatives

**Scope: this section applies to *product-facing copy* — UI strings, README, marketing text, release notes, and product surfaces — not to internal process documents.** Em dashes are fine in this guide; they are not fine in product copy.

**Applicability:** some items are web-specific. "Pure white background" (item 3) and "Inter / Geist / Space Grotesk as default" (item 10) are web concerns. A native desktop app honours the OS light/dark theme and uses the system font stack unless the brand owns a licensed face.

The 30 items:

1. **Harsh gradients** — prefer flat or very restrained transitions.
2. **Lucide icons** — pick a distinctive set or custom icons.
3. **Pure white background** (web) — off-white, cream, cool gray, or dark.
4. **Rainbow coloring** — strict 1–2 accent colours.
5. **Drop shadows on every card** — subtle, hard, or none.
6. **3 feature cards in a row** — asymmetric, 2/4-column, zig-zag, editorial.
7. **Emojis as UI icons** — real icons or custom illustrations.
8. **Liquid glass** — solid surfaces with clear hierarchy.
9. **Em dashes in copy** — commas, periods, shorter sentences.
10. **Inter / Geist / Space Grotesk** (web) — pair a display face with a clean body, or use one high-quality face with personality. Native: honour the system stack unless the brand owns a licensed face.
11. **Colored left stripe** — background, borders, or spacing.
12. **Fake testimonials** — real testimonials only, or none.
13. **Bento grids** — only if genuinely serving content.
14. **Decorative terminal window** — real product UI or a clean demo.
15. **"It's not X, it's Y"** — be direct.
16. **Checkmark bullets** — custom icons, numbers, strong typography.
17. **3 pricing tiers** — 2 tiers or single plan.
18. **No real product demos** — screenshots, videos, walkthroughs.
19. **Uniform soft corner radius** — mix intentionally.
20. **Purple and black** — a unique primary colour.
21. **No skeleton loaders** — always include.
22. **Radial orbs** — noise, grain, shapes, or clean backgrounds.
23. **Dot grids** — minimal backgrounds, texture sparingly.
24. **Sparkle icons** — meaningful icons only.
25. **Animated arrows** — purposeful micro-interactions.
26. **No ToS** — always include.
27. **No privacy policy** — always include.
28. **Hover animations on everything** — selective, purposeful.
29. **Neon colors** — muted or mid-saturation.
30. **Basic pastel sets** — intentional, desaturated, or higher-contrast.

> **Treat this list as a rejection filter. When in doubt, choose the less common, more intentional option.**

---

## 21. High-Quality UI Patterns — 20 Practical Tips

**Applicability:** prefer native-applicable patterns first. Web-only patterns apply when the project has a web frontend.

| Pattern group | Native (egui) fit | Web fit |
|---------------|-------------------|---------|
| Skeleton loaders, Optimistic updates, Command menu, Shortcut hints, Undo toasts, Interruptible animations, Hit slop, Tabular numbers, Continuous corners, Idempotency keys | Excellent | Excellent |
| Stale-while-revalidate, Safe area insets, Balanced headlines | Good | Excellent |
| Pre-rendering, BlurHash, View transitions, Swipe-to-dismiss, Predictive back, One-time code autofill, Amazon safe triangle | Limited / N/A | Excellent |

1. **Skeleton loaders** — match real content's shape; subtle shimmer.
2. **Optimistic updates** — update UI, reconcile, clear rollback path.
3. **Command menu** — searchable, fuzzy, shortcuts shown.
4. **Shortcut hints** — tooltips, menus, palette.
5. **Undo toasts** — 5–8 s window.
6. **Stale-while-revalidate** — show cached data, update silently.
7. **Pre-rendering** (web).
8. **BlurHash placeholders** (web).
9. **View transitions** (web).
10. **Swipe-to-dismiss sheets** (mobile).
11. **Interruptible animations** — cancel on user interaction.
12. **Predictive back** (mobile).
13. **Hit slop** — native: **24 logical points minimum** (§II.30.7.5, WCAG 2.2 AA 2.5.8); touch surfaces: 44–48 pt (iOS HIG).
14. **Safe area insets** (mobile/web).
15. **One-time code autofill** (web/mobile).
16. **Tabular numbers** — for data columns.
17. **Balanced headlines** (web).
18. **Continuous corners** — where appropriate.
19. **Idempotency keys** — every critical mutation.
20. **Amazon safe triangle** (pointer/web).

**Agent guidance:** prefer stack-fitting patterns. After implementing UI, run the Section 20 anti-pattern filter as a rejection check.

---

## 22. pstack — Rigorous Engineering Skills for Agents

**https://github.com/cursor/plugins/tree/main/pstack** (by Lauren Tan / @poteto, Cursor).

Core idea: *If you want to go fast, go deep first. Write less code of higher quality.* Turns the agent into something closer to a real engineering team: understand the existing system, choose a design, verify the result on the real app, review the diff, and only then report success.

- **`/poteto-mode`** — main entry point.
- 20+ playbooks: investigation, bug fix, performance, feature, refactoring, prototype, visual parity, shipping, overnight runs.
- Verification, unslop, multi-model review.

**Installation.** pstack is distributed as a Cursor plugin and a Claude Code plugin. It does **not** have a native DSH package. Do not install Cursor or Claude Code plugin tooling on a machine that runs DSH.

**How to use pstack with DSH:**
1. **Read as reference.** Open the relevant playbook in the pstack repository and adapt its workflow into a DSH command (`.dsh/command/<name>.md`) or skill (`~/.config/dsh/skill/<name>/`) following §5.6 deployment.
2. **Use the concepts, not the plugin.** The playbooks' posture (understand the existing system, choose a design, verify on the real app, review the diff, then report) is complementary to this guide's clean-room, never-crash, and CI-only rules. Read them as one would read a coding-standards document.
3. **Record the decision.** Add an entry to `DECISIONS.md` stating which playbooks were adapted, and record the pstack licence (Apache-2.0) in `ATTRIBUTION.md`.
4. **No local `cargo`.** Any adapted playbook that requires a local `cargo` invocation is prohibited by §4 item 5. The `ci.sh` verification path (§5.6.4) replaces the playbook's local-verification steps.
5. **Stricter rule wins.** When an adapted pstack playbook and this guide conflict, the stricter rule wins — especially clean-room, secrets, and legal boundaries.

---

## 23. Config file examples (appendices)

These are starting points, not law. Copy into the repo, adapt, commit.

**`.ci-required.txt`:**

```
ci
pr-author-is-dev
secret-scan
personal-data-scan
cargo-deny
```

`ci` is the umbrella check; the rest are the individual gates. Checks not listed are still reported but do not block a merge (per §4).

**`rust-toolchain.toml`:**

```toml
[toolchain]
channel = "1.83.0"
components = ["rustfmt", "clippy"]
targets = ["x86_64-unknown-linux-gnu", "x86_64-pc-windows-msvc", "aarch64-apple-darwin"]
profile = "minimal"
```

**`deny.toml` (abridged):**

```toml
[graph]
targets = [
  { triple = "x86_64-unknown-linux-gnu" },
  { triple = "x86_64-pc-windows-msvc" },
  { triple = "aarch64-apple-darwin" },
]

[licenses]
allow = ["MIT", "Apache-2.0", "BSD-3-Clause", "ISC", "Unicode-DFS-2016", "MPL-2.0"]

[bans]
multiple-versions = "warn"
deny = []

[advisories]
db-urls = ["https://github.com/rustsec/advisory-db"]
```

**`.gitleaks.toml` (minimal):**

```toml
title = "gitleaks config"
[extend]
useDefault = true
[allowlist]
paths = [
  "docs/research/.*",     # summaries + URLs only; still scanned by personal-data scan
  "Cargo.lock",
]
```

**`.github/dependabot.yml`:**

```yaml
version: 2
updates:
  - package-ecosystem: "cargo"
    directory: "/"
    schedule: { interval: "weekly" }
  - package-ecosystem: "github-actions"
    directory: "/"
    schedule: { interval: "weekly" }
```

**`gh api user` — expected outputs (recorded in `DECISIONS.md`):**

- `login` — the Dev account handle
- `name` — `<App Name> Dev`
- `email` — `dev@example.com`

---

## 24. Token-metrics and contributor schemas

### 24.1 `docs/tokens.md` — token usage

One line per release. Machine-readable by the release workflow and the in-app release-notes popup.

```markdown
| Version | Date       | Tokens this release | Tokens cumulative | Notes                     |
|---------|------------|---------------------|-------------------|---------------------------|
| v0.1.0  | 2026-11-20 | 1_204_338           | 1_204_338         | First functional build    |
| v0.1.1  | 2026-11-27 | 402_115             | 1_606_453         | Bugfix + a11y polish      |
```

Source: the model provider's usage export, aggregated by release tag boundary. The release workflow reads the last row and injects it into the GitHub Release body. The in-app popup reads a `tokens.json` bundled at build time, generated from the same file by a build script.

```json
{
  "current_version": "0.1.1",
  "releases": [
    { "version": "0.1.0", "date": "2026-11-20", "tokens_this": 1204338, "tokens_cumulative": 1204338 },
    { "version": "0.1.1", "date": "2026-11-27", "tokens_this": 402115,  "tokens_cumulative": 1606453 }
  ]
}
```

### 24.2 `contributors/people.toml` — consent-verified credits

One entry per human contributor. AI agents and models are credited under About ▸ Models, never here.

```toml
[[person]]
name = "Jane Doe"
display = "Jane D."
url = "https://github.com/janedoe"
consented = 2026-11-01
```

The app's About ▸ Contributors screen renders only entries with a non-empty `consented` date. `CONTRIBUTING.md` states: "By adding yourself to `contributors/people.toml` in a pull request you consent to your `display` name and `url` being shown in the app's About screen and in the public repository."

---

## 25. External resource catalogs

Six public catalogs accelerate research, design, infrastructure, and agent-capability discovery. They are **discovery surfaces, not implementation sources**. Every entry adopted from them is subject to the same clean-room rules (§2.1–2.3), licence checks (§II.20.4), and measured-evidence rules (§6.4) as any other dependency.

### 25.1 public-apis/public-apis

**https://github.com/public-apis/public-apis**

A community-curated directory of **1,400+ free public APIs** across 50+ categories (Animals, Anime, Finance, Machine Learning, Security, Weather, and more). 90% support HTTPS; 26% explicitly support CORS. Maintained by contributors together with APILayer staff.

**Use for:**
- Sourcing a free, public API for a research spike or a v0.1.0 "one real workflow" (e.g. weather data, currency rates, public-domain imagery).
- Checking whether an API is free, HTTPS-capable, and CORS-enabled before designing around it.
- Recording the discovery in `docs/resource-catalog.md` with the API's own docs URL (not the catalog entry) as the source of truth.

**Do not:** treat a catalog listing as an integration spec. The API's own documentation is the source; the catalog is the index.

### 25.2 punkpeye/awesome-mcp-servers

**https://github.com/punkpeye/awesome-mcp-servers**

A curated list of **4,136 Model Context Protocol (MCP) servers**, organised by category (Aggregators, Art & Culture, Architecture & Design, Browser Automation, Cloud Platforms, Code Execution, Coding, Databases, and dozens more). Each entry carries language, scope (local/cloud), and OS markers. Synced with a web-based directory at glama.ai/mcp/servers.

**Use for:**
- Finding an MCP server that connects your agent to a database, browser, file system, or API without writing the integration yourself.
- Understanding what MCP servers exist before designing a custom one.
- Vetting a server against §II.20.4 before adoption: licence, maintenance signal, and security posture.

**Do not:** install an MCP server from the catalog without checking its own repository's licence and maintenance status. The catalog describes; the server's repo governs.

### 25.3 nexu-io/open-design

**https://github.com/nexu-io/open-design**

A **collaborative AI design agent workspace**, Apache 2.0, local-first and web-deployable, BYOK at every layer. Ships **31+ skills, 72+ design systems, 5 visual directions, 5 device frames**, and a plugin engine that wraps GitHub CLI so an agent can scaffold a plugin and open a PR. Supports 16 coding-agent CLIs via skills, CLI, and MCP.

**Use for:**
- **Designer role reference material** (II.27.A): study the design-system catalog, template structures, and visual directions as *public behavioural references* for your own original visual system.
- Understanding how a skill-driven design workflow is structured (skills → templates → design systems → plugins) when architecting your own design tooling.
- Adopting *patterns* (how a design system is described, how a skill is packaged), never code.

**Do not:** treat Open Design as a dependency for a native Rust GUI (II.30.1). It is a web-first workspace; its patterns are stack-mismatched to egui/eframe. Record the decision to use it as reference material in `docs/resource-catalog.md`.

### 25.4 shubhamsaboo/awesome-llm-apps

**https://github.com/shubhamsaboo/awesome-llm-apps**

**100+ open-source AI agents, agent skills, and RAG apps**, hand-built and tested end-to-end, Apache-2.0. Covers Agent Skills, Starter Agents, Advanced Agents, Game-Playing Agents, Multi-agent Teams, Voice Agents, MCP Agents, RAG, Memory Apps, and more. Works with Claude, Gemini, GPT, DeepSeek, Llama, Qwen, and other open-source models. Each skill ships real code and passes a security + eval CI gate in its own repo; adapt into `~/.config/dsh/skill/<name>/` per §19's preamble (the upstream `npx skills add` command is Claude Code-specific, not a DSH install path).

**Use for:**
- **Agent-skill discovery** (§19): the installable skills (Project Graveyard, First Reader, Scope Creep Detector, Commit Archaeologist, Dependency Doctor, Advisor Orchestrator Worker) are the canonical catalog of production-ready skills to evaluate.
- **Architecture reference** for LLM-app patterns (RAG, multi-agent, memory) when your own app has an LLM component.
- **Template scaffolding** for a v0.1.0 "one real workflow" — clone a template, adapt it, ship it.

**Do not:** clone a template and claim it as original. Every adopted template is a *starting point*; the final implementation must pass §2.1–2.2. Record the template's licence (Apache-2.0) and the decision to use it in `docs/resource-catalog.md`.

### 25.5 d4vinci/Scrapling

**https://github.com/d4vinci/Scrapling**

A Python web-scraping framework that handles everything from a single request to a full-scale crawl: fast HTTP requests, stealth browsers, JavaScript rendering, Cloudflare Turnstile/Interstitial handling, persistent sessions, CSS/XPath parsing, concurrent spiders, proxy rotation, and JSON/JSONL/CSV/XML exports. Ships an **MCP server** and an **Agent Skill** so any MCP-capable agent (including DSH) can browse and extract data. Converts pages to RAG-ready Markdown in one line (`Response.markdown()`).

**Use for:**
- **Research automation** (§2.4 step 2): fetch and convert public pages to clean Markdown for summarisation, without pasting raw HTML into the research notes.
- **MCP server** for agents that need browser automation or structured extraction during research Tasks.
- **Agent Skill** for scraping-based workflows (e.g. monitoring a public changelog or docs site).

**Do not:** scrape sites whose terms of service forbid it; the tool's stealth features are for legitimate public-data extraction, not for circumventing access controls. Record the decision to use it in `docs/resource-catalog.md` and check its licence (MIT) against `cargo deny` if adopted as a dependency.

### 25.6 ripienaar/free-for-dev

**https://github.com/ripienaar/free-for-dev**

A curated catalog of **700+ SaaS, PaaS, and IaaS offerings with permanent free tiers**, across 35+ categories relevant to infrastructure developers: cloud providers' always-free limits, CI/CD, monitoring, databases, DNS, email, crash handling, artifact repos, and more. Community-driven by 1,600+ contributors; a service qualifies only if it offers a free tier (not a trial) for at least a year, and TLS is never restricted to paid plans.

**Use for:**
- **Infrastructure discovery** when GitHub Actions alone is insufficient: free CI/CD alternatives, crash-reporting backends, log management, DNS, and database services that fit the suite's "completely free runtime" constraint (§0 question 13).
- **Fallback options** when a preferred service becomes paid or rate-limited; the catalog's recency is a signal of what is still free.
- **Recording in `docs/resource-catalog.md`** every free-tier service the suite depends on, so the dependency is visible at release time (II.24.3).

**Do not:** assume a free tier is permanent for your usage level. Re-check the service's own pricing page before a release; the catalog is a starting point, not a guarantee.

### 25.7 Use rules (seven rules)

1. **Discovery only.** A catalog entry is a *pointer*, not a spec. The entry's own source (the API's docs, the MCP server's repo, the design system's site) is the authority. Never commit a catalog's example code as your implementation.
2. **Record every adoption.** Any API, MCP server, template, skill, or free-tier service adopted from these catalogs goes into `docs/resource-catalog.md` with: the catalog URL, the source URL, the licence, the version or date pinned, and the decision (`DECISIONS.md` reference).
3. **Licence gate.** Every adopted item passes §II.20.4 (`cargo deny`, SPDX check, `ATTRIBUTION.md` entry). A catalog entry that says "MIT" is not sufficient — the source repo's `LICENSE` file is.
4. **Clean-room gate.** Adoption from a catalog does not relax §2.1–2.3. A template or design system studied from Open Design or awesome-llm-apps is still subject to the "could this have been written without access to the internals?" test.
5. **Free-tier verification.** Any service from `free-for-dev` used in the release pipeline or runtime is re-checked against the service's own pricing page before each release (II.24.3).
6. **No vendor lock-in by accident.** A catalog may surface a service that is the only free option for a need. Record the lock-in risk in `DECISIONS.md` and name a fallback before adopting it.
7. **No runtime-specific install.** A catalog entry that only documents a Claude Code / Cursor / Codex install path is not installed on a DSH machine. Use it as reference or skip it.

## Appendix: Version & Sync Record

- **Guide version:** 2026-11-22 (`~/guide.md`, 1,990 lines)
- **Sections read:** Part I (§0–§18), Part II (§II.18–§II.30), and §19–§25 — read in full
- **Last sync:** 2026-11-22
- **Next sync required:** When guide.md is updated
- **Local AGENTS.md last updated:** 2026-11-22
