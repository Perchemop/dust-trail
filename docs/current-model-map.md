# Current Model Map - Dust Trail

## Guide Information
- **Guide version:** 2026-11-19 (`~/guide.md`)
- **Sections read:** Part I (§0–18), Part II (§II.18–§II.30), Appendix A–O, and §19–§25 — read in full
- **Repo AGENTS.md sync state:** Synced 2026-11-19 — the repository's `AGENTS.md` now matches the machine-global version.

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

## Sync Record
- **Sync date:** 2026-11-19
- **Changes made:** Updated repository `AGENTS.md` to match machine-global version 2026-11-19, incorporating:
  - Updated sync record preamble
  - Section II.26.7: Dev identity rule for PR/merge authorship
  - Section 5.6: OpenCode Workflow Protocol (Smart Zone, Case 1-5 matrix, `/tdd`, `/implement`, etc.)
  - Section 5.7: Multi-Agent Architecture protocol (Case-4-only teams, on-disk team logs, allowlist-only permissions)
  - Section 9.3: Updated branch protection checklist with specific required checks
  - New sections 19-25 (skills catalog, dead code prohibition, design guidelines, pstack, etc.)
  - Section 12: Suite-wide Standards and Parallel Agents protocol
  - Sections II.26-II.30: Dev identity, personal data/secrets, Rust GUI/UX, supply chain, token metrics
  - Addendum A-O: Merged client mode, MCP, phases 0-8, etc.