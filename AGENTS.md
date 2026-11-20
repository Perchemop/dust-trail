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

**Runtime (guide preamble, 2026-11-19).** This guide targets **OpenCode** as
the agent runtime. The only supported tool path is OpenCode plus the suite's
`.opencode/` layout (§12) and the `ci.sh` verification shim (§4). Other agent
runtimes (Claude Code, Cursor, Codex, Aider, etc.) are **not supported**: a
reference to one of them appears only when the guide is describing material
for adaptation, never as an installation target.

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

Guide §5.2 fixes the tiers; the operative rotation lives in config, not in this document. Record in `docs/current-model-map.md`.

1. **Tier 1 — `opencode/*` free models.** No API key, cheapest free quota. Always first.
2. **Tier 2 — `nvidia/*` free models** via API, reached only after tier 1 is exhausted.
3. **Tier 3 — `openrouter/*` free models**, reached only after tiers 1 and 2, via the
   owner-supplied OpenRouter free-tier key (owner-directed deviation from the guide's
   two-tier policy; recorded here). OpenRouter is a metered free quota: **50 free-model
   requests per day** on the shared limit, and adding credits lifts it to 1000 per day.
   Nothing is ever selected on a paid OpenRouter model.
4. Process unchanged across providers.
5. **Groq is not rotated.** Its catalog holds one beta chat model plus classifier
   and Arabic-only models, and nothing in the failover plugin switches a session
   *into* Groq. Treat it as a deliberate manual pin only.

### 5.3 Canonical Agent Roles

`.opencode/agents/`: Orchestrator (plan only), Coder (execute), Explorer/Researcher (public context), Reviewer (PR vs acceptance + clean-room). Read-only roles: deny edit/bash.

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

**Check mechanics**
- Query GitHub Releases API (`/repos/<owner>/<repo>/releases?per_page=30`) on a background thread; compare semver to the running version.
- **Quiet startup check, once per run:** a failure is logged, never shown as a modal; the app starts normally.
- **Help ▸ Check for Updates (explicit):** reports a visible status — Checking / Up to date / Update available / Failed (with the error). A check already running becomes the requested one.
- Fail soft offline; conditional GETs (ETag / If-None-Match); brief cache; handle GitHub's rate limit gracefully (tell the user, stop checking until reset).
- Semver rule: a dev/integration build (e.g. `0.17.0-integration.20261029.f5b3f123`) is **not** offered the release it already contains; a release is offered only when strictly newer.

**Startup prompt modal (the standard notification)**
When a newer release is found, show one standardized modal: app icon + accent colour, new version, release notes, and exactly four actions:
- **Update & Restart** — verify, install, then restart through the app's normal close path.
- **Download & Install** — discover the platform-specific installer from the release assets, download to temp, verify SHA-256 against `SHA256SUMS.txt` and the minisign signature, then offer to open the installer. Never auto-installs; the user always confirms. Gated on a verified signature (never offered on Failed or Missing signature state).
- **Download** — open the release page; only safe https web links are opened (an unsafe or malformed link falls back to the app's releases page, which must always be a safe link).
- **Later** — dismiss; an installation already in progress continues and reports through the updater.

**Persistent install requirement.** Every installed release must create a permanent app-drawer / start-menu entry so the app remains visible after installation: a `.desktop` file on Linux, the `.app` bundle in `/Applications` on macOS, and a Start Menu entry on Windows. A raw extracted binary that runs only from `Downloads` does not qualify as installed.

**Stale version remediation (before first launch / on update).** Upon detecting a newer release and before activating the staged update, the app must scan the app-drawer / start-menu directory for any previously installed versions of the same app and remove them completely. This ensures that:
- Only the current latest version appears in the app drawer / start menu.
- Old versions do not consume disk space or confuse the user.
- The update process is seamless: the user sees one entry, which is always the current version.
- This remediation step runs automatically — the user is never asked to manually remove old versions, nor directed to explore system folders or use package manager commands.
- If an old version cannot be removed (e.g. file-in-use, permission issue), the app logs the issue and continues with the new installation; the user is not interrupted or required to take any action.

**Self-update applicability (install-type detection)**
- Detect how the running copy was installed: macOS `.app`, Windows MSI / portable zip / standalone, Linux AppImage / deb / rpm / pacman / Flatpak / standalone.
- **Development builds never self-update.** A `cargo run`/build-tree copy (Cargo cache markers) gets the download link instead.
- **Package-manager-owned copies defer to their package manager** (Homebrew, MacPorts, Nix, scoop, winget, Chocolatey, Store, AUR) — in-place replacement would leave their records stale.
- Only a packaged copy in a supported format self-updates; anything else — and any unsupported platform — gets the **download link**, never a broken half-install.
- **Never replace a running app.** A restart-helper process waits for the app to exit, finishes anything the OS prevents while running (e.g. Windows locks a running exe), and starts the new version.
- **Staged, then renamed; verify, then activate.** Download → verify (SHA-256 + signature) → stage under a hidden name → rename into place → only then activate. A failed step removes the staged copy and leaves the running version untouched.
- **Unsaved-work gating:** an update restart waits while the app holds unsaved work ("Save or close your open work first; the update restarts the app").
- **Anti-loop marker:** a version just installed is recorded with a timestamp and not re-offered for 24 h, so a tag/binary version mismatch cannot loop update → restart → "update available".
- **Failure recovery:** a failed self-update writes its error to a log file shown once at the next start; the update stays on offer.

**Token & authentication for update-check / crash-report (guide §7.2, synced 2026-11-16).**
- **No privileged bot token** is ever embedded in client binaries.
- Optional user-supplied GitHub token (or device flow) may be used for higher rate limits or creating crash/bug issues.
- **Minimal scopes** only: `public_repo` or finer.
- **Persistence:** a stored user token must use the platform credential store / keychain (or an encrypted file with a machine-bound key) — never plain text in the app's normal config directory.
- Document the privacy implications in the README / privacy note.

**Signature verification (required, guide §7.2).** Every release ships `SHA256SUMS.txt` and a signature over that file (minisign per guide; this repo's recorded decision is a GPG-detached `.asc` — see `DECISIONS.md`). The public key is pinned in the update-check crate or a compile-time constant; key rotation requires a new app release carrying the new public key. Verification failure (missing/bad signature, hash mismatch) aborts the update and leaves the running version untouched. The current public key fingerprint is documented in `docs/update-signing.md` and in the release notes of any key-rotation release.

**Opt-out and localization**
- An env var / build flag **disables update checks entirely** for distro packaging (Nix, Flatpak, deb/rpm repositories) — the system package manager owns updates there.
- All updater messages (prompt, status, failure notes) follow the app's **interface language**, translated at draw time; network error text passes through as received.

**In-app update GUI (the user's only update path).** Every suite app must provide an **integrated, one-click update mechanism entirely within the application GUI**. The user must never be directed to a website, GitHub Releases page, command line, or package manager to initiate an update. The update flow is:

1. **Background check:** On startup (once per run), the app quietly checks GitHub Releases API for a newer version. If the app is up to date, no UI is shown.
2. **Subtle notification:** When a newer release is found, a small, non-intrusive banner/toast appears at the top or bottom of the main window — not a modal that blocks the app. It states: "New version available: X.Y.Z. Select to update."
3. **One-click update dialog:** Selecting the banner opens a standardized inline dialog (within the app's normal window, not a separate modal) with: Current version and new version; Release highlights (Added/Changed/Fixed, derived from release notes); Two buttons: **Update & Restart** and **Later**.
4. **Update & Restart:** The app performs the full update pipeline internally: Downloads the platform-specific installer to a temp location; Verifies SHA-256 against `SHA256SUMS.txt` and the minisign signature; Runs the stale-version remediation (removes old app-drawer entries); Stages the new version under a hidden name; Renames it into place; Restarts the app automatically (or waits for user launch). All of this happens without the user leaving the app, opening a browser, or running any commands.
5. **Later:** Dismisses the notification; an installation already in progress continues and reports through the updater.

**Never direct the user to GitHub or a browser for routine updates.** The only exception is if the update flow fails at step 4 after signature verification — in that case, a "Download manually from release page" link may be offered, but this is a failure path, not the normal flow.

**Platform-specific installer detection.** The app detects how it was originally installed (macOS `.app`, Windows `.msi`/portable zip/standalone, Linux AppImage / deb / rpm / pacman / Flatpak / standalone) and only offers self-update for packaged copies in supported formats. Development builds (`cargo run`) and package-manager-owned copies defer to their respective managers.

**Anti-loop marker.** A version just installed is recorded with a timestamp and not re-offered for 24 h, so a tag/binary version mismatch cannot loop update → restart → "update available".

**Signing-key handling and compromise procedure (guide 2026-11-17).** The minisign private key lives in a hardware token or a scoped secret accessible **only** to the release workflow — never on a developer machine. `SHA256SUMS.txt` is signed **after** artefact upload and its signature is attached to the release. If the private key is suspected compromised: (1) publish a new app release whose bundled public key is a newly generated pair, before any further artefact is signed with the old key; (2) mark the affected release(s) with a clear "Security" banner in the release notes and the in-app release-notes popup; (3) update `docs/update-signing.md` with the old fingerprint, the new fingerprint, and the reason; (4) rotate the secret in the release workflow. **Never silently re-sign existing artefacts with a new key** — that creates ambiguity and hides the incident.

**Failure recovery:** a failed self-update writes its error to a log file shown once at the next start; the update stays on offer.

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
├── .opencode/agents/
├── crates/
│   ├── crash-report/
│   └── update-check/
├── apps/{app-name}/
├── docs/
│   ├── project-brief.md
│   ├── scope.md
│   ├── inspiration.md
│   ├── research/
│   ├── design/
│   └── ...
├── ROADMAP.md
├── CHANGELOG.md
├── IMPLEMENTATION_PLAN.md
├── REQUIREMENTS_TRACEABILITY.md
├── DECISIONS.md
├── PROGRESS.md
├── ATTRIBUTION.md
├── LICENSE*
└── README.md
```

**Parallel-agent build hygiene.** When multiple agents work in the same workspace:
- Each agent uses a dedicated `CARGO_TARGET_DIR=target/agent-<name>` to avoid compiled artifact conflicts.
- New commands and logic go into **new modules** — never edit an existing module's code to add your feature.
- File ownership is strict: an agent may only edit files it "owns" (marked by a `OWNERS` file or agreed convention). Do not fix others' in-flight changes.
- `Cargo.toml` writes are atomic: modify a copy, validate with `cargo check --all-targets`, then replace the original. A broken manifest for one agent breaks the whole workspace.
- Each agent's target directory is capped at ~10 GB disk usage; if nearing the limit, clean or archive old builds before continuing.

---

## 13. Architectural Stress Test — Gaps Found & Mitigations

The following gaps were identified by adversarial review of this process. Each mitigation is now **in force**.

| Gap | Risk | Mitigation |
|-----|------|------------|
| Vague "first functional build" | Endless foundation without a shippable slice | **Hard v0.1.0 gate** (section 6.1) |
| No PR discipline | Agents merge incomplete or dirty work | **PR template + checklist** (section 8.3) |
| Release path learned too late | Broken tags/artefacts at the worst time | **CI + release.yml from day one** (section 4) |
| Research → code without review | Copyright bleed, UI cloning | **Human gate on research** (sections 2.3, 8.4) |
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

> Resume clean-room build via Chat → Task → PR. Read guide + addendum if present (never commit them). Read docs/*, ROADMAP, CHANGELOG, PROGRESS, process-feedback, capability-inventory, resource-catalog, specs/, tickets/, team-log/. Discover free models. If no v0.1.0, only §6.1 (Rust: testable core + headless tests + CI). CI green before client try-steps. Research = summaries + URLs; human gate. No RE, no secrets, no heavy local builds. Log process friction to docs/process-feedback.md without waiving safety. End with: shipped, client try-steps, next, any process feedback logged.

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

### II.24.3 Release / packaging

- Cross-platform build artefacts verified on every supported platform: each CI run attaches binaries plus the lockfile used for the build.
- App launches on a clean machine; save → reopen → identical state.
- Version bumped; `CHANGELOG.md` has a release entry.
- No secrets/keys committed; no local-only files in the tree.
- `LICENSE` present; `ATTRIBUTION.md` complete and accurate.
- Release assets (II.24.3) scanned for secrets and personal data; the SBOM and any installer carry no maintainer home path or personal identifier.

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
- **No open PR older than 30 days without a recent update comment; no draft PRs; stale labels triaged.**

---

## Appendix: Version & Sync Record

- **Guide version:** 2026-11-19 (`~/guide.md`)
- **Sections read:** Part I (§0–18), Part II (§II.18–§II.30), Appendix A–O, and §19–§26
- **Last sync:** 2026-11-19
- **Next sync required:** When guide.md is updated
- **Local AGENTS.md last updated:** 2026-11-19