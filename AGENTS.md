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

## 3. Technology Stack (Decided During Discovery) (guide §3)

Defaults when the human has no strong preference:
- **Language:** Pure Rust when suitable
- **GUI:** egui + eframe + wgpu, or iced
- **Platform priority:** (1) human’s local OS first (2) all practical platforms via CI
- **Persistence:** SQLite or lightweight local store
- **Architecture:** Command-centric
- **Suite shared crates:**
  - `crash-report` — panic hook, diagnostics, optional GitHub Issue create
  - `update-check` — GitHub Releases API, version compare
- **CI:** Multi-OS builds, tests, release artefacts on tag

## 6. Progressive Releases & Roadmap (guide §6)

### v0.1.0 — First Functional Test Build (Hard Gate)
**v0.1.0 is not “hello world.”** It is the first *useful* installable build:
- Installable / runnable: Binary or package for **primary local OS** via CI artefact or local run
- One real workflow: At least one end-to-end user task with **real** data model + persistence (not mock UI)
- App identity: Original name, window title, basic theme
- Crash stub: Panic hook logs or shows a minimal dialog
- Update-check stub: Reads current version; optional “check later” placeholder OK if API wired in 0.2.x
- CI green: Format, lint, tests, primary OS build
- Docs: README with build/run instructions; CHANGELOG entry for 0.1.0
- Legal baseline: LICENSE, ATTRIBUTION started, no secrets, no proprietary assets
- Tag: `v0.1.0` published as GitHub Release with short honest notes

After v0.1.0:
1. Expand `ROADMAP.md`: stage labels, feature breadth %, ready-for-work %, semver milestones
2. Public research notes (`docs/research/`) with **human gate** before large Tasks
3. Frequent semver releases (`0.1.1`, `0.2.0`, …) with **detailed** GitHub Release notes
4. Update readiness % only from evidence (CI + local verification)

### Release Notes Standard
Each GitHub Release body should include:
- Summary (1–3 sentences, honest)
- Added / Changed / Fixed
- Known limitations
- Roadmap progress (links or % if useful)
- Artefact list + checksums when available

## 7. Suite Infrastructure — Crash Reporting & Update Check (guide §7)

### Crash Detection & Bug Reporting
- Panic hook / error boundary: app version, OS, backtrace (scrub secrets/paths), optional user notes.
- UI preview before send; user can cancel.
- **GitHub Issues:** optional user token or device flow to create issues.
- Fallback: pre-filled New Issue URL if no token.
- **Never** embed a privileged bot token in client binaries.
- Document privacy in README.

### Update Check — Standard Flow
Every suite app embeds the shared `update-check` crate and drives **one Updater instance per app**. The app calls `tick()` / `show()` each frame and acts on the returned action.
- **Quiet startup check:** once per run; failure logged, never shown as modal; app starts normally.
- **Explicit check (Help ▸ Check for Updates):** reports Checking / Up to date / Update available / Failed (with error).
- Fail soft offline; conditional GETs; handle GitHub rate limit gracefully.
- **Startup prompt modal:** app icon + accent colour, new version, release notes, exactly three actions:
  - **Update & Restart** — verify, install, then restart through app’s normal close path.
  - **Download & Install** — discover the platform-specific installer from the release assets, download to temp, verify SHA-256 against `SHA256SUMS.txt` and the minisign signature, then offer to open the installer. Never auto-installs; the user always confirms. Gated on a verified signature (never offered on Failed or Missing signature state).
  - **Download** — open the release page; only safe https web links are opened (an unsafe or malformed link falls back to the app’s releases page, which must always be a safe link).
  - **Later** — dismiss; installation already in progress continues and reports through the updater.
- **Persistent install requirement.** Every installed release must create a permanent app-drawer / start-menu entry so the app remains visible after installation: a `.desktop` file on Linux, the `.app` bundle in `/Applications` on macOS, and a Start Menu entry on Windows. A raw extracted binary that runs only from `Downloads` does not qualify as installed.
- **Stale version remediation (before first launch / on update).** Upon detecting a newer release and before activating the staged update, the app must scan the app-drawer / start-menu directory for any previously installed versions of the same app and remove them completely. This ensures that:
  - Only the current latest version appears in the app drawer / start menu.
  - Old versions do not consume disk space or confuse the user.
  - The update process is seamless: the user sees one entry, which is always the current version.
  - This remediation step runs automatically — the user is never asked to manually remove old versions, nor directed to explore system folders or use package manager commands.
  - If an old version cannot be removed (e.g. file-in-use, permission issue), the app logs the issue and continues with the new installation; the user is not interrupted or required to take any action.
- **Self-update applicability (install-type detection).** Detect how the running copy was installed: macOS `.app`, Windows MSI / portable zip / standalone, Linux AppImage / deb / rpm / pacman / Flatpak / standalone. **Development builds never self-update.** A `cargo run`/build-tree copy (Cargo cache markers) gets the download link instead. **Package-manager-owned copies defer to their package manager** (Homebrew, MacPorts, Nix, scoop, winget, Chocolatey, Store, AUR) — in-place replacement would leave their records stale. Only a packaged copy in a supported format self-updates; anything else — and any unsupported platform — gets the **download link**, never a broken half-install.
- **Never replace a running app.** A restart-helper process waits for the app to exit, finishes anything the OS prevents while running (e.g. Windows locks a running exe), and starts the new version.
- **Staged, then renamed; verify, then activate.** Download → verify (SHA-256 + signature) → stage under hidden name → rename → activate. A failed step removes the staged copy and leaves the running version untouched.
- **Unsaved-work gating:** an update restart waits while the app holds unsaved work (“Save or close your open work first; the update restarts the app”).
- **Anti-loop marker.** A version just installed is recorded with a timestamp and not re-offered for 24 h, so a tag/binary version mismatch cannot loop update → restart → “update available”.
- **Failure recovery:** a failed self-update writes its error to a log file shown once at the next start; the update stays on offer.

**Opt-out and localization**
- An env var / build flag **disables update checks entirely** for distro packaging (Nix, Flatpak, deb/rpm repositories) — the system package manager owns updates there.
- All updater messages (prompt, status, failure notes) follow the app’s **interface language**, translated at draw time; network error text passes through as received.

**In-app update GUI (the user’s only update path).** Every suite app must provide an **integrated, one-click update mechanism entirely within the application GUI**. The user must never be directed to a website, GitHub Releases page, command line, or package manager to initiate an update. The update flow is:

1. **Background check:** On startup (once per run), the app quietly checks GitHub Releases API for a newer version. If the app is up to date, no UI is shown.
2. **Subtle notification:** When a newer release is found, a small, non-intrusive banner/toast appears at the top or bottom of the main window — not a modal that blocks the app. It states: “New version available: X.Y.Z. Select to update.”
3. **One-click update dialog:** Selecting the banner opens a standardized inline dialog (within the app’s normal window, not a separate modal) with: Current version and new version; Release highlights (Added/Changed/Fixed, derived from release notes); Two buttons: **Update & Restart** and **Later**.
4. **Update & Restart:** The app performs the full update pipeline internally: Downloads the platform-specific installer to a temp location; Verifies SHA-256 against `SHA256SUMS.txt` and the minisign signature; Runs the stale-version remediation (removes old app-drawer entries); Stages the new version under a hidden name; Renames it into place → Restarts the app automatically (or waits for user launch). All of this happens without the user leaving the app, opening a browser, or running any commands.
5. **Later:** Dismisses the notification; an installation already in progress continues and reports through the updater.

**Never direct the user to GitHub or a browser for routine updates.** The only exception is if the update flow fails at step 4 after signature verification — in that case, a “Download manually from release page” link may be offered, but this is a failure path, not the normal flow.

**Platform-specific installer detection.** The app detects how it was originally installed (macOS `.app`, Windows `.msi`/portable zip/standalone, Linux AppImage / deb / rpm / pacman / Flatpak / standalone) and only offers self-update for packaged copies in supported formats. Development builds (`cargo run`) and package-manager-owned copies defer to their respective managers.

**Anti-loop marker.** A version just installed is recorded with a timestamp and not re-offered for 24 h, so a tag/binary version mismatch cannot loop update → restart → “update available”.

**Failure recovery.** A failed self-update writes its error to a log file shown once at the next start; the update stays on offer.

### Release pipeline (draft → publish)
- Release builds on a `release` branch derived from `main`. Version lives in root `Cargo.toml` workspace package.
- CI builds a **draft** GitHub Release named `<app> v<version>` with release notes from CHANGELOG.
- **Machine-checked release contract:** every day and before upload, validates asset naming, `SHA256SUMS.txt`, no stray assets.
- **verify.sh:** downloads each artefact, verifies SHA-256 matches `SHA256SUMS.txt`, signature validates against project GPG key. If verification fails, release not promoted.
- **Human publishes the draft.** Maintainer reviews draft release, confirms notes accurate, publishes.

## 8. Path to a Finished App + PR Discipline (guide §8)

### Phased Tasks
1. Foundation — skeleton, CI, launch, theme, PR template, release workflow
2. v0.1.0 — first functional test build (Section 6.1)
3. Roadmap expansion + public research (human-gated)
4. Shared crash-report + update-check crates
5. Remaining workflows — one Task / release slice at a time
6. Polish, a11y, empty/error states
7. Multi-platform CI + installers
8. Semver releases until Section 9 gates pass
9. Declare finished only when Section 9 is satisfied

### Definition of Done (Every Task)
- Acceptance criteria met with real behaviour
- Tests updated; CI green
- PR opened using template; reviewed
- Exercisable on local machine when binary available
- Tracking + ROADMAP/CHANGELOG updated as appropriate
- No fake success states; no PC-freezing local builds
- Clean-room / legal checklist not violated

### Pull Request Template (Required)
Create `.github/PULL_REQUEST_TEMPLATE.md`:
```
## Summary
<!-- What this PR does in 1–3 sentences -->

## Type
- [ ] Feature
- [ ] Fix
- [ ] Docs / research (summaries + URLs only)
- [ ] Infra (CI, release, shared crates)
- [ ] Refactor

## Checklist
- [ ] Acceptance criteria met with **real** behaviour (no fake UI)
- [ ] Tests added or updated
- [ ] CI is green (or will be after merge of required base)
- [ ] No unbounded local full builds used to “verify” on a constrained machine
- [ ] No proprietary source, assets, or RE material introduced
- [ ] Public research notes are original summaries + URLs only
- [ ] ROADMAP / CHANGELOG / parity updated if this completes a tracked item
- [ ] Secrets and tokens not committed
- [ ] User-facing strings and UI are original (no copied trade dress)

## Local verification
<!-- How a human can try this on the primary OS -->

## Notes / risks
```
Reviewers reject PRs that fail the checklist.

## 9. Quality Control — Finished Product Gates (guide §9)

- Every in-scope capability Implemented and Verified (local first where possible)
- Main workflows end-to-end
- Persistence and integrity tested
- No fake functionality
- CI green; primary platform hands-on tested
- Additional platforms as scoped
- Crash reporting and update-check work as designed
- ROADMAP and CHANGELOG accurate
- No secrets; licences listed; README accurate and user-friendly for a non-technical average user — professional paid-app feel, not developer docs. README must be written in plain language, explain what the app does and why it matters, show how to install/run with minimal steps, and include screenshots of key features when polished enough. No code-centric jargon, no build instructions for end users.
- Section 2.5 legal checklist clean
- Clean-room rules respected
- **GitHub hygiene gate on every major push to main or version release:** Review GitHub repository health — verify README accurate, clear stale/broken CI checks, close/triage obsolete PRs/issues, remove draft PRs and stale issue labels, ensure repo description/topics/badges correct, confirm Releases page clean. No release with lingering errors, stale PRs, or out-of-date README.
- **Release notes and token metrics:** Every semver release must include detailed release notes describing app usage / feature changes, plus token usage metrics: total tokens consumed since beginning development and tokens used in this release. Release notes published to GitHub Releases and embedded in-app.
- **In-app version & release notes:** App must display version number accurately but subtly in a UI-appropriate corner, non-intrusive. Selecting the version opens a dedicated release notes pop-up showing full release notes for all versions, detailed app usage per release, and cumulative / per-release token usage metrics.
- **Auto-update check:** Every app must be able to check for a newer release, notify user in simple user-friendly way, and offer one-click auto-update from latest GitHub Release. Check, notification, and update flow must be simple, consistent across all suite apps, and non-intrusive.
- **Contributor credits system:** App includes credits screen (About ▸ Contributors) with consent-verified entries from `contributors/people.toml`. AI agents and models credited separately under About ▸ Models, never as people. PRs must never claim agent or bot authorship.
- **Build attestations (optional):** Release pipeline may include SLSA provenance and Sigstore attestation step (`gh attestation verify`) verifying build integrity of every release artefact.

## 10. UI / UX & Visual Assets (guide §10)

- Clean, modern, information-dense but not cluttered; consistent tokens; strong keyboard support
- **Original** visual design — no commercial look-alike branding

### Logos, Icons, Illustrations
1. Describe marks in `docs/design/brand.md` (meaning, shapes, colours — original).
2. Prefer agent- or hand-authored **SVG** under `assets/`; record licence in `ATTRIBUTION.md`.
3. Marketing rasters only if licence-clear; prefer SVG in-app.
4. Never copy competitor icons or trademarks.
5. Generate platform icons from SVG in CI when needed.
6. **UI verification evidence:** before opening a PR, attach offscreen snapshot render or control-channel screenshot (headless, no display needed) showing the UI change. Include before/after screenshots in PR body. Colours, radii, and spacing must come from design tokens — never hard-coded values.

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
- UI: egui/eframe 0.37 GUI only (default feature); TUI/crossterm removed.
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

## Clean-room rules (guide Part I §2)

### Allowed — public black-box research
Only public materials to understand *what* features exist and *how users
interact*: official public documentation/help pages, publicly published
manuals/getting-started guides, public tutorials/blogs/talks, public marketing
screenshots and demo videos (workflow understanding only — not pixel-copying),
public API specifications, open standards. Store only original behavioural
summaries, acceptance criteria, and URLs to public sources — never verbatim
copyrighted manual text, scraped HTML, or third-party screenshots as project
assets.

### Forbidden
1. Decompile, disassemble, or reverse-engineer proprietary binaries/installers.
2. Copy proprietary source, icons, fonts, exact layouts, string tables,
   presets, or internal algorithms.
3. Use prior reverse-engineering reports as implementation input.
4. Reproduce trademarked names, logos, or distinctive trade dress as product
   identity.
5. Paste large copyrighted excerpts into codebase or public docs.
6. Implementation inspired only by public behaviour, public specs, and
   `docs/inspiration.md` / `docs/scope.md`.
7. All data models, commands, UI, interaction design must be **original**.
8. Third-party assets must be permissively licensed and listed in
   `ATTRIBUTION.md`.
9. Prefer dual licence **MIT OR Apache-2.0**.
10. Do not claim affiliation with, endorsement by, or compatibility
    certification from commercial vendors unless true and authorised.

### Trademarks and naming (guide II.18 / II.19.2)
Product name, window title, and visual identity must be original. Before
committing to a name, search existing projects, package registries, and the
WIPO Global Brand Database for conflicts. Reserve handles (domain, org/user
name, package name) even if unused. Record once in `DECISIONS.md`; never
revisit without a new decision entry.

## 11. Source of Truth and Tracking Files (guide §11)

- `docs/project-brief.md`, `scope.md`, `inspiration.md`, `product-name.md`, `architecture.md`
- `docs/parity.md`
- `docs/research/*.md` (summaries + URLs; human-gated)
- `docs/process-feedback.md` (append-only agent friction log)
- `docs/current-model-map.md`
- `docs/design/brand.md`
- `ROADMAP.md`
- `IMPLEMENTATION_PLAN.md`, `REQUIREMENTS_TRACEABILITY.md`
- `DECISIONS.md`, `PROGRESS.md`, `CHANGELOG.md`, `ATTRIBUTION.md`
- `docs/qc/`, `docs/development.md`
- `.github/PULL_REQUEST_TEMPLATE.md`
- `docs/gaps.md` (ranked worklist maintained per release)
- `docs/devlog.md` (gitignored; one entry per session: what landed, version numbers, still-open items — continuity record when sessions end abruptly)

Separate **Facts / Assumptions / Decisions**.

## Completion mandate — every app session runs to overall completion
Drive **every slice and every step through to overall completion of the
application**, not to a convenient stopping point.

- Work the queue in the repository's own records — `ROADMAP.md`,
  `IMPLEMENTATION_PLAN.md`, `PROGRESS.md`, `REQUIREMENTS_TRACEABILITY.md`. If an
  in-scope item is unfinished, the session is not finished.
- Keep the cycle moving: plan → implement → test → CI → fix → **next slice**.
  Do not pause between slices to ask whether to continue; continue.
- Never present a summary as though the work were finished while items remain.
  A summary ends a task; it never substitutes for finishing one.
- Declare the application complete **only** when the guide's quality gates pass
  (Part I §9, the §2.5 per-release legal checklist, and CI green on the default
  branch). "Mostly done" is not done.

## Part II — Workspace additions

### II.19 Legal depth (extends Part I section 2)

#### II.19.1 Copyright and originality
Every line of code, every string, every icon, every layout and every algorithm must be original: derived from public standards, the project's own spec, and capability-level inspiration recorded in `docs/inspiration.md`. The test is not "does it look similar?" but "could this have been written without access to the inspiration product's internals?" If the answer is no, it does not ship.
The designer maintains the capability inventory (`docs/capability-inventory.md`) with a source URL for every claimed behaviour. No capability may be implemented until its source URL is recorded and its wording is original.

#### II.19.2 Trademarks and naming
The product name, window title, and visual identity must be original. Before committing to a name, search existing projects, package registries, and the WIPO Global Brand Database for conflicts. Reserve handles (domain, org/user name, package name) even if unused. Choose clarity over cleverness. This decision is recorded once in `DECISIONS.md` and never revisited without a new decision entry.

#### II.19.3 Licence
Prefer dual licence **MIT OR Apache-2.0**. Before the first public release, `ATTRIBUTION.md` must list every third-party crate with its licence, and the `cargo deny` check in CI (`deny.toml`) must pass. The dual-licence "OR" is deliberate: confirm it per project, and report it accurately in any SBOM.

#### II.19.4 Secrets and tokens
No API keys, tokens, or credentials in git or release binaries. Crash reports and update checks use user-supplied tokens only (GitHub Releases API, Issues API via device flow); no embedded privileged bot token ever ships in a client binary. Secret scanning (gitleaks) runs in CI on every PR.

#### II.19.5 Release legal checklist
The checklist in Part I §2.5 runs before **every** public release, not just v1.0.0. The release is not cut until every row is checked and recorded in `PROGRESS.md` with the CI run ID as evidence.

## II.20 Security-audit skill

### II.20.1 What is in scope
The security audit covers:
- The application itself on every supported platform (for example Windows, macOS and Linux).
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
| Low | Hardening opportunity, no direct exploit path | Track in backlog; fix at maintainer discretion |

### II.20.3 Reporting and disclosure
Reports go through GitHub's private vulnerability reporting (Security tab → "Report a vulnerability", or a draft advisory). Acknowledgement within seven days — a firm commitment given the project's spare-time cadence. Coordinated disclosure: the maintainer confirms, judges severity, prepares a fix, and agrees a disclosure date with the reporter. Reporters are credited unless they ask not to be.

### II.20.4 Scanner gates
Every PR runs the applicable scanners: secret scanning, SAST, and a dependency/licence check (`cargo-deny` for Rust projects). The SBOM (CycloneDX) is generated and attached to every release. These gates are required status checks on `main` (`.ci-required.txt`); direct pushes are disabled.

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

## II.22 Marketing

### II.22.1 Message
Write one paragraph: what the app does, who it is for, why it is different. This becomes the README opening, the repo description, and the announcement text. Prefer plain language and inclusive wording ("they" for a single person); many readers are not native English speakers.

### II.22.2 Handles and presence
Reserve the handles (domain, org/user name, package name) even if unused, so the project has a single "home" URL to point at. The writing style is part of the brand — consistent across README, repo description, release notes, and issue templates.

### II.22.3 What marketing is not
No false affiliation or "drop-in replacement" claims unless carefully qualified. No claims of compatibility certification from commercial vendors unless true and authorised. Release notes are honest about scope and limitations; they do not over-promise.

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

## II.24 Definition of Done

### II.24.1 Per slice (every Task)
- Acceptance criteria met with real behaviour — no fake UI, no mock data.
- Unit tests cover happy path, boundary, and failure/edge cases; headless domain tests need no display.
- Code is formatter- and linter-clean in the project's own toolchain (for Rust, CI runs `cargo fmt --all -- --check` and clippy with `-D warnings`; no toolchain command is run locally).
- CI green on GitHub Actions: the project's format → lint → test → release-build chain passes on every supported platform.
- No fake success states, dead controls, or placeholder-only data.
- **No personal data and no secret in the change.** The scan in II.29 is clean across the working tree, the history and the commit metadata. Placeholder, seed, sample and fixture values are obviously synthetic (`Jane Doe`, `dev@example.com`, `example.com`), never a real person's.
- `CHANGELOG.md`, `PROGRESS.md`, and `REQUIREMENTS_TRACEABILITY.md` updated with exact CI results.
- Change is small and reviewable; PR opened using the template.

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
- Release assets (`II.29.6`) scanned for secrets and personal data; the SBOM and any installer carry no maintainer home path or personal identifier.

## II.25 Launch readiness

Checked before a public launch or a major release. Items already covered by the release/packaging and legal sections above are not repeated here.

- `README.md` answers, in order: what this does, why it is useful, how to get started, and where to get more help.
- `SECURITY.md`, `CODE_OF_CONDUCT.md`, `CONTRIBUTING.md`, `GOVERNANCE.md`, `VISION.md`, `ACCESSIBILITY.md` and `CONTRIBUTORS.md` are present and current.
- The issue forms (`bug_report`, `feature_request`, `accessibility`) and the pull-request template are in place and render correctly on GitHub.
- Branch protection on `main` requires the authoritative CI checks (`.ci-required.txt`); direct pushes are disabled.
- The stated response expectation (a reply within seven days, maintained in spare time) is published and honest.
- Supported platforms and privacy claims in `README.md` match the built binaries and the code — no overstatement.
- No false affiliation or "drop-in replacement" claims anywhere in the public docs.
- Scanner gates are active and green: secret scanning, SAST, dependency/licence audit, and an SBOM generated and attached to the release.
- Dependabot is enabled for the project's package ecosystem(s) and GitHub Actions.

## II.26 Open questions for the human

### II.26.1 "Local tests must be runnable" vs "no heavy work locally"
Part I section 4 prefers GitHub Actions for heavy work and forbids unbounded local builds that risk freezing the PC, while explicitly wanting the app runnable locally for UX testing. The Part I rule is about *this* machine; `CONTRIBUTING.md` must still document the exact commands to build and test locally, and state that CI is authoritative here only because the *maintainer's* machine is constrained.

### II.26.6 Licence choice: "MIT OR Apache-2.0" default
Part I section 2.2 item 9 ("Prefer dual licence **MIT OR Apache-2.0**") and the licence row of the section 2.5 checklist. No conflict — but confirm the "OR" is intentional for each project, and that `ATTRIBUTION.md` and any SBOM report it accurately.

### II.26.7 The git-identity rule covers pull requests, but GitHub cannot enforce it
The workspace git-identity rule says every git action — "every commit, pull request, merge, and tag" — is authored as `"<App Name> Dev" <dev@example.com>`. What is enforceable: the *commit object*. `git config --local` sets author and committer, and `ci.sh` asserts the identity before committing and refuses a mismatch. What is not enforceable: the **pull request's author** and the **pusher shown on GitHub**. GitHub attributes both to whichever account holds the push credential. The rule as written cannot be satisfied for PRs on the current setup; it is a missing mechanism, not a mistake. Options: (a) narrow the rule to "every *commit*", (b) create a machine account and push with its token, or (c) leave the rule as written and record PR authorship as a known, accepted exception.

## II.27 Designer role and standing responsibilities

The designer is a standing role — not a final polish phase. Four responsibilities run continuously from the first slice to the last release:

### (A) Clean-room analysis of the inspiration product
Study what the reference product(s) in `docs/inspiration.md` actually do, from public sources only, recording the URL for every claim: official documentation, manuals, tutorials, blogs, talks, public screenshots and demo videos (for workflow understanding only — never pixel-copying), public API specifications, and open standards. The method is black box: enumerate what the product *does* — every screen, workflow, command, setting, file format, import/export path, keyboard shortcut, and observable edge-case behaviour. Forbidden absolutely: decompiling or reverse-engineering proprietary binaries; copying icons, fonts, exact layouts, string tables, presets or internal algorithms; using prior reverse-engineering output; reproducing trademarked names, logos or trade dress; committing verbatim copyrighted text or third-party screenshots; pixel-perfect UI cloning. Write original behavioural summaries plus URLs under `docs/inspiration/` (consolidated into `docs/inspiration.md`). New research requires owner approval before implementation depends on it (guide Part I §2.3, §8.4).

### (B) The capability inventory
`docs/capability-inventory.md` defines "all the features and abilities of the app we are designing." A capability not in it can be neither claimed done nor reported missing. One row per capability: ID (e.g. `CAP-014`), Product, Capability, Observable behaviour, Acceptance criteria, Source URL(s), Status (not started / partial / implemented / out of scope). Keep enumerating until a pass over the public surface yields nothing new. Record implementation evidence (test name, CI run) in `docs/parity.md`. Move readiness percentages only from verified evidence. If a capability cannot be delivered within the clean-room rules, mark it `out of scope` with the reason and raise it via the `question` tool — never drop it silently.

### (C) Design vision and its implementation
The designer owns the design and builds it: information architecture, interaction model, visual system (colour, type scale, spacing, radii, elevation, motion), iconography, voice and tone of UI text. The designer may edit source to realise the design, working with `ui-dev` where one exists; domain and business logic stay out of the UI and go to `domain-dev`. Every state is designed: empty, loading, error, disabled, permission-denied, long content, narrow window, high-contrast, reduced motion. No default or unstyled controls, no placeholder or lorem text, no clipped text, no mismatched iconography, no dead controls.

### (D) Parity review
Before any slice, milestone or release is called done, check the app against the inventory and report per capability ID: met, partial, missing, or deliberately out of scope (with reason), each with evidence. The application is not complete while an in-scope capability is unmet. Judge the rendered product, not the intention: present but unreachable, unclear, or broken in a real state is partial, not met. Where we exceed the inspiration product, record it — parity is a floor, not a ceiling. Report honestly: an inflated parity claim is worse than an admitted gap.

### Clean-room analysis method
The designer studies public behaviour only. For each claimed feature of the inspiration product:
1. Identify a public source (official docs, manual, public video, public API spec) and record its URL.
2. Summarise the behaviour in original wording — never paste the source's text.
3. Record the summary and URL in `docs/inspiration/` (consolidated into `docs/inspiration.md`).
4. Submit to the human gate before any implementation depends on it.
No decompilation, no disassembly, no proprietary source, no prior RE output. The output of this section is a list of observable behaviours with URLs, not a copy of the product.

### Capability inventory
`docs/capability-inventory.md` is the single source of truth for what the app is designed to do. Each capability row carries:
- **ID** — stable, e.g. `CAP-014`, never reused or renumbered.
- **Product** — which product the behaviour was observed in.
- **Capability** — one sentence, original wording.
- **Observable behaviour** — what a user can see and do.
- **Acceptance criteria** — how we know it is implemented correctly.
- **Source URL(s)** — public evidence of the inspiration behaviour.
- **Status** — not started / partial / implemented / out of scope.
A capability marked `out of scope` must carry a reason and be raised with the owner via the `question` tool. Never drop a capability silently.
Implementation evidence for each ID lives in `docs/parity.md` (test name, CI run). Readiness percentages move only from verified evidence.

### Design vision
The design vision covers:
- **Information architecture** — how the app's screens, views and settings relate.
- **Interaction model** — how the user moves through flows (create → edit → review → history, as the product requires).
- **Visual system** — colour tokens, type scale, spacing rhythm, radii, elevation, motion.
- **Iconography** — original SVG under `assets/`, licensed and listed in `ATTRIBUTION.md`; never copied competitor icons.
- **Voice and tone** — plain language, inclusive wording, consistent across UI, README, release notes, and issue templates.

### Design implementation
The designer implements the design where the app has no `ui-dev`; otherwise works with `ui-dev`. Domain and business logic stay out of the UI: those changes go to `domain-dev`. The designer edits UI files only (egui/eframe panels, widgets, dialogs, themes).
Every state is designed: empty, loading, error, disabled, permission-denied, long content, narrow window, high-contrast, reduced motion. No default or unstyled controls, no placeholder or lorem text, no clipped text, no mismatched iconography, no dead controls.

### Parity review method
Before any slice, milestone or release is called done, the designer checks the app against the capability inventory and reports per ID: **met** — implemented and verified, with evidence (test name, CI run). **partial** — present but unreachable, unclear, or broken in a real state. **missing** — not implemented. **deliberately out of scope** — with the recorded reason. The application is not complete while an in-scope capability is unmet or partial without a plan. Parity is a floor, not a ceiling: exceeding the inspiration product is recorded and celebrated.

### Parity reporting
Findings are reported in severity order, each with the screen/state, what is wrong, and the concrete fix. The designer verifies the **rendered result**, not just the diff — using CI screenshots or a runnable artefact. Re-review after every fix. Say plainly when it is good; do not invent findings, and do not stay silent about a real gap.

### The loop, on every slice
1. **Analyse** (II.27.2) → 2. **Catalogue** (II.27.3) → 3. **Design and build** (II.27.4, II.27.5) → 4. **Verify in CI** → 5. **Re-check parity** (II.27.7, II.27.8) → 6. **Report**. Verify the rendered result, not just the diff. Report findings in severity order, each with the screen/state, what is wrong, and the concrete fix.

### Constraints
Quality gates run in GitHub Actions, never on the client machine. Never run a build, test, lint, or any `cargo` command locally. Verify with `~/.config/opencode/scripts/ci.sh run "<description>"`. Never present a question or a request for owner input in prose — always use the `question` tool. Drive each slice through to overall completion of the app.

## II.28 Naming conventions inside a repository

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

### II.28.2 What does **not** carry the prefix, and must not
These names are **fixed by cargo, git or GitHub**, and prefixing them breaks the tooling:
| Kind | Correct | Why a prefix breaks it |
|------|---------|------|
| Manifest | `Cargo.toml` | Cargo discovers crates by this exact filename; `<app>_Cargo.toml` is simply not a crate. |
| Crate roots | `src/main.rs`, `src/lib.rs` | Fixed entry points. |
| Module files inside a crate | `engine.rs`, `report.rs`, `update_check.rs` | The module path *is* the name (`<app>_core::engine`); an `<app>_engine.rs` module would be reachable only as `<app>_core::<app>_engine` and reads as stutter. Keep these plain. |
| Integration tests | `tests/properties.rs` | Discovered by path; the crate name is already in the package. |
| GitHub community files | `README.md`, `LICENSE`, `LICENSE-MIT`, `LICENSE-APACHE`, `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, `SECURITY.md`, `CHANGELOG.md`, `GOVERNANCE.md` | GitHub only recognises community health files and the licence badge by these **exact** names; `<app>_SECURITY.md` is invisible to the security tab and the Private Vulnerability Reporting flow. |
| Tooling configs | `.gitignore`, `.ci-required.txt`, `deny.toml`, `dist-workspace.toml`, `.github/workflows/*.yml` | Read by name by git, cargo-deny, cargo-dist and GitHub Actions. |
| Already-scoped directories | `crates/core/`, `docs/architecture.md` | The directory already answers "which app" (the repository) and "which layer"; repeating it in every file inside it is noise. |

So the rule is **"prefix what leaves the repository or must be globally unique; never prefix what a tool finds by name."**

### II.28.3 Deviations to check in each repository
- **Version-first artefact names.** An app may name artefacts `<app>-<version>-<target-triple>` with a versioned checksum file. Its update-check code parses those names, so changing them is a code change plus a release change. Record the deviation in `DECISIONS.md` and decide it, rather than silently aligning.
- **Architecture spelling.** `arm64` and `aarch64` are the same architecture with two spellings. Pick one across the suite (`aarch64` is the target-triple spelling) when the release names are next touched.

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

### II.29.6 Release-asset scan (part of II.24.3)
```
# tree + history
git grep -n -I -i -E '/home/[a-z0-9_-]+|@(gmail|outlook|hotmail|proton|yahoo)\.'
git log --all -S"<personal value>" --oneline
# published artefacts, after download or extraction
grep -rIn -i -E '/home/[a-z0-9_-]+' <extracted-asset-dir>
```
Only `/home/runner` (CI) and obviously synthetic values are acceptable hits.

## 5.5 House Rules (Storytold / Suite Standard)
These 15 rules apply in every suite repository. They are enforced through the shared CI house-rules check (see §4.6) and the `craft-contrib` standards repo. A later rule may be stricter, not weaker.

1. **A person is accountable.** Whoever runs an agent answers for what it opens and for its review comments.
2. **Check, then claim.** Before writing code, read the issue comments, search pull requests for `#N` and the keywords, and check recent commits on `main`. Comment on the issue first. Release a claim you will not finish.
3. **Verify, don't guess.** Open a definition before using it. Confirm the cause in the code, even when someone names a location.
4. **Keep fixes narrow and tested.** Fix what the issue describes, where the bug is. Add a test that fails without the fix.
5. **Review the diff before opening a pull request.** Identifiers exist as used, the test fails without the fix, nothing unrelated changed, and the description is true.
6. **Look for a better pull request.** If another fixes the same thing, close yours when it is better or equal, and say why.
7. **Report only what ran.** In CI status summaries, report only the jobs that actually executed — never fabricate pass/fail for steps that were skipped or masked.
8. **Main `main` through PR only.** No direct pushes to `main`; every change must land via a verified pull request merge.
9. **Secrets are never committed.** No API keys, tokens, or credentials in git or release binaries. Crash reports and update checks use user-supplied tokens only; no embedded privileged bot token ever ships in a client binary.
10. **CI stays shared.** Workflow definitions live in a shared suite repo; feature PRs must not alter the shared CI definitions — make a new issue/PR in the shared repo instead.
11. **Guards say why.** Any agent or tool that blocks or gates work must state the concrete reason in plain language — not just "blocked" or "failed."
12. **Instructions must be readable by everyone.** No notes about one machine's private configuration; all steps must be runnable by any suite contributor.
13. **PR movement SLA.** Open PRs receive a first-response comment within 7 days and a decision (merge, revise, or close) within 14 days; stale PRs (30 days unmodified) are automatically candidates for closure.
14. **Narrow "stop and ask" list.** Only these topics may stop work and trigger a human question: licences, publishing, secrets, money, destructive git operations. For everything else: keep working.
15. **No agent attribution in PRs.** PR descriptions and merge commits must not claim agent or bot authorship. Models are credited separately (About ▸ Models), never as people.

## 12. Reference Implementations & Inspiration (guide §12)

### Inspiration
- **ArtCraft Suite / PhotoCraft** – free, open-source creative suite built as 13+ pure Rust clean-room apps. Key patterns to borrow: suite-wide conventions, native + WebAssembly builds, shared crates for crash-report and update-check, ArtCraft Launcher for consistent install/updates, public GitHub Releases per app, and a community guide site. Source: artcraftsuite.org, github.com/storytold/photocraft.
- **Clean-room reimplementation guidance** – GitHub topic clean-room-reimplementation lists PhotoCraft, OpenRCT2, Wine, etc. Reinforces clean-room process, observable-behaviour only, no proprietary code.
- **Rust egui suite patterns** – egui/eframe guides show immediate-mode UI, cross-platform builds, and CI artefact distribution. Provides basis for subtle version UI and in-app release notes popup.
- **Release notes best practice** – detailed release notes with Added/Changed/Fixed/Known limitations, published to GitHub Releases and embedded in-app. Token usage metrics per release and cumulative totals are a differentiation for this suite.
- **Auto-update patterns** – consistent update-check crate using GitHub Releases API, soft-fail offline, one-click download for OS/arch, user-friendly notification.

Build on these by keeping the suite consistent: shared `crates/update-check` API, shared crash-report, identical version display and release notes popup, user-friendly README with screenshots, and GitHub hygiene gates on every major push/release.

### Suite-wide Standards (guide §12.1)
- **Token usage metrics tracking:** Use a CI-managed JSON ledger `docs/tokens.json` with fields `total_tokens_since_start`, `release_tokens`, `release_version`. CI updates the ledger on each semver tag and commits it. Release notes must reference ledger values.
- **Update-check distribution:** `crates/update-check` is maintained as a git submodule shared by all suite repos. Pin submodule to a tag; update via `git submodule update --remote` in CI.
- **Auto-update security:** Releases are signed and include `SHA256SUMS.txt`. In-app auto-update verifies signature + checksum before install. Soft-fail on network, never block app start.
- **Version UI standard:** Version displayed subtly in bottom-right corner, non-intrusive. Click opens standardized Release Notes popup with: version history, detailed app usage, token metrics, and Update button. Same layout across all suite apps.
- **Addendum archival:** `addendum.md` archived to `addendum.md.archive`. `guide.md` is single source of truth.
- **README screenshots gate:** Screenshots must be reviewed/updated in README on v1.0 and every major version release. Minor releases may skip if UI unchanged.

### Parallel Agents (guide §12.2)
The suite is built with a paid-professional-staff workflow emulated by parallel OpenCode agents. Free models are maximized by running a supervisor + workers model with 6-8 agents per slice, refreshed per slice with a persistent supervisor.

**Principles:**
- Parallelism first: every vertical slice spawns a coordinated team of specialists in parallel, not sequentially.
- Supervisor + workers: one supervisor agent orchestrates, delegates, merges results, and ensures quality gates.
- Free-model maximization: use OpenCode free → NVIDIA free → paid fallback routing per role; never run heavy builds locally.
- Business-grade output: each role produces professional-grade artefacts as if hired staff.
- Model capacity: 6-8 parallel agents consume free-model tiers quickly. If free models are exhausted mid-slice, shrink the team (sequential critical path first, parallel non-blocking work second) or pause and resume when quota resets. Record model exhaustion in `docs/current-model-map.md` and `docs/process-feedback.md`.

**Team composition per slice:**
- Supervisor / Tech Lead – plans slice, assigns tasks, merges outputs, enforces guide.md gates; **sole user of the `question` tool**; workers report blockers to the supervisor, never directly to the human
- Lead Developer – implements core Rust domain logic and tests
- Reviewer – code review, clean-room compliance, security, clippy, naming
- QA Tester / Bug Hunter – writes and runs integration tests, finds edge cases
- UI/UX Designer – designs screens, flows, empty states, accessibility
- Graphic Designer – creates icons, assets, branding, screenshots
- Tooling Dev – CI, release notes, update-check, crash-report wiring
- Optional rotating specialists – performance, accessibility, docs writer

**Worker output format:** Each worker reports back to the supervisor with: artefacts produced (files), summary of what was done, acceptance criteria met/not met, blockers, tests added/updated (name the test file/function).

**Workflow:**
1. Supervisor creates task brief with acceptance criteria for each specialist.
2. Workers run in parallel, each producing artefacts in their domain.
3. Reviewer and QA run in parallel on produced code.
4. Supervisor merges worker outputs, resolves conflicts, updates ROADMAP/CHANGELOG/PROGRESS.
5. Supervisor asks any human questions (via `question` tool) — never workers directly.
6. Push branch → `ci.sh run` → CI green → PR → merge to main.
7. Repeat per slice with persistent supervisor continuity.

**Operating rules:**
- Agents are spun up per slice, supervisor persists across slices.
- All outputs must pass guide.md Quality Gates §9 and legal checklist §2.5.
- No agent runs `cargo` locally; all builds/tests via CI.
- Release notes, token metrics, and GitHub hygiene gates apply to every slice.
- Parallel work never bypasses human client sign-off on scope and research notes.
- Only the supervisor pushes, merges, or tags — ensures git identity `"<App Name> Dev" <dev@example.com>` is consistent.

## II.30 Rust GUI/UX — stack matching, tokens and evidence (guide §II.30)

### II.30.1 Hard rule — stack matching
| Source | Stack | Use for a native Rust desktop GUI? |
|--------|-------|-----------------------------------|
| **Astryx** (`meta/astryx`) | React 19 + StyleX | **No** as a dependency. Optional **inspiration only** — token names, density vocabulary, a11y wording. |
| shadcn/ui, Material web, any CSS design system | JS/CSS | **No** as code. Patterns only. |
| **egui / eframe / iced / Slint / GPUI** ecosystems | Rust | **Yes** — primary sources. |

Agents must:
1. Detect the actual GUI crate and its resolved version (II.30.2) **before** recommending anything.
2. Match every recommendation to that crate; an egui recipe is not advice for an iced app.
3. Never propose adding React, StyleX, Astryx, Tailwind or a CSS framework to a pure Rust immediate-mode or retained-geometry app unless the repository already ships a web front end.
4. Never answer "improve the design" with prose alone. A UX claim needs tokens, contrast, keyboard paths or rendered evidence (II.30.8).

### II.30.2 Detect the stack before recommending anything
Read-only commands only — allowed on the client machine; `cargo` is not.
- Declared dependencies: `grep -nE '^(eframe|egui|egui_extras|egui-wgpu|iced|slint|gpui|tauri|dioxus|leptos)[[:space:]]*=' Cargo.toml apps/*/Cargo.toml crates/*/Cargo.toml`
- Resolved version: `grep -A2 -E '^name = "(egui|eframe|iced|slint|gpui)"' Cargo.lock`
- Where used: `grep -rln 'egui::' --include='*.rs' . | head -20`

Record in `docs/design/gui-stack.md`: toolkit, **resolved** version, shell shape (does the UI render into a plain `&mut Ui`?), theme module path, whether an a11y/snapshot harness already exists, and the window/shell layout model.

### II.30.3 Version discipline
- Pin recommendations to the version resolved in `Cargo.lock`, never to the range in `Cargo.toml`.
- **A UX slice never bumps the toolkit.** Dependency moves are their own slice: manifest + lock diff, CI green, rendered evidence re-reviewed, and a `CHANGELOG.md` note.
- Prefer crates that document support for the project's egui version.
- Adopting a design crate the project's toolkit version has left behind is a recorded risk in `DECISIONS.md`, not an accident discovered at review time.
- Confirm the API shape from current documentation (Context7 or the crate's own docs) before adopting a design crate.

### II.30.6 One token source — semantic roles, not hex literals
A GUI slice starts with tokens, not widgets. One theme module owns every colour, space, radius, elevation and text size; widget code reads roles.
- Placement: `apps/<app>/src/theme/` or a `crates/<app>-theme` crate with no egui dependency for the palette maths.
- Semantic roles (minimum set): `surface.base`/`.raised`/`.sunken`, `text.primary`/`.secondary`/`.muted`/`.disabled`, `accent`/`.hover`/`.on`, `danger`/`.warning`/`.success` (+ `.on`), `border.subtle`/`.strong`, `focus.ring`, `overlay.scrim`, `selection.bg`/`.text`.
- Scales: Spacing 2-4-8-12-16-24-32-48 (4-point base); radii 2/4/8/12; elevation three levels maximum; type scale one ratio (1.125 or 1.2) with roles `display`, `h1`, `h2`, `body`, `small`, `mono`.
- Mapping to the toolkit: `Tokens` exposes `fn visuals(&self) -> egui::Visuals` and `fn style(&self) -> egui::Style`.
- Variants: Light, dark, and high contrast, with system-follow as the default and the user's choice persisted.

**Enforcement (process only, cheap):** A CI grep gate fails the build when a raw colour constructor appears outside the theme module:
```bash
! grep -rn --include='*.rs' -E 'Color32::from_(rgb|rgba|gray|black|white)' \
    apps/*/src crates/*/src | grep -v 'src/theme/'
```

### II.30.7 Accessibility and input as code
**1. Contrast is a test, not an opinion.** Keep luminance and contrast maths as pure functions in the token crate and assert the palette:
```
contrast(a, b) = (L1 + 0.05) / (L2 + 0.05), with L = 0.2126R + 0.7152G + 0.0722B
after sRGB linearisation
```
Required: **4.5:1** for body text, **3:1** for large text (≥ 24 px regular or ≥ 18.66 px bold), **3:1** for meaningful UI boundaries and focus rings, **3:1** minimum for every state colour against its own surface. Assert both themes and the high-contrast variant in one `#[test]`. This runs headless and is a merge gate.

**2. Colour is never the only signal.** Errors carry text and an icon; selection carries a fill **and** a marker; disabled carries reduced emphasis plus a reason.

**3. Keyboard completes every flow.** Tab order follows reading order; Esc dismisses the topmost dialog; Enter activates the default action; focus is never trapped; the focus ring uses `focus.ring` and is visible on every interactive widget. The shortcut table lives in `docs/qc/shortcuts.md` and is updated whenever one is added.

**4. Icon-only controls are named.** A tooltip is not a name. Set the widget's `WidgetInfo`/`labelled_by` text as well, so the AccessKit tree carries the label.

**5. Target size.** Interactive targets are at least **24 logical points** on their shortest side; primary actions and destructive actions are at least 32. Density variants (compact / standard / comfortable) change spacing, never below the 24-point floor.

**6. Reflow.** At a narrow window and at 200% zoom no control clips or overlaps; the shell degrades by collapsing the side panel before shrinking the canvas. One panel at a time may shrink; none may become unreachable.

**7. Reduced motion — honest limitation.** egui has no cross-platform system-level reduced-motion query. Provide an in-app preference, default animations **off** when the platform signal is unavailable, keep transitions under ~120 ms and non-blocking, and record the limitation in `ACCESSIBILITY.md`.

### II.30.8 Evidence — rendered, not intended
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
```
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

**1. Astryx, React, StyleX or any CSS design system inside a native Rust GUI.**
**2. Two competing egui design systems in one binary.**
**3. Rebuilding the app shell around a framework without an explicit migration task and client approval.**
**4. Copying the inspiration product's pixels, icons, fonts, string tables or presets (II.27.2).**
**5. An emoji set standing in for an icon system; icons are original SVG under `assets/`, licensed and listed in `ATTRIBUTION.md`.**
**6. A toolkit or design-crate version bump hidden inside a UX slice.**
**7. "Design improved" claimed without contrast, keyboard or rendered evidence.**
**8. Any heavy build, test or `cargo` command on the client machine (II.27.10).**

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

## XI. Design Guidelines (design-guidelines.md)

This section transcribes `~/design-guidelines.md` — the canonical source for visual and interaction design rules used by agents working on guide.md-governed projects.

Agents should apply the positive patterns in Section 2 first, then run the anti-pattern list as a rejection filter before shipping UI.

### XI.1 Design Anti-Patterns — "Vibecoded" Signals & Intentional Alternatives

The following 30 items are common visual, typographic, and layout signals that indicate AI-generated ("vibecoded") design. When present, they signal the designer relied on default templates rather than intentional design decisions. Use this as a hardening checklist — avoid all 30, or deliberately subvert them with purpose.

**1. Harsh gradients**
**Why it's bad:** High-contrast, multi-stop gradients (especially purple-to-blue or neon) are the #1 visual fingerprint of AI-generated sites. They feel artificial and overused.  
**Better:** Soft, single-direction gradients with low contrast, or solid colors with subtle texture/noise. Prefer flat or very restrained color transitions.

**2. Lucide icons**
**Why it's bad:** Lucide became the default icon set for shadcn/Tailwind projects. Using it unchanged makes your UI look identical to thousands of other AI sites.  
**Better:** Choose a distinctive icon set (Phosphor, Tabler, Heroicons, Remix, or custom icons). Or mix styles intentionally.

**3. Pure white background**
**Why it's bad:** Stark `#FFFFFF` with no warmth feels clinical and template-like.  
**Better:** Slightly off-white, warm cream, cool gray, or very dark backgrounds. Give the page some temperature.

**4. Rainbow coloring**
**Why it's bad:** Random multi-color accents with no hierarchy look chaotic and unprofessional.  
**Better:** Strict 1–2 accent colors maximum. Everything else should be neutral.

**5. Drop shadows**
**Why it's bad:** Soft, diffuse, multi-layer shadows on every card are a classic AI default.  
**Better:** Very subtle shadows, hard shadows, or no shadows at all (use borders or background contrast instead).

**6. 3 feature cards in a row**
**Why it's bad:** The most overused layout pattern in AI landing pages. Instantly recognizable.  
**Better:** Asymmetric layouts, 2-column, 4-column, zig-zag, or editorial-style feature sections.

**7. Emojis**
**Why it's bad:** Emojis as icons or in headings feel playful but lazy and unprofessional in product UI.  
**Better:** Real icons or custom illustrations. Use emojis sparingly (if at all) for tone, never as primary UI icons.

**8. Liquid glass**
**Why it's bad:** Heavy glassmorphism (blur + transparency + border) became the new purple gradient. It ages quickly.  
**Better:** Solid surfaces with clear hierarchy, or very light glass used only on specific elements (e.g. floating nav).

**9. Em dashes**
**Why it's bad:** AI models overuse em dashes (—) in marketing copy. It's a subtle but strong writing tell.  
**Better:** Use commas, periods, or rephrase. Prefer shorter, punchier sentences.

**10. Inter / Geist / Space Grotesk**
**Why it's bad:** These three fonts dominate AI-generated sites. Using them as the main typeface makes your site blend in.  
**Better:** Pair a distinctive display font with a clean body font. Or use a single high-quality typeface with strong personality.

**11. Colored left stripe**
**Why it's bad:** Thin colored vertical bars on the left of cards/sections is a very common AI decorative habit.  
**Better:** Use background color, borders, or spacing for hierarchy instead of decorative stripes.

**12. Fake testimonials**
**Why it's bad:** Invented names + generic praise + stock avatars destroy trust the moment someone notices.  
**Better:** Real testimonials only, or none. Use logos of real companies, usage metrics, or case studies instead.

**13. Bento grids**
**Why it's bad:** Once distinctive, now heavily overused in AI designs.  
**Better:** Use them only if they genuinely serve the content. Prefer more classic or asymmetric layouts when possible.

**14. Terminal window**
**Why it's bad:** Decorative terminal/code windows are a cliché for "developer tool" landing pages.  
**Better:** Show the real product UI, actual screenshots, or a clean interactive demo instead.

**15. "It's not X, it's Y"**
**Why it's bad:** Extremely common AI copy pattern ("It's not just a tool, it's a...") that feels formulaic.  
**Better:** Be direct. State what the product actually does without the rhetorical contrast.

**16. Checkmark bullets**
**Why it's bad:** Green checkmarks in circles are the default "feature list" treatment in AI output.  
**Better:** Custom icons, numbers, simple dashes, or just strong typography hierarchy.

**17. 3 pricing tiers**
**Why it's bad:** The classic Free / Pro / Enterprise (or Starter / Growth / Scale) layout is the most templated pricing section.  
**Better:** 2 tiers, usage-based, or a single clear plan with add-ons. Make the pricing structure match how you actually charge.

**18. No real product demos**
**Why it's bad:** Sites that only show abstract illustrations or generic mockups feel incomplete.  
**Better:** Real screenshots, short product videos, interactive demos, or Loom-style walkthroughs.

**19. Soft corner radius**
**Why it's bad:** Uniform large border-radius on every element creates a soft, indistinct "AI aesthetic."  
**Better:** Mix radii intentionally (sharp + soft), or go fully sharp for a more editorial feel.

**20. Purple and black**
**Why it's bad:** The dominant AI color combination for years. Extremely recognizable.  
**Better:** Choose a unique primary color that fits the brand. Avoid defaulting to purple/violet/indigo.

**21. No skeleton loaders**
**Why it's bad:** Missing loading states make the app feel unfinished or janky.  
**Better:** Always include proper skeleton screens or subtle loading indicators for async content.

**22. Radial orbs**
**Why it's bad:** Glowing circular blurs in the background are a classic decorative AI filler.  
**Better:** Subtle noise, grain, geometric shapes, or just clean solid backgrounds.

**23. Dot grids**
**Why it's bad:** Dot grid patterns as backgrounds became a very common "modern" AI decoration.  
**Better:** Minimal backgrounds. Use texture sparingly and purposefully.

**24. Sparkle icons**
**Why it's bad:** Sparkles / stars / ✨ used as decorative icons scream AI-generated.  
**Better:** Meaningful icons only. Avoid decorative sparkles entirely.

**25. Animated arrows**
**Why it's bad:** Looping or bouncing arrows are a cheap way to add motion that quickly feels gimmicky.  
**Better:** Purposeful micro-interactions. Motion should support usability, not just look "alive."

**26. No TOS**
**Why it's bad:** Missing legal pages signals the site was thrown together quickly.  
**Better:** Always include Terms of Service (even a simple one).

**27. No privacy policy**
**Why it's bad:** Same as above — looks incomplete and can create real legal risk.  
**Better:** Always ship a Privacy Policy.

**28. Hover animations**
**Why it's bad:** Adding hover effects to every element creates visual noise and feels unrefined.  
**Better:** Selective, purposeful hover states only on interactive elements that benefit from them.

**29. Neon colors**
**Why it's bad:** High-saturation neon accents feel dated and "trying too hard."  
**Better:** Muted or mid-saturation accents. Save high intensity for rare, intentional moments.

**30. Basic pastel colors**
**Why it's bad:** Soft pastel palettes (especially the common blue/pink/mint/yellow card sets) are heavily overused in AI designs.  
**Better:** More intentional, slightly desaturated, or higher-contrast color systems with personality.

**Pro tip:** Treat the list above as hard constraints. When in doubt, choose the less common, more intentional option.

### XI.2 High-Quality UI Patterns

See `design-guidelines.md` **Section 2** for 20 practical patterns that raise perceived quality, responsiveness, and trust.

**Applicability for the suite’s native Rust + egui stack:**

| Pattern | Native (egui) fit | Web fit |
|---------|-------------------|---------|
| Skeleton loaders, Optimistic updates, Command menu, Shortcut hints, Undo toasts, Interruptible animations, Hit slop, Tabular numbers, Continuous corners, Idempotency keys | Excellent | Excellent |
| Stale-while-revalidate, Safe area insets, Balanced headlines | Good | Excellent |
| Pre-rendering, BlurHash placeholders, View transitions, Swipe-to-dismiss sheets, Predictive back, One-time code autofill, Amazon safe triangle | Limited / N/A | Excellent |

**Agent guidance:**

1. Prefer the patterns in Section 2 that fit the current stack (native egui first; web patterns apply only when the project has a web frontend).
2. After implementing UI, run the anti-pattern checklist in Section XI.1 as a rejection filter.
3. When in doubt, choose the less common, more intentional option.

### XI.3 Design Guidelines Synchronization

- `design-guidelines.md` is the canonical source for visual and interaction design rules.
- This section is reviewed every section in complete detail per **guide.md §12.3**.
- Updates to `design-guidelines.md` must be reflected in AGENTS.md within the same session cycle.
- The anti-pattern checklist and high-quality patterns are enforced through the CI code-review gate.

## Questions and user input — always presented via the question tool
If you need a decision, a clarification, or any input from the owner, **present
it with the `question` tool**. Never bury a question in prose, never assume an
answer, and never stall silently waiting for one.

- Ask only when genuinely blocked: a destructive or irreversible action needs
  confirming, two readings of the requirement lead to different
  implementations, or the needed fact exists nowhere in the repo or docs.
- Offer concrete options with your recommendation first, and keep it to the
  minimum number of questions needed to unblock.
- Then act on the answer immediately and carry on with the slice — do not
  restart the task, and do not ask the same thing twice.
- Everything else: derive the answer from the code, the guide, or the
  repository's records and keep working.

---

# APPENDIX A — Merged Addendum (Single Source of Truth)

## K. Rust applications — will autonomous QA work?

**Yes — with a Rust-shaped process.**  
Native Rust GUIs (egui/eframe, iced, etc.) do **not** get full Playwright-style
browser QA for free. Autonomous QA still works well if the app is structured so
**most behaviour lives outside the paint loop** and is tested with `cargo test`
in GitHub Actions.

### K.1 What works well in Rust (use these)

| Mechanism | Role |
|-----------|------|
| `cargo test` in CI | Primary autonomous QA engine |
| Domain / command-layer tests | Real workflows without opening a window |
| Temp-dir save/load tests | Persistence and file formats |
| `clippy -D warnings` + `fmt` | Catch foot-guns before runtime |
| Multi-OS CI matrix (after v0.1.0) | “Runs properly” on more than one OS |
| Release job gated on tests | No green tag without suite pass |
| Regression tests from client BROKEN reports | Suite gets smarter over time |

### K.2 What does *not* work like web apps

| Web pattern | Rust native reality |
|-------------|---------------------|
| Playwright clicks through DOM | egui/iced have no DOM; browser MCP does not drive the native window |
| Easy full-UI CI smoke | Full GUI automation is optional/hard; do not block v0.1.0 on it |
| “Just add Playwright MCP” | Use only for **WASM/web** targets, or skip |

**Rule:** Autonomous QA for Rust desktop = **strong headless tests + CI**. Client
still does a short **visual/UX** pass on green builds.

### K.3 Required architecture (agent must design this)

Split the app so tests do not need a GPU window:

```
apps/{name}/          → UI shell (egui/eframe) — thin
crates/{name}-core/   → domain, commands, persistence — thick, fully tested
crates/crash-report/
crates/update-check/
```

- UI sends **commands** / messages into core.
- Core returns **results** / view-models.
- **Workflow tests call core only** (create → mutate → save → load → assert).
- UI bugs still need client eyes; logic bugs should fail CI first.

If the agent puts business logic only inside `update()` / paint code with no
testable API, autonomous QA collapses. **Reject that design.**

### K.4 Definition of done (Rust Task)

A Task is not done until:

1. `cargo test` covers the acceptance criteria at **core** level when feasible.
2. CI job runs `fmt`, `clippy -D warnings`, `cargo test`, and `cargo build
   --release` (or workspace equivalents) for the **primary OS**.
3. CI is **green** before asking the client to try a build.
4. Client receives **numbered try-steps** for the one human path (open app → …).
5. On client `BROKEN`: add a **regression test** in core that fails without the
   fix, then fix.

Trivial tests (`assert_eq!(1+1,2)`) do not count as workflow coverage.

### K.5 Workflow test pattern (agent should write)

In `crates/{name}-core` (names illustrative):

```rust
#[test]
fn workflow_create_save_load_roundtrip() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("doc.bin"); // or .json / project extension

    let mut app = CoreApp::new_empty();
    app.apply(Command::AddItem { title: "A".into() }).unwrap();
    app.save_to(&path).unwrap();

    let loaded = CoreApp::load_from(&path).unwrap();
    assert_eq!(loaded.item_count(), 1);
    assert_eq!(loaded.item_title(0).unwrap(), "A");
}
```

Every ROADMAP slice that changes user-visible behaviour should add or extend
such a test.

### K.6 CI minimum (Rust)

`.github/workflows/ci.yml` must include roughly:

```yaml
# Pseudocode — agent adapts to workspace layout
- rustfmt --check
- clippy -- -D warnings
- cargo test --workspace
- cargo build --release -p {app}
- upload binary / artefact for primary OS
```

`release.yml`: publish GitHub Release **only if** the same tests passed on that
commit/tag.

**Caching:** use `Swatinem/rust-cache` (or current equivalent) so free Actions
minutes are not wasted.

**Client laptop:** do **not** run full `cargo test` / release builds locally
when the machine is slow. Push and use CI artefacts.

### K.7 GUI / visual QA (Rust native)

| Approach | When |
|----------|------|
| Client try-steps on CI artefact | **Always** for v0.1.0 and each release |
| Optional screenshot docs in PR | Nice-to-have; not a merge gate |
| WASM + Playwright in CI | Only if the project has a web target |
| `eframe` headless / image tests | Advanced; optional later, not required for v0.1.0 |

Do not delay v0.1.0 waiting for perfect GUI automation.

For the full stack-matching, token, component-recipe and evidence method that
now backs this table, see **II.30**.

### K.8 Crash-report and update-check crates (testability)

- Unit-test payload building and secret scrubbing **without** network.
- Update-check: mock HTTP or test version-compare pure functions.
- Integration “submit issue” may stay manual / behind feature flag in CI.

### K.9 Client BROKEN format (unchanged, Rust-aware)

```text
BROKEN
What I did: ...
What I expected: ...
What happened: ...
```

Agent: encode as `#[test]` under core when possible → fix → CI green → ask
client to re-try.

### K.10 Honest limits

- Autonomous QA **will** catch logic, persistence, and many regressions in
  Rust.
- Autonomous QA **will not** fully replace “does the UI feel professional?”
- Free models still write flaky or shallow tests; reviewer agent / PR checklist
  must demand **workflow** tests.
- Cross-compilation and signed installers are later milestones; primary OS
  green tests come first.

### K.11 Agent bootstrap rules (Rust projects)

On new Rust apps, the agent must:

1. Create `crates/*-core` (or equivalent) before large UI work.
2. Land at least **one** save/load or core workflow test before tagging
   **v0.1.0**.
3. Wire CI as in K.6 before calling foundation “done.”
4. Prefer `tempfile` + in-memory or temp paths in tests (no machine-specific
   paths).
5. Document in README: “Download the CI artefact; you do not need Rust
   installed to try the app.”
6. Add `rust-toolchain.toml` (stable channel pin) so CI and agents use the
   same compiler.
7. Add `tempfile` as a dev-dependency of core for workflow tests.
8. **Never** run full `cargo test` / `cargo build --release` on the client
   laptop when resource-constrained; push and use CI artefacts.
9. **HARD BUILD ENFORCEMENT — NEVER REVERT:** All models, agents, and
   sessions are forbidden from running `cargo` or any resource-intensive
   build/test command locally. Heavy work is **only** via
   `~/.config/opencode/scripts/ci.sh` → GitHub Actions. This rule is global
   for all suite repos and is enforced by the `opencode-ci-only` plugin. No
   exceptions.

### K.12 Clippy pragmatism

- **After** the first green CI skeleton: enable `clippy -- -D warnings` as a
  gate.
- During chaotic scaffolding only: clippy may run as report-only **for at most
  one PR**, then become blocking.
- Do not permanently disable clippy to hide design problems.

### K.13 Tests must not need a display

- Core tests run headless on CI with **no** `DISPLAY`, GPU, or window server.
- Do not call egui/eframe/iced APIs from `*-core` tests.
- If a test needs a GUI, it is in the wrong crate — move logic to core or mark
  `#[ignore]` with a tracked issue (avoid ignores for v0.1.0 paths).

### K.14 Non-Rust applications — the same rules, translated (added 2026-10-27)

Sections K and M are written for Rust. They are **binding in intent** for every
app, so a TypeScript/web/React app (for example VellumScribe) obeys the same
four rules with its own toolchain. Read every K/M rule through this table:

| Rust rule (K/M) | Non-Rust equivalent |
|-----------------|---------------------|
| `crates/*-core` with logic outside the paint loop | Domain/service layer in its own modules, imported by UI components; UI holds no business logic. A change that puts real logic inside a component instead of a service is a review rejection, exactly as burying logic in `update()` is in Rust. |
| `cargo test` in CI as the primary QA engine | The project's headless test runner (`vitest` here) as the primary QA engine, run on Linux, Windows and macOS. |
| Headless workflow test per user-visible slice (K.5) | A test that drives the real service/domain path with real persistence — not a component snapshot or an `expect(true)`. Name it in the PR. |
| `clippy -D warnings` + `rustfmt --check` gates | Lint with **zero warnings allowed** (`eslint src --max-warnings=0`) and a formatter **check** gate (`prettier --check .`) — both blocking. |
| `cargo build --release` on CI, artefact for the client | The project's release build in CI (signed APK / bundled server), artefact attached for the client to install. Never built on the client machine. |
| `Cargo.lock` / `cargo deny` / ATTRIBUTION | The lockfile committed; a licence gate (`scripts/check-licenses.mjs`) plus SBOM generation in CI; `ATTRIBUTION.md` current. |
| `egui_kittest` / rendered PNG evidence (K.7, II.27.12) | Playwright screenshots of the real built app in CI, reviewed by the designer; plus a DOM/console assertion (no uncaught exception, no `console.error`) rather than pixel baselines. |
| `rust-toolchain.toml` pin | A pinned Node version in CI plus `engines`/`.nvmrc`; the exact command list lives in `CONTRIBUTING.md`. |
| `tempfile` dev-dependency for workflow tests | The project's tmp-dir/isolated-store helper for tests; no test may depend on the developer's real database or home directory. |

Three sentences carry the whole translation:

1. **Logic tested headlessly outranks UI code** — if a behaviour can only be
   checked by looking at the screen, it is in the wrong layer.
2. **The gates are named, blocking, and run in CI on every supported OS** — a
   green local run is not the evidence; the CI run is.
3. **Nothing heavy runs on the client machine** (K.11.8, guide §4) — push and
   use the CI artefact, whatever the language.

### K.15 Reminder: no `cargo` locally

Never run `cargo build`, `cargo test`, `cargo clippy`, `cargo fmt`, or any
other resource-intensive toolchain command on this machine. All Rust
toolchain work runs in GitHub Actions via `~/.config/opencode/scripts/ci.sh`.

---

## M. Demolition findings and hard enforcements (2026-10-26)

This section records weaknesses found by adversarial review of Sections A–L and
turns them into **binding rules**. Agents must obey these even when a faster
shortcut is tempting.

### M.1 What was still wrong after Section K

| Finding | Risk | Hard enforcement |
|---------|------|------------------|
| Core/UI split is advice only | Agents bury logic in `update()`; tests become impossible | **v0.1.0 blocked** unless `crates/*-core` (or equivalent) exists with ≥1 workflow test that does not reference UI crates |
| Free models write shallow tests | CI green but features hollow | PR checklist: name the workflow test file/function that proves the Task; reviewer rejects “smoke only” |
| `clippy -D warnings` from day 0 | Scaffolding stalls | K.12 — one PR grace max, then blocking |
| No toolchain pin | CI vs agent version skew | **Required** `rust-toolchain.toml` (stable) before foundation “done” |
| `tempfile` assumed | Copy-paste tests fail to compile | **Required** dev-dep on core when workflow tests land |
| GUI imports in core tests | Linux CI fails without display | K.13 — core tests headless only |
| Agent runs cargo on client PC | Freezes slow laptop | Explicit ban in K.11 / session prompt; use Actions artefacts only |
| Dual prompts (I vs L) | Wrong prompt pasted | **I superseded; L is canonical** |
| No license/dep hygiene | Incompatible crates slip in | Before first public release: run `cargo tree`; record licences in ATTRIBUTION; prefer `cargo-deny` in CI when practical |
| No flaky-test policy | CI becomes noise | Tests must be deterministic (no sleep races, no real network); network tests mock or `#[ignore]` + issue |
| Update-check / crash tests hit network | Flaky CI | Pure functions + mocks only in default `cargo test` |
| Workspace too clever | Free models get lost | Prefer simple workspace: one app + core + ≤2 shared crates until v0.3+ |
| README assumes Rust installed | Client cannot try app | README “Try without Rust” section required at v0.1.0 |
| SmartScreen / Gatekeeper | Client thinks app is broken | Release notes mention OS trust prompts; not a code defect |
| ROADMAP % without tests | Fantasy progress | “Verified” requires linked CI workflow test or explicit client sign-off note in PROGRESS |

### M.2 Structural gate for v0.1.0 (Rust) — checklist

Agent may tag `v0.1.0` only when **all** are true:

- [ ] `rust-toolchain.toml` present  
- [ ] `crates/*-core` (or same-layout equivalent) exists  
- [ ] UI crate depends on core; core does **not** depend on UI  
- [ ] ≥1 headless workflow test (save/load or equivalent) in core  
- [ ] CI: fmt, clippy (blocking or documented one-PR grace), `cargo test --workspace`, release build, artefact upload  
- [ ] README: download artefact, run app, no Rust required  
- [ ] LICENSE + ATTRIBUTION started  
- [ ] Client has been given try-steps after a **green** CI run  

### M.3 PR checklist additions (Rust)

Append to `.github/PULL_REQUEST_TEMPLATE.md`:

```markdown
## Rust QA
- [ ] Logic lives in core (or justified exception)
- [ ] Workflow/regression test added or updated (name: `...`)
- [ ] Tests are headless (no GUI / display)
- [ ] No full release build was run on the client laptop
- [ ] CI green on this branch
```

### M.4 Dependency and licence hygiene

- Prefer small, well-known crates; justify each new dep in `DECISIONS.md` if non-trivial.  
- Before **first public** release beyond personal testing: ATTRIBUTION lists third-party licences.  
- Optional CI: `cargo deny check` when `deny.toml` is maintained — do not block v0.1.0 on perfect deny setup.

### M.5 Flaky and network policy

- Default CI tests: **no real network**, no wall-clock sleeps used as sync.  
- Crash-report / update-check: test scrubbing and version compare in pure unit tests; HTTP behind mock or ignored integration tests.  
- If a test is flaky twice, quarantine with issue link; do not keep red/flaky main.

### M.6 What autonomous QA still cannot do

- Judge visual polish, layout taste, or “feels premium.”  
- Guarantee free models implement the *right* product.  
- Replace client try-steps on real OS (SmartScreen, DPI, trackpad).  
- Prove absence of all bugs — only presence of tested behaviours.

### M.7 Net verdict (post-demolition)

The addendum is a **strong process for a non-developer client + Rust + CI**,
not an autonomous company.

| Goal | Realistic? |
|------|------------|
| Logic/regression QA without the client | **Yes**, if core split + workflow tests hold |
| Zero client involvement | **No** |
| Craft-depth product on free models alone | **No** |
| Installable, progressively improving Rust app | **Yes** |

Agents that violate M.2–M.5 are failing the process even if they produce code.

### M.8 Structural gate for non-Rust v0.1.0 — checklist (added 2026-10-27)

The analogue of M.2 for a TypeScript/web app. The agent may tag `v0.1.0` only
when **all** are true:

- [ ] A domain/service layer exists and the UI depends on it; the UI holds no
      business logic (K.3, K.14)  
- [ ] ≥1 headless workflow test exercising real persistence, runnable with no
      display (K.5, K.13 translated)  
- [ ] CI runs: formatter **check**, lint with **zero warnings allowed**, type
      check, orphan check, headless tests on Linux/Windows/macOS, release
      build, artefact upload (K.6, K.14)  
- [ ] Node version pinned in CI and in the repo (K.11.6, K.14)  
- [ ] A test-isolation helper (tmp directory or isolated store) is used by
      workflow tests (K.11.4, K.14)  
- [ ] `CONTRIBUTING.md` documents the exact local build/test commands and
      names CI as authoritative for the maintainer's machine (guide II.26.1),
      while the app itself needs no toolchain to try (K.11.5)  
- [ ] README explains how to install the CI artefact — no compiler or SDK
      required for the client  
- [ ] LICENSE + ATTRIBUTION started; licence gate and SBOM present in CI  
- [ ] Client has been given numbered try-steps after a **green** CI run, and
      every client `BROKEN` report became a regression test before the fix
      (K.4.5, K.9)

The PR checklist addition (M.3) reads the same in a non-Rust repo, with
“core” meaning the domain/service layer:

```markdown
## QA (non-Rust)
- [ ] Logic lives in the service/domain layer (or justified exception)
- [ ] Workflow/regression test added or updated (name: `...`)
- [ ] Tests are headless (no display, no real network)
- [ ] No full release build was run on the client machine
- [ ] CI green on this branch
```

---

## N. Process feedback from agents (allowed, bounded)

Agents **should** report friction in the guide/addendum when it blocks
productive work. Agents **must not** treat feedback as permission to skip
non-negotiables.

### N.1 Where to write feedback

Prefer **public repo** file (so it survives sessions):

- `docs/process-feedback.md` — append-only log

Alternatively: GitHub Issue with label `process-feedback`.

**Never** edit local `guide.md` or `addendum.md` unless the client explicitly
asks in that session.

### N.2 Template (required shape)

```text
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

### N.3 Rules

1. At most **one** feedback entry per session unless work is fully blocked.  
2. Prefer **finish the Task** with a workaround; log feedback; do not stop to
   redesign process.  
3. Feedback is **advisory** until the client promotes it into the local
   guide/addendum.  
4. Suggestions that weaken safety are logged then **ignored in practice**:
   - clean-room / no RE / no proprietary paste  
   - no push of `guide.md` or `addendum.md`  
   - v0.1.0 structural gate (including Rust M.2)  
   - CI green before asking client to try  
   - no secrets or bot tokens in git/binaries  
   - human gate on research notes  
5. Client reviews at milestones (e.g. after v0.1.0, v0.2.0) and merges only
   repeated, clear wins.

### N.4 Good vs bad feedback examples

**Good:** “Primary OS is Windows but free Actions minutes exhausted; suggest
documenting artefact retention days and a fallback path.”  
**Bad:** “Drop workflow tests so we can tag v0.1.0 faster.”

---

## O. Priority rules card (context-scarce models)

When context is full, obey this card over long prose:

1. Client is non-developer — plain language, numbered try-steps.  
2. Never push `guide.md` / `addendum.md`.  
+3. No RE / no proprietary materials / research = summaries + URLs.  
+4. No v0.1.0 → only §6.1 + Rust M.2.  
+5. Rust: testable `*-core`; headless `cargo test`; CI not client laptop.  
+6. CI green → then ask client to try.  
+7. BROKEN → regression test → fix.  
+8. Process friction → Section N feedback; do not waive safety.  
+9. Honest release notes; no Craft-parity claims without evidence.  
+10. Rust GUI/UX → II.30: match the stack (no React/CSS design system in a
+    native app), tokens + contrast test + keyboard + empty/error states, and
+    rendered evidence; never a design claim without it.

---

## L. Session start prompt (canonical — paste for OpenCode)

```text
You are the tech lead and app designer for a non-developer client.
Read local `guide.md` and `addendum.md` if present (never commit them).
Read all docs/*, ROADMAP, CHANGELOG, PROGRESS, DECISIONS, docs/process-feedback.md in the repo.
Discover free models; update `docs/current-model-map.md`.
Ensure GitHub MCP and Context7 are configured; instruct client in plain steps if not.
Obey addendum Section O priority card when context is tight.
If this is a Rust app, follow Sections K and M: testable `*-core`; headless `cargo test` + clippy in CI; `rust-toolchain.toml`; no GUI in core tests; no heavy local builds on the client laptop.
Any GUI/UX work on a native Rust app follows II.30: detect the toolkit and its resolved version first, match the stack, keep one token source, test contrast, cover empty/error/loading states and keyboard paths, and prove it with rendered or AccessKit evidence. Never add React/StyleX/Astryx or a CSS design system to a native GUI, and never claim a design improvement without that evidence.
If this is NOT a Rust app, follow K.14 and M.8: logic in a service/domain layer; headless workflow tests; blocking format/lint(zero warnings)/typecheck/test gates on every supported OS; pinned toolchain; no heavy local builds.
If v0.1.0 is not tagged, work ONLY on guide.md §6.1 and addendum M.2 until it is.
Otherwise pick the next ROADMAP item; write a Task brief; get client approval for non-trivial work.
Implement real behaviour; push; CI; PR with template; give client exact try-steps only after CI is green.
On client BROKEN: add regression test, then fix.
If blocked by process, append docs/process-feedback.md using Section N template; do not violate non-negotiables.
No proprietary reverse-engineering; public research = summaries + URLs only; human gate on research.
No secrets in git; no bot tokens in binaries; no unbounded local builds.
Speak in client language. Propose choices when blocked.
End session with: what shipped, what client should try, what is next, any process feedback logged.
```

**End of Appendix A.**
