# Current Model Map - Dust Trail

## Guide Information
- **Guide version:** 2026-11-22 (`~/guide.md`, 1,990 lines; supersedes the 2026-11-19 edition)
- **Sections read:** Part I (§0–§18), Part II (§II.18–§II.30), and §19–§25 — read in full this session
- **Repo AGENTS.md sync state:** Synced 2026-11-22 — repository `AGENTS.md` re-synced line-for-line from the 2026-11-22 guide (1,975 lines).

## Active Free Models
- **Interactive session model (2026-11-22):** `z-ai/glm-5.3-flash` — the session was switched mid-run by the runtime (earlier turns ran on `minimaxai/minimax-m3.1-flash` and `nvidia/cosmos-reason2-8b`)
- **Parallel subagents:** attempts to spawn pinned NVIDIA subagents (`nvidia/nemotron-3.5-super-vl-120b-a12b`) were refused in this session ("child LLM route not allowed for this Session"); compile-fix work was done in-session instead
- **Note:** guide §5.2 (2026-11-22) is a two-tier policy — DSH free models first, NVIDIA free via API second. The previously recorded OpenRouter Tier-3 is no longer sanctioned by the guide and has been removed from the repo ruleset.

## Token Accounting Method
- **Source:** local OpenCode SQLite database (`session_v2` + `session_message` tables)
- **Bucketing:** per-message timestamps bucketed against release tag timestamps
- **Attribution rule:** a session counts as Dust Trail work only if its first user message names the Dust Trail project/repo, or it has ≥5 tool references to the Dust Trail repository path; sessions explicitly naming another project are excluded
- **Timezone:** the machine's working timezone (UTC+10:30, ACDT). Release tag timestamps are local times, so the same offset is applied to them. The genesis session's start time in that offset matches the known project anchor, which is what confirmed the offset.
- **Validation result:** per-message grand total 2,525,965,249 vs `session_v2` grand total 2,532,252,322 (delta 6,287,073 = 0.248%, worst per-session delta 5,874,604) across 35 included sessions and 28 excluded
- **Ledger:** `docs/tokens.md` (human-readable) and `docs/tokens.json` (machine-readable) are generated programmatically from the database, never hand-written

## Public Research Method
- Library/API documentation lookups use Context7
- No proprietary material, decompilation, or reverse-engineering output has been or will be used (clean-room rule)

## MCP / Tooling Status
- Context7: configured and available
- GitHub access: via the `gh` CLI, authenticated as the Dev account `Perchemop`

## Sync Record
- **Sync date:** 2026-11-22 (guide 2026-11-19 → 2026-11-22)
- **Guide changes absorbed into repo AGENTS.md:**
  - Runtime renamed OpenCode → DeepSeek Harness (DSH); `.opencode/` layout → `.dsh/` layout; `ci.sh` path → `~/.config/dsh/scripts/ci.sh`; plugin → `dsh-ci-only`
  - §4: new rule — agent must block on `ci.sh run` until the final `=== CI RESULT: … ===` line before reporting
  - §5.2: two-tier free-model policy (OpenRouter tier and Groq note removed from the guide)
  - §5.3: Designer role added (`~/.config/dsh/agents/`; II.27)
  - §5.6: DSH Agent Workflow Protocol added (Smart Zone <150k, /new between phases, Cases 1–5, seven commands in `.dsh/command/*.md`, TDD via ci.sh with red-run CI-ID rule, reconciliation table §5.6.5)
  - §5.7: Multi-agent protocol added (Case 4 only, 2–5 teammates, one Orchestrator, disk task list, allowlist permissions, graceful shutdown, reconciliation §5.7.5)
  - §7.2: update UX restructured — canonical in-app banner/dialog vs secondary fallbacks (previous duplication resolved in the guide)
  - §8.3: PR template gains "PR opened by the Dev account" checkbox
  - §9.3: required checks now include the PR-author check (II.26.7) and secret + personal-data scans (II.29.3)
  - §12: repo tree now `.dsh/` (7 command files, skill/, settings.local.json), `contributors/people.toml`, `LICENSE-MIT`/`LICENSE-APACHE`
  - §13–§17: new stress-test rows, DSH session prompt, binding Rust fallback summary
  - II.26: restructured as "Open questions" (II.26.7 = Dev-identity rule); II.27 Designer role; II.28 naming conventions; II.29 personal-data/secrets scan now six surfaces; II.30 Rust GUI/UX (17 subsections)
  - §19: DSH Agent Skills (reference catalog, no runtime-specific installs); §20 anti-patterns; §21 UI patterns; §22 pstack (reference only); §23 config examples; §24 token/contributor schemas; §25 six catalogs + 7 use rules
- **Corrections:** "§19–§26" in the previous record was wrong — the guide has no §26; sections are §19–§25.
- **Known repo gaps (to record in docs/gaps.md):** `.ci-required.txt` lacks the pr-author and personal-data-scan checks; `.dsh/` layout, `.gitleaks.toml`, `contributors/people.toml`, `docs/team-log/`, `docs/reference/`, `docs/gaps.md` not yet created; machine still has `~/.config/opencode/` (no `~/.config/dsh/`) — ci.sh runs via the legacy path until the DSH shim exists.
