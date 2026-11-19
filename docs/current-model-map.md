# Current Model Map - Dust Trail

## Guide Information
- **Guide version:** 2026-11-19 (`~/guide.md`)
- **Sections read:** Part I (§0–18), Part II (§II.18–§II.30), Appendix A (sections A–O), and §19–§26 — read in full
- **Repo AGENTS.md sync state:** stale — the repo's own `AGENTS.md` appendix records "Guide version: 2026-11-16", which is behind the 2026-11-19 guide. Recorded as process friction. `AGENTS.md` was NOT edited (not this agent's file to touch).

## Active Free Models
- **Interactive session model:** `minimaxai/minimax-m3.1-flash` (provider `nvidia`)
- **Parallel subagents:** all pinned to `nvidia/nvidia/nemotron-3.5-super-vl-120b-a12b` per the owner's explicit directive ("use nvidia models for subagents")
- **Reason:** the OpenCode free-tier default `opencode/deepseek-v4.1-flash` reached end-of-life on 2026-11-16; four earlier subagents died with that EOL error before the pin was applied.

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

## Readiness Note
- Latest published GitHub Release: v1.5.0, which shipped **zero assets**. Two independent causes, both confirmed from the workflow logs: the release workflow run `44296170358` failed in all four build jobs on the `ItemKind` compile error (fixed in PR #24, commit `0f68267`), and the duplicate `publish.yml` workflow failed on an unquoted `on:` YAML key (file deleted).
- The tagged v1.5.0 commit `f6ae559` also had a **red** CI run (`44295856578`). The tag should not have been pushed.
- The fix and the honest v1.5.1 release are in flight on branch `opencode/ci`.
