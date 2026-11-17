# AGENTS.md

# Dust Trail

Original Western-themed ASCII adventure game, playable as TUI and GUI Rust
application. Adhere to `~/guide.md` (kept outside the repo, never committed) —
the single source of truth.

## Purpose

Build a free, open-source ASCII adventure game inspired by the Western genre
(original mechanics, art, and name — no Rockstar trademarks or trade dress).
Playable as a terminal TUI and an egui/eframe GUI.

## Standing rule: Read guide.md every section in complete detail

Before any work that touches a capability area governed by guide.md, read **every**
section of `~/guide.md` in full: Part I (mission, clean-room rules, technology
stack, operating model, progressive releases, suite infrastructure, path to a
finished app, quality control, UI/UX, source of truth, architectural stress
test, execution sequence) **and** Part II (naming/branding, legal depth,
security audit, accessibility, marketing, maintainer sustainability, definition
of done, launch readiness, open questions, designer role, clean-room research,
design vision, implementation, perceived performance, parity review, naming
conventions, personal data/secrets, Rust GUI/UX) **and** Appendix A (client
mode, MCP, exhaustive implementation process). Do not rely on memory or partial
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

## 4. Local-First Testing + GitHub Actions for Heavy Work (guide §4)

1. Produce a binary that runs on the human's local OS as early as practical.
2. Prefer GitHub Actions for full workspace builds, large tests, Clippy, multi-target packaging.
3. Pattern: push → CI builds → download artefact / run binary for UX testing.
4. Never launch unbounded local builds that risk freezing the PC.
5. **HARD BUILD RULE — NEVER REVERT:** No model, agent, or session may run `cargo` or any other resource-intensive toolchain locally. All Rust/Node/test/build commands run **only** in GitHub Actions via `~/.config/opencode/scripts/ci.sh`. This is enforced globally by the `opencode-ci-only` plugin and is non-negotiable for every app in this suite. Local work is edit/read/git only.
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

Heavy verification (builds, tests, clippy, packaging) runs only through `~/.config/opencode/scripts/ci.sh`:

- `ci.sh run "<description>"` — commits the working tree to a branch, pushes it, opens or updates a pull request so CI triggers, waits for checks, prints the outcome including failing job logs.
- `ci.sh status` — current check state.
- `ci.sh logs` — failing job logs.
- `ci.sh diff` — what would be committed.
- `ci.sh merge` — merges the current PR from the Dev account once checks are green.

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

---

## 5. Operating Model — Chat, Task, and Pull Request (guide §5)

```
Chat (Plan + Research)  →  Task (Execute)  →  Pull Request (Review)  →  Local Test  →  Merge
→  Semver Release + Release Notes  →  Update ROADMAP
```

### 5.1 Modes

**Chat (Orchestrator):** discovery, public research, architecture, Task briefs, roadmap. No large code dumps; no heavy local builds.

**Task (Coder):** one bounded vertical slice; implement; test; commit; push; PR.

### 5.2 Free Model Policy

1. **OpenCode free models** first.
2. When exhausted → **NVIDIA free models** via API.
3. Process unchanged across providers.
4. No Ollama / local model hosting.
5. Orchestrator discovers available free models and assigns best per role. Record in `docs/current-model-map.md`.

### 5.3 Canonical Agent Roles

`.opencode/agents/`:
- **Orchestrator** — plan and research only; no implementation.
- **Coder** — execute one Task.
- **Explorer/Researcher** — gather public context.
- **Reviewer** — check PR vs acceptance criteria + clean-room rules. Read-only.
- **Designer** — owns design vision, capability inventory, and parity review (II.27).

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
6. **Look for a better pull request.** If another fixes the same thing and is better or equal, close yours and say why.
7. **Report only what ran.** In CI status summaries, report only the jobs that actually executed — never fabricate pass/fail for steps that were skipped or masked.
8. **Main `main` through PR only.** No direct pushes to `main`; every change must land via a verified pull request merge. Branch protection with `enforce_admins` and the required checks in `.ci-required.txt` enforces this.
9. **Secrets are never committed.** No API keys, tokens, or credentials in git or release binaries. Crash reports and update checks use user-supplied tokens only; no embedded privileged bot token ever ships in a client binary.
10. **CI stays shared.** Workflow definitions live in a shared suite repo; feature PRs must not alter the shared CI definitions — make a new issue/PR in the shared repo instead.
11. **Guards say why.** Any agent or tool that blocks or gates work must state the concrete reason in plain language — not just "blocked" or "failed."
12. **Instructions must be readable by everyone.** No notes about one machine's private configuration; all steps must be runnable by any suite contributor.
13. **PR movement SLA.** Open PRs receive a first-response comment within 7 days and a decision (merge, revise, or close) within 14 days; stale PRs (30 days unmodified) are automatically candidates for closure.
14. **No agent attribution in PRs.** PR descriptions and merge commits must not claim agent or bot authorship. Models are credited separately (About ▸ Models), never as people.

CI integrity check: A CI job verifies that the repository's `AGENTS.md` house-rules block matches the current version stored in the shared `craft-contrib` standards repo. If the local guide.md is updated, the agent must also update the repo's `AGENTS.md` and commit the change via `ci.sh`.

---

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

- Panic hook / error boundary: app version, OS, backtrace (scrub secrets/paths), optional user notes.
- **Mandatory scrub before the payload leaves the process:**
  - Every absolute path is stripped back to its last segment (`/home/alice/foo` → `foo`; `C:\Users\alice\foo` → `foo`).
  - Env-var values matching the II.29.2 patterns are replaced with `<redacted>`.
  - Any `Authorization:` header, `token=`, `api_key=`, `secret=`, `password=` value is removed entirely.
  - Backtrace module paths keep the crate name but drop the workspace path.
- **UI preview before send**; the user can cancel.
- **Never auto-send.** The user always presses a button.
- **GitHub Issues:** optional user token or device flow to create issues (`crash`, `bug`, `user-report`).
- Fallback: pre-filled New Issue URL if no token.
- **Never** embed a privileged bot token in client binaries.
- Document privacy in README.

### 7.2 Update Check — Standard Flow

Every suite app embeds the shared `crates/update-check` crate and drives **one Updater instance per app**. The crate owns the whole flow; the app calls `tick()` / `show()` each frame and acts on the returned action, so every app handles failures, dropped workers, repaints and restart identically.

**Check mechanics**
- Query GitHub Releases API (`/repos/<owner>/<repo>/releases?per_page=30`) on a background thread; compare semver to the running version.
- **Quiet startup check, once per run:** a failure is logged, never shown as a modal; the app starts normally.
- **Help ▸ Check for Updates (explicit):** reports a visible status — Checking / Up to date / Update available / Failed (with the error). A check already running becomes the requested one.
- Fail soft offline; conditional GETs (ETag / If-None-Match); brief cache; handle GitHub's rate limit gracefully (tell the user, stop checking until reset).
- Semver rule: a dev/integration build (e.g. `0.17.0-integration.20261029.f5b3f123`) is **not** offered the release it already contains; a release is offered only when strictly newer.

**Primary in-app update UX (canonical)**
Every suite app provides an **integrated, one-click update mechanism entirely within the application GUI**. The user must never be directed to a website, GitHub Releases page, command line, or package manager to initiate an update. The update flow is:
1. **Background check:** On startup (once per run), the app quietly checks GitHub Releases API for a newer version. If the app is up to date, no UI is shown.
2. **Subtle notification:** When a newer release is found, a small, non-intrusive banner/toast appears at the top or bottom of the main window — not a modal that blocks the app. It states: "New version available: X.Y.Z. Select to update."
3. **One-click update dialog:** Selecting the banner opens a standardized inline dialog (within the app's normal window, not a separate modal) with: Current version and new version; Release highlights (Added/Changed/Fixed, derived from release notes); Two buttons: **Update & Restart** and **Later**.
4. **Update & Restart:** The app performs the full update pipeline internally: Downloads the platform-specific installer to a temp location; Verifies SHA-256 against `SHA256SUMS.txt` and the minisign signature; Runs the stale-version remediation (removes old app-drawer entries); Stages the new version under a hidden name → renames it into place → Restarts the app automatically (or waits for user launch). All of this happens without the user leaving the app, opening a browser, or running any commands.
5. **Later:** Dismisses the notification; an installation already in progress continues and reports through the updater.

**Never direct the user to GitHub or a browser for routine updates.** The only exception is if the update flow fails at step 4 after signature verification — in that case, a "Download manually from release page" link may be offered, but this is a failure path, not the normal flow.

**Platform-specific installer detection.** The app detects how it was originally installed (macOS `.app`, Windows `.msi`/portable zip/standalone, Linux AppImage / deb / rpm / pacman / Flatpak / standalone) and only offers self-update for packaged copies in supported formats. Development builds (`cargo run`) and package-manager-owned copies defer to their respective managers.

**Anti-loop marker.** A version just installed is recorded with a timestamp and not re-offered for 24 h, so a tag/binary version mismatch cannot loop update → restart → "update available".

**Failure recovery:** a failed self-update writes its error to a log file shown once at the next start; the update stays on offer.

**Opt-out and localization**
- An env var / build flag **disables update checks entirely** for distro packaging (Nix, Flatpak, deb/rpm repositories) — the system package manager owns updates there.
- All updater messages (prompt, status, failure notes) follow the app's **interface language**, translated at draw time; network error text passes through as received.

---

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
- **Require status checks to pass** — the authoritative CI checks in `.ci-required.txt` are required and **direct pushes are disabled**.
- **Require linear history.**
- **Do not allow force-pushes.**
- **Do not allow deletions.**
- **`enforce_admins: true`** — no admin bypass.
- **Restrict who can push** to the Dev account where the plan allows; otherwise CI check enforces.
- **Signed commits recommended** (`require_signed_commits: true`) — accept if the CI signing path is reliable; skip otherwise and record the reason.
- **Require conversation resolution before merging.**

Record the exact settings in `docs/dev.md` with the date.

### 9.4 Release and in-app gates

- **Release notes and token metrics:** Every semver release includes detailed release notes (app usage / feature changes) plus token usage metrics: total tokens consumed since beginning development and tokens used in this release. Release notes published to GitHub Releases and embedded in-app.
- **In-app version & release notes:** App displays version number accurately but subtly in a UI-appropriate non-intrusive corner. Selecting the version opens a dedicated release-notes pop-up showing the full release history, detailed app usage per release, and cumulative / per-release token usage metrics.
- **Auto-update check:** Every app checks for a newer release, notifies the user in a simple user-friendly way, and offers one-click auto-update from the latest GitHub Release. The check, notification, and update flow must be simple, consistent across all suite apps, and non-intrusive to normal usage.
- **Contributor credits system:** The app includes a credits screen (About ▸ Contributors) with consent-verified entries from `contributors/people.toml`. AI agents and models are credited separately under About ▸ Models, never as people. Pull requests must never claim agent or bot authorship.
- **Build attestations (optional):** The release pipeline may include an SLSA provenance and Sigstore attestation step (`gh attestation verify`). Optional but recommended for projects with formal security requirements.

---

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
- `docs/parity.md`
- `docs/capability-inventory.md` — ID / Product / Capability / Observable behaviour / Acceptance criteria / Source URLs / Status
- `docs/process-feedback.md` (append-only agent friction log; see addendum N)
- `docs/current-model-map.md`
- `docs/design/brand.md`
- `docs/tokens.md` — token usage metrics per release (§24)
- `docs/dev.md` — branch-protection settings, Dev account login, release-workflow notes
- `docs/update-signing.md` — current public key fingerprint, rotation history
- `docs/resource-catalog.md` — the projects adopted from §25, with licence, version pin, and the decision to use them recorded
- `ROADMAP.md`
- `IMPLEMENTATION_PLAN.md`, `REQUIREMENTS_TRACEABILITY.md`
- `DECISIONS.md`, `PROGRESS.md`, `CHANGELOG.md`, `ATTRIBUTION.md`
- `docs/qc/`, `docs/development.md`
- `.github/PULL_REQUEST_TEMPLATE.md`
- `docs/gaps.md` (ranked worklist maintained per release)
- `docs/devlog.md` — **local-only**, gitignored. One entry per session: what landed, version numbers, still-open items — the continuity record when sessions end abruptly.

Separate **Facts / Assumptions / Decisions**.

---

## 12. Suggested Repository Structure (guide §12)

```
{product-or-suite}/
├── .github/workflows/ci.yml
├── .github/workflows/release.yml
├── .github/PULL_REQUEST_TEMPLATE.md
├── .opencode/agents/
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
- File ownership is strict: an agent may only edit files it "owns" (marked by a `OWNERS` file or agreed convention).
- `Cargo.toml` writes are atomic: modify a copy, validate with `cargo check --all-targets`, then replace the original. A broken manifest for one agent breaks the whole workspace.
- Each agent's target directory is capped at ~10 GB disk usage; if nearing the limit, clean or archive old builds before continuing.

---

## 13. Architectural Stress Test — Gaps Found & Mitigations (guide §13)

| Gap | Risk | Mitigation |
|-----|------|------------|
| Vague "first functional build" | Endless foundation without a shippable slice | **Hard v0.1.0 gate** (Section 6.1) |
| No PR discipline | Agents merge incomplete or dirty work | **PR template + checklist** (Section 8.3) |
| Release path learned too late | Broken tags/artefacts at the worst time | **CI + release.yml from day one** (Section 4) |
| Research → code without review | Copyright bleed, UI cloning | **Human gate on research** (Section 2.3, 8.4) |
| Roadmap optimism | False "ready for work %" | Update % only from verified evidence |
| PR/issue authorship drifts to personal account | Inconsistent audit trail | **Dev-account assertion in `ci.sh` + PR-author CI check** (II.26.7) |
| Suite without shared crates | Inconsistent crash/update behaviour | Shared crates required |
| Free model web search missing | Stalled research | Browse model/MCP → CI fetch → human ladder |
| Heavy local builds | PC freeze | **Non-negotiable Actions-only** (Section 4 item 5) |
| Meta-guide in git | Process secrets / confusion | **Never push this file** |
| Trademark-adjacent naming | Legal demand letters | Discovery forces original name; legal checklist |
| "Drop-in replacement" claims | Misrepresentation | Qualified claims only |
| Crash payload leaks | Secret / path / token exposure | **Mandatory scrub** (Section 7.1, II.29.2) |
| Signing key compromise | Malicious update to every user | **Rotation procedure** (Section 7.2) |
| Token metrics promised, not measured | Requirement no one can keep | **Schema in §24, file in `docs/tokens.md`** |
| Token-reduction tools used as a substitute for real verification | False confidence; "the proxy said it saved 90%" | **Measured-only rule** in the §19 token-efficiency subsection (rule 3); `docs/tokens.md` is the only source of token numbers |
| External catalogs treated as implementation sources | Copying a catalog's examples, or trusting a catalog entry blindly | **§25 use rules**: catalogs are discovery-only, every adopted entry is verified at its own source, licence and API terms checked |

**Remaining residual risks (accepted, monitored):** free-model capability variance; human required for research approval and release honesty; patent landscape cannot be fully automated by agents.

---

## 14. Start-to-End Execution Sequence (guide §14)

1. Discovery → human confirmation.
2. Name locked; public repo created (**never push this guide**).
3. Scaffold + CI + **release.yml** + **PR template**.
4. Tracking files + stub ROADMAP.
5. Discover free models; assign roles.
6. Tasks until **v0.1.0** criteria met → tag Release.
7. Expand ROADMAP; human-gated research; shared crates.
8. Chat → Task → PR → local test → merge → semver Release loop.
9. Multi-platform expansion; polish; QC.
10. Section 9 gates + section 2.5 legal checklist → finished.
11. OpenCode free exhausted → NVIDIA free; process unchanged.

---

## 15. Session prompt (guide §15)

**Canonical session prompt lives in `addendum.md` section L** (client mode, Rust QA, process feedback).

If only this guide is available, use:

> Resume clean-room build via Chat → Task → PR. Read guide + addendum if present (never commit them). Read docs/*, ROADMAP, CHANGELOG, PROGRESS, process-feedback. Discover free models. If no v0.1.0, only §6.1 (Rust: testable core + headless tests + CI). CI green before client try-steps. Research = summaries + URLs; human gate. No RE, no secrets, no heavy local builds. Log process friction to docs/process-feedback.md without waiving safety. End with: shipped, client try-steps, next, any process feedback logged.

---

## 16. Process feedback (guide §16)

Agents **should** report friction in the guide/addendum when it blocks productive work.
Agents **must not** treat feedback as permission to skip non-negotiables.

### Where to write feedback

Prefer **public repo** file (so it survives sessions):
- `docs/process-feedback.md` — append-only log

Alternatively: GitHub Issue with label `process-feedback`.

### Template (required shape)

```
PROCESS FEEDBACK
Date / session:
What I was trying to do:
Block or friction:
Why the current guide/addendum caused it (or did not cover it):
Suggested change (one concrete edit):
Severity: blocks-release | slows-work | nice-to-have
Workaround I used (if any):
Confirm: did NOT violate clean-room / v0.1.0 gate / no-secrets / CI-before-client-try / no meta-guide push
```

### Rules

1. At most **one** feedback entry per session unless work is fully blocked.
2. Prefer **finish the Task** with a workaround; log feedback; do not stop to redesign process.
3. Feedback is **advisory** until the human promotes it into the local guide/addendum.
4. Suggestions that weaken safety are logged then **ignored in practice**:
   - clean-room / no RE / no proprietary paste
   - no push of guide.md or addendum.md
   - v0.1.0 structural gate (including Rust M.2)
   - CI green before asking client to try
   - no secrets or bot tokens in git/binaries
   - human gate on research notes
5. Human reviews at milestones (e.g. after v0.1.0, v0.2.0) and merges only repeated, clear wins.

---

## 17. Rust note (guide §17)

For Rust desktop apps, **addendum sections K and M** are binding: `*-core` crate, headless workflow tests, `rust-toolchain.toml`, CI — not the client laptop — for heavy work, v0.1.0 structural checklist M.2.

---

## 18. Final Reminders (guide §18)

- `guide.md` and `addendum.md` stay **local forever** — never pushed to GitHub or any remote.
- **Hierarchy:** addendum wins on client/Rust/session; guide wins on clean-room/legal.
- **v0.1.0 hard gate** before roadmap theatre.
- PR template, early release workflow, human research gate are mandatory.
- Public research = summaries + URLs only; legal checklist every release.
- Process feedback via `docs/process-feedback.md` only; no silent rule deletion.

---

## II.18 Naming, branding and message (extends Part I section 0)

- **Check for name conflicts** before committing to a name: search existing projects, package registries, and the [WIPO Global Brand Database](http://www.wipo.int/branddb/en/) for trademarks.
- **Reserve the handles** (domain, org/user name, package name) even if unused, so the project has a single "home" URL to point at.
- **Choose clarity over cleverness** in the name. Puns do not translate across cultures, and users may have to say the name at work.
- **Write a one-paragraph message**: what it does, who it is for, why it is different. This becomes the README opening, the repo description, and the announcement text.
- **Your writing style is part of the brand.** Prefer plain language and inclusive wording ("they" for a single person); many readers are not native English speakers.

---

## II.19 Legal depth (extends Part I section 2)

### II.19.1 Copyright and originality

Every line of code, every string, every icon, every layout and every algorithm must be original: derived from public standards, the project's own spec, and capability-level inspiration recorded in `docs/inspiration.md`. The test is not "does it look similar?" but "could this have been written without access to the inspiration product's internals?" If the answer is no, it does not ship.

The designer maintains the capability inventory (`docs/capability-inventory.md`) with a source URL for every claimed behaviour. No capability may be implemented until its source URL is recorded and its wording is original.

### II.19.2 Trademarks and naming

The product name, window title, and visual identity must be original. Search existing projects, package registries, and the [WIPO Global Brand Database](http://www.wipo.int/branddb/en/) for conflicts. Reserve handles. Choose clarity over cleverness. Record the decision in `DECISIONS.md` once; do not revisit without a new decision entry.

### II.19.3 Licence

Prefer dual licence **MIT OR Apache-2.0**. Before the first public release, `ATTRIBUTION.md` must list every third-party crate with its licence, and `cargo deny` (`deny.toml`) passes in CI. Report the licence accurately in any SBOM.

### II.19.4 Secrets and tokens

No API keys, tokens, or credentials in git or release binaries. Crash reports and update checks use user-supplied tokens only; no embedded privileged bot token ever ships in a client binary.
- Prefer the platform credential store / keychain for any persisted user token.
- Request only minimal scopes (`public_repo` or finer).
- Secret scanning (gitleaks) runs in CI on every PR and is a required status check.

### II.19.5 Release legal checklist

The checklist in §2.5 runs before **every** public release, not just v1.0.0. The release is not cut until every row is checked and recorded in `PROGRESS.md` with the CI run ID as evidence.

---

## II.20 Security-audit skill

### II.20.1 What is in scope

- The application itself on every supported platform.
- Parsing and loading of the app's own document and library files, CSV import and JSON backups (atomic write → rename, no path traversal).
- The optional update check (GitHub Releases API, soft-fail offline) and the crash-report submission path (user token, pre-filled issue URL fallback).
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

Reports go through GitHub's private vulnerability reporting (Security tab → "Report a vulnerability", or a draft advisory). Acknowledgement within seven days — a firm commitment given the project's spare-time cadence. Coordinated disclosure: the maintainer confirms, judges severity, prepares a fix, and agrees a disclosure date with the reporter. Reporters are credited unless they ask not to be.

### II.20.4 Scanner gates

Every PR runs the applicable scanners: secret scanning, SAST, and a dependency/licence check (`cargo-deny` for Rust projects). The SBOM (CycloneDX) is generated and attached to every release. These gates are required status checks on `main` (`.ci-required.txt`); direct pushes are disabled.

---

## II.21 Accessibility

### II.21.1 Commitment

Accessibility is a correctness property of the UI, not a nice-to-have. The project targets **WCAG 2.2 Level AA where feasible** for a native desktop application. "Where feasible" is deliberate: egui is an immediate-mode toolkit, some WCAG criteria are written for web content, and a few cannot be met exactly by a native renderer. Where a criterion cannot be met, the limitation is recorded in `ACCESSIBILITY.md` rather than glossed over.

### II.21.2 Primary priorities

1. **Keyboard support.** Every flow (create an item, edit it, traverse the main views, open and dismiss dialogs) must be completable from the keyboard. Focus order must be sensible and focus must be visible.
2. **Screen-reader semantics.** egui integrates AccessKit, exposing a semantic tree on supported platforms (Windows and macOS today). Interactive widgets must carry meaningful roles, names and values.
3. **Colour contrast.** Text and meaningful UI meet WCAG AA contrast in both dark and light themes; colour is never the only signal for state.
4. **Reduced motion.** Platform reduced-motion preferences are honoured: animations minimised or disabled.

### II.21.3 Severity scale for accessibility defects

| Severity | Definition |
|----------|-----------|
| Blocker | A core flow cannot be completed from the keyboard, or a screen reader cannot announce a required control |
| Critical | Focus is trapped, invisible, or lost; state is conveyed by colour alone |
| Serious | Contrast below AA on text or meaningful UI; a dialog cannot be dismissed with Esc |
| Moderate | Reflow at 200% or narrow window clips or overlaps a control |
| Minor | Cosmetic contrast or focus-ring inconsistency |

### II.21.4 Testing method

Run the accessibility smoke test in `docs/qc/checklist.md` whenever the UI is touched: keyboard-only walk-through, dialog Esc dismissal, screen-reader spot check (Narrator/NVDA/VoiceOver), reflow/zoom at 200% and narrow window, and high-contrast check in both themes. Record the result in the QC run log. Screen-reader behaviour is verified by manual spot-checks, not an automated CI suite — this limitation is recorded honestly in `ACCESSIBILITY.md`.

### II.21.5 Reporting path

Accessibility issues use the dedicated accessibility issue template (`.github/ISSUE_TEMPLATE/accessibility.yml`) so they are triaged with the same seriousness as other bugs.

---

## II.22 Marketing

### II.22.1 Message

Write one paragraph: what the app does, who it is for, why it is different. This becomes the README opening, the repo description, and the announcement text. Prefer plain language and inclusive wording ("they" for a single person); many readers are not native English speakers.

### II.22.2 Handles and presence

Reserve the handles (domain, org/user name, package name) even if unused, so the project has a single "home" URL to point at. The writing style is part of the brand — consistent across README, repo description, release notes, and issue templates.

### II.22.3 What marketing is not

No false affiliation or "drop-in replacement" claims unless carefully qualified. No claims of compatibility certification from commercial vendors unless true and authorised. Release notes are honest about scope and limitations; they do not over-promise.

---

## II.23 Maintainer sustainability

### II.23.1 Release cadence

Most projects in this workspace are maintained by one person in a few hours a week. The seven-day response expectation (acknowledging issues and PRs) is published and honest — a firm commitment, not a guarantee of an instant fix. Scope is kept small enough that this cadence is sustainable: one slice at a time, one release at a time.

### II.23.2 Burnout guardrails

- Scope creep is reflected back to the client: "That's a new milestone; approve to add to ROADMAP?"
- The Definition of Done (II.24) is applied per slice so work finishes instead of accumulating.
- Releases are progressive (0.1.1, 0.2.0, …) with detailed notes; there is no "big bang" launch that concentrates risk.
- The maintainer does not write code the client could write; the client's role is goals, priorities, and "does this work / feel right?" — not borrow-checker fixes.

### II.23.3 Human gate

Research notes (`docs/research/*.md`) require explicit human approval before dependent implementation. The agent proposes; the human approves or rejects. This prevents the maintainer from being surprised by unapproved scope.

---

## II.24 Definition of Done

### II.24.1 Per slice (every Task)

- Acceptance criteria met with real behaviour — no fake UI, no mock data.
- Unit tests cover happy path, boundary, and failure/edge cases; headless domain tests need no display.
- Code is formatter- and linter-clean in the project's own toolchain (for Rust, CI runs `cargo fmt --all -- --check` and clippy with `-D warnings`; no toolchain command is run locally).
- CI green on GitHub Actions: the project's format → lint → test → release-build chain passes on every supported platform.
- No fake success states, dead controls, or placeholder-only data.
- **No personal data and no secret in the change.** The scan in II.29 is clean across the working tree, the history and the commit metadata. Placeholder, seed, sample and fixture values are obviously synthetic (`Jane Doe`, `dev@example.com`, `example.com`), never a real person's.
- `CHANGELOG.md`, `PROGRESS.md`, and `REQUIREMENTS_TRACEABILITY.md` updated with exact CI results.
- Change is small and reviewable; PR opened using the template, **opened from the Dev account** (II.26.7).

### II.24.2 Per phase

- All phase requirement IDs traced to `DONE` with CI evidence.
- `REQUIREMENTS_TRACEABILITY.md` ↔ `IMPLEMENTATION_PLAN.md` consistent (no orphan IDs).
- Open assumptions reviewed in `docs/assumptions-and-questions.md` (answered or marked still-open with Q-IDs).
- `DECISIONS.md` has an entry for every user directive this phase.

### II.24.3 Release / packaging

- Cross-platform build artefacts verified on every supported platform: each CI run attaches binaries plus the lockfile used for the build.
- App launches on a clean machine; save → reopen → identical state.
- Version bumped; `CHANGELOG.md` has a release entry.
- No secrets/keys committed; no local-only files in the tree.
- `LICENSE` present; `ATTRIBUTION.md` complete and accurate.
- Release assets (`II.24.3`) scanned for secrets and personal data; the SBOM and any installer carry no maintainer home path or personal identifier.

---

## II.25 Launch readiness

Checked before a public launch or a major release. Items already in §9 are not repeated.

- `README.md` answers, in order: what this does, why it is useful, how to get started, and where to get more help.
- `SECURITY.md`, `CODE_OF_CONDUCT.md`, `CONTRIBUTING.md`, `GOVERNANCE.md`, `VISION.md`, `ACCESSIBILITY.md` and `CONTRIBUTORS.md` are present and current.
- The issue forms (`bug_report`, `feature_request`, `accessibility`) and the pull-request template are in place and render correctly on GitHub.
- Branch protection on `main` requires the authoritative CI checks (`.ci-required.txt`); direct pushes are disabled.
- The stated response expectation (a reply within seven days, maintained in spare time) is published and honest.
- Supported platforms and privacy claims in `README.md` match the built binaries and the code — no overstatement.
- No false affiliation or "drop-in replacement" claims anywhere in the public docs.
- Scanner gates are active and green: secret scanning, SAST, dependency/licence audit, and an SBOM generated and attached to the release.
- Dependabot is enabled for the project's package ecosystem(s) and GitHub Actions.
- No open PR older than 14 days without a decision.

---

## II.26 Open questions for the human

### II.26.1 "Local tests must be runnable" vs "no heavy work locally"

- **Status (2026-10-25): partially applied — still awaiting your formal approval.** Adoption is uneven across the repositories: some already document the exact local build and test commands and name CI as authoritative for the maintainer's machine; some name CI as authoritative but omit the commands; at least one has no `CONTRIBUTING.md` at all. Bringing every repository to the same wording is open work, not a decision — it needs no answer from you, only the edit. Nothing in Part I was reworded.

### II.26.6 Licence choice: "MIT OR Apache-2.0" default

- **Part I section 2.2 item 9** ("Prefer dual licence **MIT OR Apache-2.0**") and the licence row of the section 2.5 checklist.
- **opensource.guide** calls MIT the safest default for a blank slate, notes Apache-2.0's patent grant appeals to large companies, and warns that a dual "OR" expression should be deliberate.
- **No conflict** — but confirm the "OR" is intentional for each project, and that `ATTRIBUTION.md` and any SBOM report it accurately.

### II.26.7 The git-identity rule covers pull requests, but GitHub cannot enforce it

- **The workspace git-identity rule** says every git action — "every commit, pull request, merge, and tag" — is authored as `"<App Name> Dev" <dev@example.com>`.
- **What is enforceable:** the *commit object*. `git config --local` sets author and committer, and `ci.sh` asserts the identity before committing and refuses any mismatch. That part of the rule holds.
- **What is not enforceable:** the **pull request's author** and the **pusher shown on GitHub**. GitHub attributes both to whichever account holds the push credential. With a personal access token that is the human operator's real account, no matter what the commit metadata says. The same applies to the "merged by" line on a merge.
- **So the rule as written cannot be satisfied for PRs** on the current setup. It is not a mistake in practice, it is a missing mechanism: satisfying it needs either a bot/machine account whose credential performs the push, or a GitHub App acting for the project.
- **Second, concrete case:** a repository may be a fork of an upstream this account cannot push to, with a configured fork remote that no longer exists. There is then no writable remote at all: commits can be authored correctly but never pushed, and CI cannot be verified. That is a missing remote, not an identity failure — but it has the same effect on the gate in section 9.
- **Options (needs your decision):** (a) narrow the rule to "every *commit*", and state plainly that PR/merge authorship follows the pushing account — the honest option, and it matches what CI can actually check; (b) create a machine account and push with its token, so the rule holds end-to-end; (c) leave the rule as written and record PR authorship as a known, accepted exception. I have **not** changed Part I text; this is recorded for your decision only.

---

## II.27 Designer role and standing responsibilities

The designer is a standing role — not a final polish phase. Four responsibilities run continuously from the first slice to the last release:

**(A) Clean-room analysis of the inspiration product.** Study what the reference product(s) in `docs/inspiration.md` actually do, from public sources only, recording the URL for every claim: official documentation, manuals, tutorials, blogs, talks, public screenshots and demo videos (for workflow understanding only — never pixel-copying), public API specifications, and open standards. The method is black box: enumerate what the product *does* — every screen, workflow, command, setting, file format, import/export path, keyboard shortcut, and observable edge-case behaviour. Forbidden absolutely: decompiling or reverse-engineering proprietary binaries; copying icons, fonts, exact layouts, string tables, presets or internal algorithms; using prior reverse-engineering output; reproducing trademarked names, logos or trade dress; committing verbatim copyrighted text or third-party screenshots; pixel-perfect UI cloning. Write original behavioural summaries plus URLs under `docs/inspiration/` (consolidated into `docs/inspiration.md`). New research requires owner approval before implementation depends on it (guide Part I §2.3, §8.4).

**(B) The capability inventory.** `docs/capability-inventory.md` defines "all the features and abilities of the app we are designing." A capability not in it can be neither claimed done nor reported missing. One row per capability: ID (e.g. `CAP-014`), Product, Capability, Observable behaviour, Acceptance criteria, Source URL(s), Status (not started / partial / implemented / out of scope). Keep enumerating until a pass over the public surface yields nothing new. Record implementation evidence (test name, CI run) in `docs/parity.md`. Move readiness percentages only from verified evidence. If a capability cannot be delivered within the clean-room rules, mark it `out of scope` with the reason and raise it via the `question` tool — never drop it silently.

**(C) Design vision and its implementation.** The designer owns the design and builds it: information architecture, interaction model, visual system (colour, type scale, spacing, radii, elevation, motion), iconography, voice and tone of UI text. The designer may edit source to realise the design, working with `ui-dev` where one exists; domain and business logic stay out of the UI and go to `domain-dev`. Every state is designed: empty, loading, error, disabled, permission-denied, long content, narrow window, high-contrast, reduced motion. No default or unstyled controls, no placeholder or lorem text, no clipped text, no mismatched iconography, no dead controls, no cluttered toolbars. When a tool set grows beyond a handful of items, use a drop-down menu grouped by family rather than an inline strip. Outside the app: logo, wordmark, app icon set (all platforms), splash/installer art, illustration style, docs diagrams, README/social graphics, and `docs/design/brand.md`.

**(D) Parity review.** Before any slice, milestone or release is called done, check the app against the inventory and report per capability ID: met, partial, missing, or deliberately out of scope (with reason), each with evidence. The application is not complete while an in-scope capability is unmet. Judge the rendered product, not the intention: present but unreachable, unclear, or broken in a real state is partial, not met. Where we exceed the inspiration product, record it — parity is a floor, not a ceiling. Report honestly: an inflated parity claim is worse than an admitted gap.

---

## II.28 Naming conventions inside a repository

Part I section 0 fixes the **product** name and II.18 covers brand and handles. Neither says how the files *inside* the repository are named. This section states the convention, generalised from the suite's reference implementation so it applies to every app.

The shape is `<app>-<role>` / `<app>-<os>-<arch>`. In Rust source the hyphen becomes an underscore, so the same convention reads `<app>_core` in code.

### II.28.1 What carries the app prefix

The app name goes first for all of these:

| Kind | Shape | Rule |
|------|-------|------|
| Workspace crates (package names) | `<app>-core`, `<app>-commands`, `<app>-persistence`, `<app>-ui`, `<app>-app` | `<app>-<role>` — the crate name states **which app** and **which layer**, so a `Cargo.lock`, a log line or a `cargo test -p` argument is unambiguous on its own. |
| Crate identifiers in Rust | `<app>_core`, `<app>_app` | The hyphenated package name with `_`; that is cargo's own rule, not a choice. |
| The binary | `<app>` | Just the app name: the user runs the app, not `<app>-app`. |
| Release artefacts | `<app>-linux-x86_64`, `<app>-linux-aarch64`, `<app>-windows-x86_64`, `<app>-macos-aarch64` | `<app>-<os>-<arch>`. |
| Checksum file | `SHA256SUMS.txt` | Fixed name, listed beside the artefacts so `sha256sum -c SHA256SUMS.txt` works from the release page. |

Why the prefix earns its keep: an artefact called `<app>-linux-x86_64` cannot be mistaken for another app's build when it lands in a shared download directory, a package mirror or a user's `~/Downloads`, and a crate called `<app>-core` cannot collide with a generic `core` in a dependency tree. Self-update and asset-picking code depends on those artefact names, which is the strongest reason to fix them before a release exists rather than after.

### II.28.2 What does **not** carry the prefix, and must not

This is the part that is easy to get wrong, so it is written out explicitly. These names are **fixed by cargo, git or GitHub**, and prefixing them breaks the tooling:

| Kind | Correct | Why a prefix breaks it |
|------|---------|------------------------|
| Manifest | `Cargo.toml` | Cargo discovers crates by this exact filename; `<app>_Cargo.toml` is simply not a crate. |
| Crate roots | `src/main.rs`, `src/lib.rs` | Fixed entry points. |
| Module files inside a crate | `engine.rs`, `report.rs`, `update_check.rs` | The module path *is* the name (`<app>_core::engine`); an `<app>_engine.rs` module would be reachable only as `<app>_core::<app>_engine` and reads as stutter. Keep these plain. |
| Integration tests | `tests/properties.rs` | Discovered by path; the crate name is already in the package. |
| GitHub community files | `README.md`, `LICENSE`, `LICENSE-MIT`, `LICENSE-APACHE`, `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, `SECURITY.md`, `CHANGELOG.md`, `GOVERNANCE.md` | GitHub only recognises community health files and the licence badge by these **exact** names; `<app>_SECURITY.md` is invisible to the security tab and the Private Vulnerability Reporting flow. |
| Tooling configs | `.gitignore`, `.ci-required.txt`, `deny.toml`, `dist-workspace.toml`, `.github/workflows/*.yml` | Read by name by git, cargo-deny, cargo-dist and GitHub Actions. |
| Already-scoped directories | `crates/core/`, `docs/architecture.md` | The directory already answers "which app" (the repository) and "which layer"; repeating it in every file inside it is noise. |

So the rule is **"prefix what leaves the repository or must be globally unique; never prefix what a tool finds by name."** The app prefix identifies build outputs and crates. It does not rewrite the inside of a crate.

### II.28.3 Deviations to check in each repository

Two shapes recur and must be settled explicitly rather than left inconsistent:

- **Version-first artefact names.** An app may name artefacts `<app>-<version>-<target-triple>` (for example `<app>-0.2.0-x86_64-unknown-linux-gnu`) with a versioned checksum file, instead of `<app>-<os>-<arch>` with `SHA256SUMS.txt`. Its update-check code parses those names, so changing them is a code change plus a release change, not a rename. Record the deviation in `DECISIONS.md` and decide it, rather than silently aligning.
- **Architecture spelling.** `arm64` and `aarch64` are the same architecture with two spellings. Pick one across the suite (`aarch64` is the target-triple spelling) when the release names are next touched.

Rebuilding artefacts under this shape is a release-naming decision, not a mechanical rename: the self-update path, the checksum file and any published download URLs change together, and old links must keep working or be redirected.

---

## II.29 Personal data and secrets: the mandatory scan gate

A repository publishes three things at once: its files, its whole history, and its build artefacts. A real person's name, address, email, phone number, account name or secret is not "content" — it is a defect, and it must never be coded in, committed, released, or quoted. This section makes that check mandatory for every agent, before every commit and before every release.

**Owner directive (2026-10-27): every agent runs a full scan, and nothing coded or released may contain this information.** Treat a missed scan as a failed slice, not a style issue.

### II.29.1 What must never appear

Never commit or ship:
- A real person's **name**, **email address**, **phone number**, **postal address**, or company/government identifier.
- A **personal value used as a default**: a hard-coded demo or seed account, a form placeholder, a fallback identity, a test fixture, sample or example data, or a populated commit/issue template.
- The **maintainer's OS account name or absolute home path** — `/home/<os-user>/...`, `C:\Users\<os-user>\...`. The account name is usually the real name. Use `~` or a relative path instead.
- **Secrets**: tokens, keys, passwords, `.env` contents, private URLs, webhook IDs, session cookies.
- Any **quoted copy** of the above in docs, audit notes, `CHANGELOG`, issue or PR text. A document may describe the defect; it must not reproduce the value.

Values that merely look personal are fine, but they must be **obviously synthetic**: `dev@example.com`, `example.com`, `Jane Doe`, `+1 555 0100`, `ACME Demo JV`. Do not "fix" a leak by substituting another plausible real name.

### II.29.2 The scan is mandatory, and "full" has four parts

Before the first commit of a slice, before every push, and before every release:

1. **The whole working tree** — every file type, including dotfiles and dot-directories (`.github/`, `.opencode/`, `.env*`) and lockfiles. A scan that skips dot-directories misses exactly where agent config hides the maintainer's home path.
2. **The whole history, not just `HEAD`.** A value deleted in a later commit is still readable in the earlier one. Use `git log -S"<value>" --all` and `git grep` across `origin/main` and every branch.
3. **The published surface, not just the local clone** — the repo contents, issues and pull requests, release bodies, and every **release asset** (tarball, AppImage, installer, SBOM). Build artefacts can embed absolute build paths: `/home/runner` is fine, a maintainer home path is not.
4. **The metadata** — commit author and committer name/email, tag messages, and the repository description, homepage and topics.

A pattern set that has caught real leaks in this workspace:

```
(?i)/home/[a-z0-9_-]+
(?i)C:\Users\[A-Za-z0-9_.-]+
[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}
(?i)(api[_-]?key|secret|token|password|passwd|bearer)\s*[:=]\s*['"][^'"]+
```

A hit is a finding unless the value is **obviously synthetic** or is **third-party upstream content** (for example a dependency's `LICENSE` carrying its own authors' addresses). Classify every hit; never just count them.

### II.29.3 Wire it into the gates

- Run a **secret scan** (gitleaks or equivalent) and a **personal-data pattern scan** on every pull request, as **required** status checks.
- Run the same scan in the **release job** against the built artefact set, before assets are attached.
- Add both to `.ci-required.txt`, so a red scan blocks a merge.
- Record the scan result in `PROGRESS.md` with the CI run ID, like any other gate (II.24).

A scan that is not a required check is a suggestion, not a gate.

### II.29.4 When a value is already published

Removing it from `HEAD` is step one, never the fix. A value that ever reached a public repository stays readable in history, in forks and in the web UI.

1. **Remove it from the working tree and every current file** (a normal PR). This stops the leak growing.
2. **Redact quoted copies** in docs and audits — keep the finding, delete the value.
3. **Purge history** when the value is a personal identifier or a secret: rewrite the affected branches (`git filter-repo --replace-text`), force-push, and have every clone re-fetch. Branch protection with `enforce_admins` blocks this, so `allow_force_pushes` is temporarily enabled and restored afterwards. That is a deliberate, **owner-approved** action, never a reflex.
4. **Ask the platform** when history cannot be rewritten: GitHub Support can purge cached views, but it does not rewrite the repository.
5. **Rotate anything secret**, regardless of history. A token that was ever pushed is compromised.

Record the incident, the fix and the residual risk in `CHANGELOG.md` or `SECURITY.md` — describing the defect, never repeating the value.

### II.29.5 Assignment of responsibility

- **Every agent**, before committing or pushing: run the scan for its own change and stop if it is not clean.
- **The reviewer**: reject any change that adds a plausible real identifier or a secret, even in a comment, fixture or placeholder.
- **The designer**: no real name, photo, address or personal datum in mockups, sample content or copy.
- **The orchestrator**: run the whole-history and release-artefact parts of the scan before any release is cut (II.24.3), and raise history-purging decisions with the owner.

### II.29.5 Release-asset scan (part of II.24.3)

```
# tree + history
git grep -n -I -i -E '/home/[a-z0-9_-]+|@(gmail|outlook|hotmail|proton|yahoo)\.'
git log --all -S"<personal value>" --oneline
# published artefacts, after download or extraction
grep -rIn -i -E '/home/[a-z0-9_-]+' <extracted-asset-dir>
```

Only `/home/runner` (CI) and obviously synthetic values are acceptable hits.

---

## II.30 Rust GUI/UX — stack matching, tokens and evidence (added 2026-11-09)

### II.30.1 Hard rule — stack matching

| Source | Stack | Use for a native Rust desktop GUI? |
|--------|-------|-----------------------------------|
| Astryx | React 19 + StyleX | **No** as a dependency. Optional **inspiration only** — token names, density vocabulary, a11y wording. |
| shadcn/ui, Material web, any CSS design system | JS/CSS | **No** as code. Patterns only. |
| **egui / eframe / iced / Slint / GPUI** ecosystems | Rust | **Yes** — primary sources. |

Agents must:

1. Detect the actual GUI crate and its **resolved** version from `Cargo.lock` (not the `Cargo.toml` range) and the shell shape before recommending anything; record it in `docs/design/gui-stack.md`.
2. Match every recommendation to that crate; an egui recipe is not advice for an iced app.
3. Never propose adding React, StyleX, Astryx, Tailwind or a CSS framework to a pure Rust app unless the repository already ships a web front end.
4. Never answer "improve the design" with prose alone. A UX claim needs tokens, contrast, keyboard paths or rendered evidence (II.30.8).

### II.30.2 Detect the stack before recommending anything

Read-only commands only — they are allowed on the client machine; `cargo` is not (II.27.10, K.11.9).

```bash
# declared dependencies (workspace, app and crate manifests)
grep -nE '^(eframe|egui|egui_extras|egui-wgpu|iced|slint|gpui|tauri|dioxus|leptos)[[:space:]]*=' \
  Cargo.toml apps/*/Cargo.toml crates/*/Cargo.toml 2>/dev/null
# resolved version — the only version that matters
grep -A2 -E '^name = "(egui|eframe|iced|slint|gpui)"' Cargo.lock
# where the toolkit is actually used
grep -rln 'egui::' --include='*.rs' . | head -20
```

Record in the PR or `docs/design/gui-stack.md`: toolkit, **resolved** version, shell shape (the II.27.12 table — does the UI render into a plain `&mut Ui`?), theme module path, whether an a11y/snapshot harness already exists, and the window/shell layout model (panels, rail, tabs, docking).

The shell shape decides which evidence is possible. Do not promise screenshot tests for an app whose UI cannot render without `eframe::CreationContext` or `eframe::Frame`; II.27.12 makes that a refactor slice first, and a fake harness that silently renders nothing is worse than none.

### II.30.3 Version discipline

- Pin recommendations to the version resolved in `Cargo.lock`, never to the range in `Cargo.toml`. `egui = "0.32"` may resolve anything in that minor line; widget APIs differ.
- **A UX slice never bumps the toolkit.** Dependency moves are their own slice: manifest + lock diff, CI green, rendered evidence re-reviewed, and a `CHANGELOG.md` note. Mixing the two makes a design regression indistinguishable from an API break.
- Prefer crates that document support for the project's egui version. A crate written against egui 0.36 is not "close enough" to 0.32; widget code compiles against exactly one API.
- Adopting a design crate the project's toolkit version has left behind is a recorded risk in `DECISIONS.md`, not an accident discovered at review time.
- Confirm the API shape from current documentation (Context7 or the crate's own docs) before adopting a design crate. A remembered API from an older egui release is a guess.

### II.30.4 egui/eframe resource map

Tags: **adopt dependency** (add it), **copy pattern** (study and reimplement), **process only** (CI or review rule).

**Design systems, theming and components**

| Crate / project | Role | Agent guidance |
|-----------------|------|----------------|
| **shadcn-egui** | shadcn-style presets, palettes, fonts, icons and components on plain egui | Strong visual reference. **Copy pattern** by default; **adopt** only if the brand and licence allow a third-party look. |
| **egui-pop** (+ **egui-pop-tokens**) | DTCG design tokens, visual kits, workbench, commands/keymaps, agent-oriented API | **Copy pattern** for token structure and command/keymap architecture. Its full `App` host is **adopt** only for greenfield or an explicit migration task. |
| **spaceui** | Semantic themes, SVG icons, custom window shell, columns | **Copy pattern** for shell, rail and status-bar structure. |
| **modern-egui** | Tokens plus UI helpers; targets a recent egui minor | **Adopt** when its documented egui version matches the project's; otherwise copy the token helpers. |
| **egui_sauge** | WCAG-AA-oriented palette, density, workbench chrome | **Copy pattern** for a11y-first palette and IDE-like layout. |
| **egui_elegance** (`elegance`) | Opinionated buttons, inputs, tabs, cards | **Adopt** or copy to polish individual widgets; not a whole-app framework. |
| **theme-egui** | Semantic roles → `egui::Visuals` | **Copy pattern** for the token→visuals mapping (II.30.6). |
| **egui_aesthetix** (`elegance`) | Trait-based themes (Adwaita, Nord, …) | Alternative theming approach; do not stack it with another theme crate. |
| **egui_material3** | Material 3 components and themes | **Adopt** only if a Material look is a product decision. |
| **fluent_egui** | Fluent / WinUI-inspired widgets | **Adopt** only if a Fluent look is a product decision. |
| **kylig** | Animation / FX, reduced-motion aware | **Adopt** for motion polish only, and only with the reduced-motion rule in II.30.7.7. |

**Layout and docking**

| Crate | Role | Agent guidance |
|-------|------|----------------|
| **egui_dock** | Tab docking, drag, floating windows | **Adopt** only when user-rearrangeable panels are a product goal. |
| **egui_tiles** (and docking forks) | Flexible tiling and grid layouts | **Adopt** for genuinely tiled workspaces; prefer plain panels for a fixed three-pane shell. |

**Official and testing**

| Resource | Role | Agent guidance |
|----------|------|----------------|
| **egui** + **egui_extras** | Core widgets, tables, image loaders, date picker | First-party baseline; prefer these over a wrapper crate for anything generic. |
| **egui accessibility docs** | AccessKit integration, `labelled_by`, `WidgetInfo`, roles | **Process only** — the implementation rule for II.30.7. |
| **egui_kittest** | AccessKit queries and optional snapshot baselines in CI | **Adopt** where the seam allows; note II.27.12 before promising it. |

**Hard limit.** One theming/design path per app. `shadcn-egui` + `spaceui` + `egui_aesthetix` in one binary is a review blocker, not a rich design.

### II.30.5 Other Rust GUI stacks

| Stack | UX improvement sources | Rule |
|-------|-----------------------|------|
| **iced** | iced's own theme and widget ecosystem; Elm-style message structure | Do not mix egui crates or egui design systems into an iced app. |
| **Slint** | `.slint` components, built-in styling, Slint design tooling | Design in Slint's model; no Rust widget shims for its styling. |
| **GPUI** | Retained-mode GPUI patterns and its own element/model system | No egui design crates; no immediate-mode recipes applied literally. |
| **Tauri / Dioxus / Leptos (web UI)** | Web design systems — Astryx-class kits **do** apply **to the frontend only** | The Rust side stays a backend; the web layer keeps its own token discipline. |

### II.30.6 One token source — semantic roles, not hex literals

A GUI slice starts with tokens, not widgets. One theme module owns every colour, space, radius, elevation and text size; widget code reads roles.

**Placement.** `apps/<app>/src/theme/` or a `crates/<app>-theme` crate with no egui dependency for the palette maths (II.27.10 tests it headless, per K.13). The crate that renders depends on the crate that defines tokens, never the reverse.

**Semantic roles (minimum set).**

| Role | Paints | Requirement |
|------|--------|-------------|
| `surface.base` / `.raised` / `.sunken` | panels, popups, wells | Distinguishable without relying on borders alone. |
| `text.primary` / `.secondary` / `.muted` / `.disabled` | body, labels, hints, disabled | AA against the surface each sits on. |
| `accent` / `accent.hover` / `accent.on` | primary action, selection | `on` text meets AA on `accent`. |
| `danger` / `warning` / `success` (+ `.on`) | validation and status | Never the only state signal (II.30.7.2). |
| `border.subtle` / `.strong` | separators, input outlines | `strong` ≥ 3:1 against adjacent surface. |
| `focus.ring` | keyboard focus | ≥ 3:1 against both the control and the surface. |
| `overlay.scrim` | modal dimming | Keeps dialog text readable at the composited colour. |
| `selection.bg` / `selection.text` | text selection, list highlight | Contrast as text-on-fill. |

**Scales.** Spacing 2-4-8-12-16-24-32-48 (4-point base); radii 2/4/8/12; elevation three levels maximum; type scale one ratio (1.125 or 1.2) with roles `display`, `h1`, `h2`, `body`, `small`, `mono`, mapped once into `Style::text_styles`.

**Mapping to the toolkit.** `Tokens` exposes `fn visuals(&self) -> egui::Visuals` and `fn style(&self) -> egui::Style`, driving `panel_fill`, `window_fill`, `extreme_bg_color`, `faint_bg_color`, `override_text_color`, `selection.bg_fill`, `hyperlink_color`, `warn_fg_color`, `error_fg_color`, the `widgets.*` fill/stroke pairs, `window_shadow`/`popup_shadow`, plus `spacing.item_spacing`, `button_padding` and `window_margin` from the spacing scale.

**Variants.** Light, dark, and high contrast, with system-follow as the default and the user's choice persisted. `Visuals::high_contrast()` may seed the third variant, but it must still pass the II.30.7.1 tests — a seed is not a pass.

**Enforcement (process only, cheap).** A CI grep gate fails the build when a raw colour constructor appears outside the theme module:

```bash
! grep -rn --include='*.rs' -E 'Color32::from_(rgb|rgba|gray|black|white)' \
    apps/*/src crates/*/src | grep -v 'src/theme/'
```

Widget-level one-off colours are the single most common regression in an egui codebase, and the gate catches them without a rendering harness.

### II.30.7 Accessibility and input as code

II.21 sets the commitment; these are the implementation rules.

**1. Contrast is a test, not an opinion.** Keep luminance and contrast maths as pure functions in the token crate and assert the palette:

```text
contrast(a, b) = (L1 + 0.05) / (L2 + 0.05), with L = 0.2126R + 0.7152G + 0.0722B
after sRGB linearisation
```

Required: **4.5:1** for body text, **3:1** for large text (≥ 24 px regular or ≥ 18.66 px bold), **3:1** for meaningful UI boundaries and focus rings, **3:1** minimum for every state colour against its own surface. Assert both themes and the high-contrast variant in one `#[test]`. This runs headless and is a merge gate.

**2. Colour is never the only signal.** Errors carry text and an icon; selection carries a fill **and** a marker; disabled carries reduced emphasis plus a reason.

**3. Keyboard completes every flow.** Tab order follows reading order; Esc dismisses the topmost dialog; Enter activates the default action; focus is never trapped; the focus ring uses `focus.ring` and is visible on every interactive widget. The shortcut table lives in `docs/qc/shortcuts.md` and is updated whenever one is added.

**4. Icon-only controls are named.** A tooltip is not a name. Set the widget's `WidgetInfo`/`labelled_by` text as well, so the AccessKit tree carries the label (II.21.2.2).

**5. Target size.** Interactive targets are at least **24 logical points** on their shortest side; primary actions and destructive actions are at least 32. Density variants (compact / standard / comfortable) change spacing, never below the 24-point floor.

**6. Reflow.** At a narrow window and at 200% zoom no control clips or overlaps; the shell degrades by collapsing the side panel before shrinking the canvas. One panel at a time may shrink; none may become unreachable.

**7. Reduced motion — honest limitation.** egui has no cross-platform system-level reduced-motion query. Provide an in-app preference, default animations **off** when the platform signal is unavailable, keep transitions under ~120 ms and non-blocking, and record the limitation in `ACCESSIBILITY.md` as II.21.1 requires.

### II.30.8 Evidence — rendered, not intended

II.27.12 owns the harness mechanics (the seam refactor, `xvfb`, software Vulkan, `--test-threads=1`, artefacts never committed). This section fixes **what** a UX slice must show.

**Minimum evidence per UX slice.** Shell chrome in light and dark, one primary panel in its empty state, one error state, one dialog, and the narrow-window layout — captured from the built app and reviewed by the designer. Artefacts go to the PR description or the review record; they are never committed.

**What `egui_kittest` can gate where a harness exists.** AccessKit assertions are real tests, not screenshots: the named control exists with the right role, the dialog dismisses on Esc, focus moves where the flow says it should, the empty state renders its call to action. Snapshot PNGs are for the eye; the assertions are the gate.

**What it cannot.** Visual taste, spacing rhythm, "feels professional", and anything behind a seam the app has not refactored. Say so plainly rather than implying the render was checked.

**No pixel baselines.** A diff that fails on a font-metric change teaches the team to ignore the gate.

### II.30.9 Workflows the agent runs

| Workflow | Outcome | Gate |
|----------|---------|------|
| Token → toolkit visuals | One semantic token table drives light, dark and high contrast | Contrast test green |
| Component recipe sheet | Primary / secondary / ghost / danger plus sizes documented and reused | Review: no ad-hoc buttons |
| Empty / error / loading states | Every primary panel has a real, non-dead state | Rendered evidence |
| Keyboard and focus pass | Shortcuts listed, focus visible, nothing trapped, icon controls named | AccessKit test or manual QC log |
| Contrast CI | Unit tests for text and UI contrast | Merge gate |
| Snapshot / kittest | Shell and critical dialogs baselined | Artefact reviewed |
| Dependency slice | Toolkit and design-helper bumps isolated | CI green, evidence re-reviewed |

### II.30.10 Component recipe sheet

Four button roles and three sizes, no more without a written reason:

| Role | Use | Rules |
|------|-----|-------|
| **Primary** | The one action a view exists for | At most one per view; accent fill; `accent.on` text. |
| **Secondary** | Supporting actions | Outline or subtle fill; never competes with primary. |
| **Ghost** | Toolbar and inline actions | No container until hover; still ≥ 24 pt target. |
| **Danger** | Destructive, irreversible | Separated from the primary cluster; confirmation names the consequence. |

States every recipe specifies: default, hover, active, focus, disabled, loading. Loading keeps the control's size stable so the layout does not jump. Disabled explains why on hover/focus and is never the only route to a task.

### II.30.11 States every primary panel owns

| State | Requirement | Copy rule |
|-------|-------------|-----------|
| Empty | Says what the panel is for and offers the first action | What this is, why it is empty, what to do next — never "No data". |
| Loading | Skeleton or spinner; no layout shift on completion | No fake content; no spinner shorter than 400 ms of perceived work. |
| Error | Names what failed, keeps the user's input, offers retry | What happened, what to do next, no blame, no error code alone. |
| Disabled | Explains the precondition | Reason plus the step that enables it. |
| Permission denied | Says who can grant access | No dead end. |
| Long content | Scrolls or truncates with a way to see all | Truncation never hides the only copy of a value. |
| Narrow window | Collapses predictably | The canvas shrinks last; nothing becomes unreachable. |
| High contrast / reduced motion | Renders and animates per II.30.6 and II.30.7.7 | No state conveyed by motion alone. |

### II.30.12 Decision tree

```text
Is the UI web (Tauri / Dioxus / Leptos frontend)?
  YES -> Web design systems, including Astryx-class kits, apply to the frontend.
  NO  -> Native Rust GUI path.
        Is the toolkit egui?
          YES -> Token discipline + modern-egui / shadcn-egui / sauge / elegance
                 patterns; egui_dock or egui_tiles only if docking is a goal;
                 egui_kittest + AccessKit for evidence.
          NO  -> Use that toolkit's official theming and widget ecosystem only.
        Is the task "polish the existing app"?
          YES -> Selective widgets + tokens + a11y + split oversized UI modules.
          NO  -> Greenfield: a fuller kit (egui-pop, spaceui, sauge workbench) may
                 be evaluated against brand and maintenance cost.
```

### II.30.13 Deliverables for a GUI/UX request

Every GUI/UX answer contains, in this order:

1. **Stack detection** — crate and resolved version, shell shape.
2. **Gap list** — tokens, contrast, keyboard, empty states, density, consistency, module size.
3. **Recommendations**, each tagged **adopt dependency** | **copy pattern only** | **process/CI only**.
4. **Non-goals** — what this slice will not do (II.30.14).
5. **Implementation slices** — small enough for one PR each, with the test and a11y note per slice.
6. **Evidence plan** — which of II.30.8's artefacts or assertions will prove the slice.

### II.30.14 Non-goals

- Astryx, React, StyleX or any CSS design system inside a native Rust GUI.
- Two competing egui design systems in one binary.
- Rebuilding the app shell around a framework without an explicit migration task and client approval.
- Copying the inspiration product's pixels, icons, fonts, string tables or presets (II.27.2).
- An emoji set standing in for an icon system; icons are original SVG under `assets/`, licensed and listed in `ATTRIBUTION.md`.
- A toolkit or design-crate version bump hidden inside a UX slice.
- "Design improved" claimed without contrast, keyboard or rendered evidence.
- Any heavy build, test or `cargo` command on the client machine (II.27.10).

### II.30.15 Definition of done for a UX slice (adds to II.24.1)

- [ ] Stack and resolved version recorded; no crate outside its supported egui version.
- [ ] Every colour, space, radius and text size comes from the token module; the raw-colour grep gate passes.
- [ ] Contrast tests assert light, dark and high contrast palettes (4.5:1 text, 3:1 large text and UI boundaries).
- [ ] Every primary panel has empty, loading and error states with real copy.
- [ ] Keyboard completes the flow; focus is visible; icon-only controls carry an AccessKit label; shortcuts documented.
- [ ] Targets ≥ 24 pt; narrow window and 200% zoom do not clip a control.
- [ ] Rendered evidence captured and reviewed (II.30.8); AccessKit assertions added where a harness exists.
- [ ] Adopted crates' licences recorded in `ATTRIBUTION.md` and allowed by `cargo deny`.
- [ ] `CHANGELOG.md`, `PROGRESS.md` and the PR template's design section updated with the evidence, not just the claim.

### II.30.16 Licence and clean-room hygiene for GUI crates

Every adopted crate passes the dependency hygiene of II.20.4 and M.4: SPDX checked, licence compatible with the project, recorded in `ATTRIBUTION.md`, and allowed by `cargo deny`. Design galleries and demo apps are **study material**: copy the interaction pattern, reimplement the look, and never lift icons, fonts, copy or exact layouts. A bundled font needs its licence in `ATTRIBUTION.md`; the default egui font set is sufficient unless the brand owns a licensed face.

### II.30.17 Quick card (context-scarce models)

1. Detect the toolkit and its `Cargo.lock` version before recommending.
2. Match the stack: no React/CSS design system in a native Rust GUI.
3. One design path per app; no multi-system mashup.
4. Tokens first: semantic roles, one theme module, no scattered hex.
5. Contrast is a unit test (4.5:1 text, 3:1 UI), not an opinion.
6. Every panel has empty, loading and error states with real copy.
7. Keyboard completes the flow; icon-only controls are named.
8. Evidence is rendered or asserted, never asserted from prose.
9. Toolkit bumps are their own slice; UX slices do not move versions.
10. No `cargo` locally — verify with `ci.sh` and CI artefacts.

---

## 19. Claude Code / Codex Skills (Top 10 by GitHub Stars, early October 2026)

The following skills are ranked by GitHub star count from the brochbuilds.com/skills catalog (snapshot: ~early October 2026). Star counts have grown since then; the install commands use the `npx skills add` CLI pattern compatible with Claude Code and Claude Code-based agents.

### Top 10 Skills (ranked by stars)

| Rank | Skill | Stars (approx.) | Description | Install |
|------|-------|-----------------|-------------|---------|
| 1 | **obra/superpowers** | ~295k | Plans, writes tests, and debugs before writing code. The most-starred skills framework. | `npx skills add obra/superpowers -a claude-code` |
| 2 | **mattpocock/skills** | ~275k | Small, composable engineering skills straight from Matt Pocock's own setup. | `npx skills add mattpocock/skills -a claude-code` |
| 3 | **multica-ai/andrej-karpathy-skills** | ~217k | One CLAUDE.md derived from Karpathy's notes that improves how Claude writes code. | `npx skills add multica-ai/andrej-karpathy-skills -a claude-code` |
| 4 | **anthropics/skills** | ~180k | Anthropic's official set (docs, PDFs, spreadsheets, slides, design, skill-creator, etc.). | `npx skills add anthropics/skills -a claude-code` |
| 5 | **DietrichGebert/ponytail** | ~154k | Makes the agent code like the "laziest senior dev": less code, fewer bugs. | `npx skills add DietrichGebert/ponytail -a claude-code` |
| 6 | **nextlevelbuilder/ui-ux-pro-max-skill** | ~133k | Design intelligence (styles, palettes, UX rules) so UI stops looking AI-generated. | `npx skills add nextlevelbuilder/ui-ux-pro-max-skill -a claude-code` |
| 7 | **Graphify-Labs/graphify** | ~124k | Turns a codebase + docs + schemas into a queryable knowledge graph. | `npx skills add Graphify-Labs/graphify -a claude-code` |
| 8 | **JuliusBrussee/caveman** | ~110k | Aggressively cuts output tokens by making the agent reply more tersely ("caveman" style). | `npx skills add JuliusBrussee/caveman -a claude-code` |
| 9 | **addyosmani/agent-skills** | ~101k | Production engineering workflows from Addy Osmani (spec → build → test → ship). | `npx skills add addyosmani/agent-skills -a claude-code` |
| 10 | **Leonxlnx/taste-skill** | ~116k | Gives coding agents real design taste / art direction + reference images + browser check so UI doesn't look generic. | `npx skills add Leonxlnx/taste-skill -a claude-code` (or as a Claude Code plugin: `/plugin marketplace add Leonxlnx/taste-skill` then `/plugin install taste-skill@taste-skill`) |

### Remaining Skills from the Full 320-List

The full page lists all 320 skill repos (with install commands, categories, and search). Star counts on the page are a snapshot from ~October 3, 2026 — many of these have grown further since then. Agents should discover and add skills per-project needs via the `npx skills explore` CLI or the brochbuilds.com catalog.

### Skill Integration Patterns (from guide.md practice)

- **Skill selection** follows the guide's agent role routing: supervisor assigns skills per task, never heavy local builds.
- Skills are discovered via free model browsing (OpenCode → NVIDIA fallback) and MCP/Catalog lookups.
- Installed skills are referenced in task briefs and their capabilities are documented in `DECISIONS.md` per the guide's dependency hygiene rules (II.29.6, M.4).
- **No skill is installed that requires `cargo` or resource-intensive builds on the client machine** (per the HARD BUILD RULE, §4 item 5).
- **Skills that conflict with clean-room rules (reverse engineering, proprietary material, decompiled outputs) are prohibited per §2.2 and the M.4 demolition findings.**

---

## 20. Dead code prohibition (guide hard rule)

**No agent shall fix an issue by creating new code or leaving behind dead code** — every write must be planned, tested for integration, and verified to work properly with every other piece of code. There must be no dead, unfinished, or test code left in the codebase, and no fallbacks that are not fully implemented and documented. All code contributions must pass CI, satisfy the dependency hygiene gates, and be verified as working end-to-end before merge.

Concretely, every write must:

- Be planned as part of the slice's acceptance criteria — no speculative scaffolding "for later".
- Be integrated: reachable from the entry points the slice claims to change, not an orphan module, unused export, or dead route.
- Carry its test in the same slice, and the test must exercise the integration, not merely the unit.
- Leave the codebase with no leftover debug stubs, commented-out blocks, unused imports, disabled checks, or "temporary" fallbacks that are not fully implemented and documented.
- Pass CI, satisfy the dependency hygiene gates (M.4), and be verified as working end-to-end before merge — a green lint on an unintegrated file is not "working".

This rule binds every role (orchestrator, domain-dev, ui-dev, fixer, test-writer, docs-writer): a fix that ships dead code is an incomplete fix, and a review that accepts dead code is a failed review.

---

## 21. Design Guidelines (design-guidelines.md)

This section transcribes `~/design-guidelines.md` — the canonical source for visual and interaction design rules used by agents on guide.md-governed projects — in complete detail per guide §12.3. It contains two complementary checklists: **anti-patterns** (signals that make an interface look generic or AI-generated — "vibecoded"; avoid them) and **high-quality patterns** (concrete techniques that raise perceived quality, responsiveness, and trust; prefer them). Apply the positive patterns first, then run the anti-pattern list as a rejection filter before shipping UI.

---

### 21.1 Design Anti-Patterns — "Vibecoded" Signals & Intentional Alternatives

The following 30 items are common visual, typographic, and layout signals that indicate AI-generated ("vibecoded") design. When present, they signal the designer relied on default templates rather than intentional design decisions. Use this as a hardening checklist — avoid all 30, or deliberately subvert them with purpose.

#### 21.1.1 Harsh gradients

**Why it's bad:** High-contrast, multi-stop gradients (especially purple-to-blue or neon) are the #1 visual fingerprint of AI-generated sites. They feel artificial and overused.  
**Better:** Soft, single-direction gradients with low contrast, or solid colors with subtle texture/noise. Prefer flat or very restrained color transitions.

#### 21.1.2 Lucide icons

**Why it's bad:** Lucide became the default icon set for shadcn/Tailwind projects. Using it unchanged makes your UI look identical to thousands of other AI sites.  
**Better:** Choose a distinctive icon set (Phosphor, Tabler, Heroicons, Remix, or custom icons). Or mix styles intentionally.

#### 21.1.3 Pure white background

**Why it's bad:** Stark `#FFFFFF` with no warmth feels clinical and template-like.  
**Better:** Slightly off-white, warm cream, cool gray, or very dark backgrounds. Give the page some temperature.

#### 21.1.4 Rainbow coloring

**Why it's bad:** Random multi-color accents with no hierarchy look chaotic and unprofessional.  
**Better:** Strict 1–2 accent colors maximum. Everything else should be neutral.

#### 21.1.5 Drop shadows

**Why it's bad:** Soft, diffuse, multi-layer shadows on every card are a classic AI default.  
**Better:** Very subtle shadows, hard shadows, or no shadows at all (use borders or background contrast instead).

#### 21.1.6 3 feature cards in a row

**Why it's bad:** The most overused layout pattern in AI landing pages. Instantly recognizable.  
**Better:** Asymmetric layouts, 2-column, 4-column, zig-zag, or editorial-style feature sections.

#### 21.1.7 Emojis

**Why it's bad:** Emojis as icons or in headings feel playful but lazy and unprofessional in product UI.  
**Better:** Real icons or custom illustrations. Use emojis sparingly (if at all) for tone, never as primary UI icons.

#### 21.1.8 Liquid glass

**Why it's bad:** Heavy glassmorphism (blur + transparency + border) became the new purple gradient. It ages quickly.  
**Better:** Solid surfaces with clear hierarchy, or very light glass used only on specific elements (e.g. floating nav).

#### 21.1.9 Em dashes

**Why it's bad:** AI models overuse em dashes (—) in marketing copy. It's a subtle but strong writing tell.  
**Better:** Use commas, periods, or rephrase. Prefer shorter, punchier sentences.

#### 21.1.10 Inter / Geist / Space Grotesk

**Why it's bad:** These three fonts dominate AI-generated sites. Using them as the main typeface makes your site blend in.  
**Better:** Pair a distinctive display font with a clean body font. Or use a single high-quality typeface with strong personality.

#### 21.1.11 Colored left stripe

**Why it's bad:** Thin colored vertical bars on the left of cards/sections is a very common AI decorative habit.  
**Better:** Use background color, borders, or spacing for hierarchy instead of decorative stripes.

#### 21.1.12 Fake testimonials

**Why it's bad:** Invented names + generic praise + stock avatars destroy trust the moment someone notices.  
**Better:** Real testimonials only, or none. Use logos of real companies, usage metrics, or case studies instead.

#### 21.1.13 Bento grids

**Why it's bad:** Once distinctive, now heavily overused in AI designs.  
**Better:** Use them only if they genuinely serve the content. Prefer more classic or asymmetric layouts when possible.

#### 21.1.14 Terminal window

**Why it's bad:** Decorative terminal/code windows are a cliché for "developer tool" landing pages.  
**Better:** Show the real product UI, actual screenshots, or a clean interactive demo instead.

#### 21.1.15 "It's not X, it's Y"

**Why it's bad:** Extremely common AI copy pattern ("It's not just a tool, it's a…") that feels formulaic.  
**Better:** Be direct. State what the product actually does without the rhetorical contrast.

#### 21.1.16 Checkmark bullets

**Why it's bad:** Green checkmarks in circles are the default "feature list" treatment in AI output.  
**Better:** Custom icons, numbers, simple dashes, or just strong typography hierarchy.

#### 21.1.17 3 pricing tiers

**Why it's bad:** The classic Free / Pro / Enterprise (or Starter / Growth / Scale) layout is the most templated pricing section.  
**Better:** 2 tiers, usage-based, or a single clear plan with add-ons. Make the pricing structure match how you actually charge.

#### 21.1.18 No real product demos

**Why it's bad:** Sites that only show abstract illustrations or generic mockups feel incomplete.  
**Better:** Real screenshots, short product videos, interactive demos, or Loom-style walkthroughs.

#### 21.1.19 Soft corner radius

**Why it's bad:** Uniform large border-radius on every element creates a soft, indistinct "AI aesthetic."  
**Better:** Mix radii intentionally (sharp + soft), or go fully sharp for a more editorial feel.

#### 21.1.20 Purple and black

**Why it's bad:** The dominant AI color combination for years. Extremely recognizable.  
**Better:** Choose a unique primary color that fits the brand. Avoid defaulting to purple/violet/indigo.

#### 21.1.21 No skeleton loaders

**Why it's bad:** Missing loading states make the app feel unfinished or janky.  
**Better:** Always include proper skeleton screens or subtle loading indicators for async content.

#### 21.1.22 Radial orbs

**Why it's bad:** Glowing circular blurs in the background are a classic decorative AI filler.  
**Better:** Subtle noise, grain, geometric shapes, or just clean solid backgrounds.

#### 21.1.23 Dot grids

**Why it's bad:** Dot grid patterns as backgrounds became a very common "modern" AI decoration.  
**Better:** Minimal backgrounds. Use texture sparingly and purposefully.

#### 21.1.24 Sparkle icons

**Why it's bad:** Sparkles / stars / ✨ used as decorative icons scream AI-generated.  
**Better:** Meaningful icons only. Avoid decorative sparkles entirely.

#### 21.1.25 Animated arrows

**Why it's bad:** Looping or bouncing arrows are a cheap way to add motion that quickly feels gimmicky.  
**Better:** Purposeful micro-interactions. Motion should support usability, not just look "alive."

#### 21.1.26 No TOS

**Why it's bad:** Missing legal pages signals the site was thrown together quickly.  
**Better:** Always include Terms of Service (even a simple one).

#### 21.1.27 No privacy policy

**Why it's bad:** Same as above — looks incomplete and can create real legal risk.  
**Better:** Always ship a Privacy Policy.

#### 21.1.28 Hover animations

**Why it's bad:** Adding hover effects to every element creates visual noise and feels unrefined.  
**Better:** Selective, purposeful hover states only on interactive elements that benefit from them.

#### 21.1.29 Neon colors

**Why it's bad:** High-saturation neon accents feel dated and "trying too hard."  
**Better:** Muted or mid-saturation accents. Save high intensity for rare, intentional moments.

#### 21.1.30 Basic pastel colors

**Why it's bad:** Soft pastel palettes (especially the common blue/pink/mint/yellow card sets) are heavily overused in AI designs.  
**Better:** More intentional, slightly desaturated, or higher-contrast color systems with personality.

**Pro tip:** Treat the list above as hard constraints. When in doubt, choose the less common, more intentional option.

---

### 21.2 High-Quality UI Patterns — 20 Practical Tips

These 20 patterns come from high-quality products (Apple, Linear, Gmail, Stripe, Amazon, etc.). They improve perceived quality, responsiveness, and trust.

**Applicability note (important for this suite):**  
The default technology stack in `guide.md` is pure Rust + egui (native desktop). Some patterns below are native-friendly; others are primarily web-oriented. Prefer the native-applicable patterns first. When the project has a web front-end (Tauri, Dioxus, Leptos, etc.), the full set applies.

| Pattern | Native (egui) fit | Web fit |
|---------|-------------------|---------|
| Skeleton loaders, Optimistic updates, Command menu, Shortcut hints, Undo toasts, Interruptible animations, Hit slop, Tabular numbers, Continuous corners, Idempotency keys | Excellent | Excellent |
| Stale-while-revalidate, Safe area insets, Balanced headlines | Good | Excellent |
| Pre-rendering, BlurHash, View transitions, Swipe-to-dismiss, Predictive back, One-time code autofill, Amazon safe triangle | Limited / N/A | Excellent |

#### 21.2.1 Skeleton loaders

**Why it matters:** Blank screens or spinners feel broken. Skeletons maintain layout and set expectations while content loads.  
**Better approach:** Match the exact shape and size of the real content. Use subtle shimmer, not aggressive pulsing. Prefer content-aware skeletons over generic gray boxes.

#### 21.2.2 Optimistic updates

**Why it matters:** Waiting for the server makes the app feel slow. Optimistic updates make actions feel instant.  
**Better approach:** Update the UI immediately, then reconcile with the server. Always have a clear rollback path if the request fails (and show an undo or error state).

#### 21.2.3 Command menu

**Why it matters:** Power users want to do everything from the keyboard. A command palette (⌘K / Ctrl+K) dramatically improves speed and discoverability.  
**Better approach:** Make it searchable, support fuzzy matching, show keyboard shortcuts next to actions, and include recent/frequent items at the top.

#### 21.2.4 Shortcut hints

**Why it matters:** Hidden keyboard shortcuts are useless. Showing them teaches users and increases adoption.  
**Better approach:** Display shortcuts in tooltips, menus, and the command palette. Use consistent formatting (e.g. `⌘⇧P`). Don't overwhelm beginners — reveal them progressively.

#### 21.2.5 Undo toasts

**Why it matters:** Destructive or hard-to-reverse actions feel scary. An undo toast gives users a safety net and reduces anxiety.  
**Better approach:** Show a temporary toast with an Undo button after delete/archive/move actions. Keep it visible long enough (5–8 seconds) and make the undo reliable.

#### 21.2.6 Stale-while-revalidate

**Why it matters:** Showing nothing while refreshing data creates flicker and feels slow. Showing slightly stale data while fetching fresh data feels much smoother.  
**Better approach:** Cache responses, show the cached version immediately, then silently update when fresh data arrives. Great for lists, dashboards, and profile data.

#### 21.2.7 Pre-rendering

**Why it matters:** Waiting for JavaScript to hydrate creates a blank or non-interactive page. Pre-rendering (SSR/SSG) gets content in front of users faster.  
**Better approach:** Pre-render critical pages and above-the-fold content. Use progressive enhancement so the page is usable even before full hydration. *(Primarily web.)*

#### 21.2.8 BlurHash placeholders

**Why it matters:** Empty image boxes or generic spinners look cheap. BlurHash (or similar) gives a beautiful, lightweight preview while the real image loads.  
**Better approach:** Generate a tiny BlurHash string on the server and render it as a blurred placeholder. It should closely match the final image's colors and composition. *(Primarily web / image-heavy UIs.)*

#### 21.2.9 View transitions

**Why it matters:** Instant cuts between pages feel abrupt. Smooth transitions make the app feel cohesive and polished.  
**Better approach:** Use the View Transitions API (or equivalent) for shared element transitions and page changes. Keep them short and purposeful — don't animate everything. *(Primarily web.)*

#### 21.2.10 Swipe-to-dismiss sheets

**Why it matters:** Bottom sheets and modals that can only be closed with a tiny X feel restrictive. Swipe-to-dismiss matches native mobile expectations.  
**Better approach:** Support swipe-down to dismiss on mobile sheets. Add a subtle drag indicator and make the gesture feel physical (with velocity and rubber-banding).

#### 21.2.11 Interruptible animations

**Why it matters:** Animations that can't be interrupted feel laggy and unresponsive when the user tries to interact during them.  
**Better approach:** Make animations cancelable. If the user clicks or navigates while something is animating, stop or reverse the animation immediately.

#### 21.2.12 Predictive back

**Why it matters:** On Android (and increasingly elsewhere), users expect a preview of where the back gesture will take them.  
**Better approach:** Implement predictive back animations that show the previous screen peeking in as the user swipes. It reduces navigation anxiety. *(Mobile / platform-specific.)*

#### 21.2.13 Hit slop

**Why it matters:** Small touch targets are frustrating and inaccessible. Extra invisible padding around tappable elements makes them much easier to hit.  
**Better approach:** Expand the touch target beyond the visible element (especially for icons and small buttons). Aim for at least 44–48pt effective target size.

#### 21.2.14 Safe area insets

**Why it matters:** Content hidden under notches, Dynamic Island, home indicators, or rounded corners looks broken.  
**Better approach:** Always respect safe area insets on mobile. Use `env(safe-area-inset-*)` or the platform equivalents so content never collides with system UI.

#### 21.2.15 One-time code autofill

**Why it matters:** Making users manually type 6-digit codes from SMS/email is friction. Autofill is expected in modern apps.  
**Better approach:** Use the correct autocomplete attributes (`one-time-code`) and support platform autofill (iOS, Android, password managers). Handle pasting cleanly too. *(Primarily web / mobile web.)*

#### 21.2.16 Tabular numbers

**Why it matters:** When numbers in a column have different widths (1 vs 7 vs 0), the layout shifts and becomes hard to scan.  
**Better approach:** Use `font-variant-numeric: tabular-nums` (or the equivalent) for any data tables, prices, stats, or timestamps so digits align vertically.

#### 21.2.17 Balanced headlines

**Why it matters:** Headlines that leave a single short word on the last line ("orphans") look unfinished and weaker.  
**Better approach:** Use text balancing (`text-wrap: balance` or manual tweaks) so multi-line headings break more evenly and look intentional.

#### 21.2.18 Continuous corners

**Why it matters:** Regular border-radius creates a slight flattening at the corners. Continuous (squircles) corners feel more premium and closer to high-end design systems.  
**Better approach:** Use continuous corner curves where appropriate (especially on cards, buttons, and avatars) instead of simple `border-radius`.

#### 21.2.19 Idempotency keys

**Why it matters:** Double-submits (from double-clicks or flaky networks) can create duplicate charges, messages, or records.  
**Better approach:** Send a unique idempotency key with mutation requests so the server can safely ignore duplicates. Essential for payments, form submissions, and any critical action.

#### 21.2.20 Amazon safe triangle

**Why it matters:** Dropdowns and flyout menus that disappear the moment the cursor leaves the trigger are frustrating. Users often take a diagonal path.  
**Better approach:** Implement the "safe triangle" (or safe area) technique: keep the menu open while the pointer is moving toward it, even if it briefly leaves the trigger. Popularized by Amazon's mega-menus. *(Primarily pointer / web.)*

---

**Agent guidance**

1. Prefer the patterns in Section 2 that fit the current stack (native egui first; web patterns apply only when the project has a Tauri/Dioxus/Leptos frontend).
2. After implementing UI, run the anti-pattern checklist in Section 21.1 as a rejection filter.
3. When in doubt, choose the less common, more intentional option.

---

## 22. pstack — Rigorous Engineering Skills for Agents

**https://github.com/cursor/plugins/tree/main/pstack** (by Lauren Tan / @poteto, Cursor).

Core idea: *If you want to go fast, go deep first. Write less code of higher quality.* Turns the agent into something closer to a real engineering team: understand the existing system, choose a design, verify the result on the real app, review the diff, and only then report success.

- **`/poteto-mode`** (or `/pstack:poteto-mode` in Claude Code) — main entry point.
- 20+ playbooks: investigation, bug fix, performance, feature, refactoring, prototype, visual parity, shipping, overnight runs.
- Verification, unslop, multi-model review.

**Installation — Cursor:** `/add-plugin pstack`, then `/setup-pstack`. **Claude Code:** `/plugin marketplace add cursor/plugins` then `/plugin install pstack@cursor-plugins`.

**How to use with this guide:**

1. Prefer poteto-mode (or equivalent rigor mode) for substantial tasks.
2. pstack's verification and "prove it on the real app" posture is complementary to this guide's clean-room, never-crash, and CI-only rules.
3. Do not install or run anything that violates §4 item 5 (no local `cargo`).
5. When pstack and this guide conflict, the stricter rule wins — especially clean-room, secrets, and legal boundaries. pstack skills that require a local `cargo` invocation are prohibited by §4 item 5.

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

---

## 24. Token Metrics Schema

`docs/tokens.md` schema (per release):

```json
{
  "release": "0.4.0",
  "date": "2026-11-16",
  "total_tokens_since_start": 1234567,
  "release_tokens": 45678,
  "release_version": "0.4.0"
}
```

Release notes must reference the ledger values. CI updates the ledger on each semver tag and commits it.

---

## 25. Agent-skill libraries (curated catalogs)

Two of the §25 catalogs are **directories of installable agent skills**, not single skills. They are discovery surfaces, not endorsement:

- **shubhamsaboo/awesome-llm-apps** (§25.4) — 100+ agents, RAG apps, and installable agent skills. Each skill ships real code, passes a security + eval CI gate, and installs with `npx skills add <repo>/tree/main/<path>`. Use it to discover patterns (Project Graveyard, First Reader, Scope Creep Detector, Commit Archaeologist, Dependency Doctor, Advisor Orchestrator Worker) — then apply §19's "installed skills" rules (licence in `ATTRIBUTION.md`, capability + version in `DECISIONS.md`).
- **punkpeye/awesome-mcp-servers** (§25.2) — 4,000+ MCP servers. Not a skill library, but the MCP-server layer that many skills depend on. Use it to find servers your agent can connect to; vet every server against §II.20.4 before adoption.

### Skill Integration Patterns (from guide.md practice)

- **Skill selection** follows the guide's agent role routing; supervisor assigns skills per task, never heavy local builds.
- Skills are discovered via free model browsing (OpenCode → NVIDIA fallback) and MCP/Catalog lookups.
- Installed skills are referenced in task briefs and their capabilities are documented in `DECISIONS.md` per the guide's dependency hygiene rules (II.29.6, M.4).
- **No skill is installed that requires `cargo` or resource-intensive builds on the client machine** (per the HARD BUILD RULE, §4 item 5).
- **Skills that conflict with clean-room rules (reverse engineering, proprietary material, decompiled outputs) are prohibited per §2.2 and the M.4 demolition findings.**

---

## 26. Priority rules card (context-scarce models)

When context is full, obey this card over long prose:

1. Client is non-developer — plain language, numbered try-steps.
2. Never push guide.md / addendum.md.
3. No RE / no proprietary materials / research = summaries + URLs.
4. No v0.1.0 → only §6.1 + Rust M.2.
5. Rust: testable *-core; headless `cargo test` + clippy in CI; rust-toolchain.toml; no GUI in core tests; no heavy local builds on the client laptop.
6. CI green → then ask client to try.
7. BROKEN → regression test → fix.
8. Process friction → Section 16 feedback; do not waive safety.
9. Honest release notes; no Craft-parity claims without evidence.
10. Rust GUI/UX → II.30: match the stack (no React/CSS design system in a native app), tokens + contrast test + keyboard + empty/error states, and rendered evidence; never a design claim without it.

---

## Appendix: Version & Sync Record

- **Guide version:** 2026-11-16 (read in full)
- **Sections read:** Part I (0–18), Part II (II.18–II.30), Appendix A–O, and §20–25
- **Last sync:** 2026-11-16
- **Next sync required:** When guide.md is updated
- **Local AGENTS.md last updated:** 2026-11-16