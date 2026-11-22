PROCESS FEEDBACK
Date / session: 2026-11-17
What I was trying to do: Implement deterministic procedural map generation for Dust Trail per guide requirements
Block or friction: None - guide provided clear §6.1 v0.1.0 gate and §13 stress test mitigations
Why the current guide/addendum caused it: Clear requirements on deterministic generation, connectivity guarantees, and performance targets
Suggested change: None needed - implementation matches requirements
Severity: blocks-release | slows-work | nice-to-have
Workaround I used: None needed
Confirm: did NOT violate clean-room / v0.1.0 gate / no-secrets / CI-before-client-try / no meta-guide push

PROCESS FEEDBACK
Date / session: 2026-11-19, Dust Trail v2.0.0 planning session
What I was trying to do: spawn four parallel agents to map the TUI removal surface, the ASCII rendering pipeline, the production-readiness gaps, and the release workflow
Block or friction: all four subagents failed immediately with `The model 'deepseek-ai/deepseek-v4.1-flash' has reached its end of life on 2026-11-16T08:00:00Z and is no longer available`
Why the current guide/addendum caused it: the guide's §5.2 free-model policy and the failover tables still route several roles to `opencode/deepseek-v4.1-flash`, a model the provider has retired; the catalogue was not re-scanned after the retirement date
Suggested change: re-scan the provider catalogue before spawning teams and prune retired model IDs from the per-role failover chains in `~/.config/opencode/failover.json`; the guide should state that a retired model in a chain fails the whole subagent call rather than falling through to the next entry
Severity: slows-work
Workaround I used: pinned every subagent explicitly to `nvidia/nvidia/nemotron-3.5-super-vl-120b-a12b` per the owner's directive; all four tasks then completed
Confirm: did NOT violate clean-room / v0.1.0 gate / no-secrets / CI-before-client-try / no meta-guide push

PROCESS FEEDBACK
Date / session: 2026-11-19, Dust Trail PR #24 merge
What I was trying to do: merge the green GUI compile-fix pull request through the documented CI helper
Block or friction: `ci.sh merge` fails with `ERROR: unknown command: merge (try: run, status, logs, diff, open)`. The installed helper at `~/.config/opencode/scripts/ci.sh` only implements `run`, `status`, `logs`, `diff` and `open`, while the guide's §4 and the repository's `AGENTS.md` both document a `merge` subcommand
Why the current guide/addendum caused it: the guide describes a newer version of the shared helper script than the one installed on this machine; the two were out of sync
Suggested change: make `ci.sh` print its supported subcommands when it receives an unknown command, and state in the guide that `ci.sh merge` is only available in helper versions newer than the one recorded in `docs/dev.md`; otherwise use `gh pr merge --merge` with the Dev credential
Severity: slows-work
Workaround I used: merged PR #24 with `gh pr merge 24 --merge --delete-branch=false`, which matches the repository's established merge-commit history (all prior merges are GitHub-authored merge commits through the `Perchemop` Dev credential)
Confirm: did NOT violate clean-room / v0.1.0 gate / no-secrets / CI-before-client-try / no meta-guide push
---

PROCESS FEEDBACK
Date / session: 2026-11-22 (Dust Trail, guide 2026-11-22 review + ruleset sync session)
What I was trying to do: delegate the CI compile-error fixes to a subagent, and later spawn any pinned-NVIDIA subagent, while running the legacy ci.sh verification loop.
Block or friction: (1) two subagent launches failed before doing any work ("failed before it finished... no closing message") with no diagnostic surfaced to the parent; (2) explicit pinned-NVIDIA routes (nvidia/nemotron-3.5-super-vl-120b-a12b) were refused with "child LLM route ... is not allowed for this Session", so subagent delegation was unavailable for the whole session; (3) the guide's documented `ci.sh merge` subcommand does not exist in the legacy shim on this machine (commands: run, status, logs, diff, open), so the PR merge had to go through `gh pr merge` directly.
Why the current guide/addendum caused it (or did not cover it): guide §5.7/§5.6 assume sub-agent dispatch and `ci.sh merge` are available; the runtime in this session allows neither, and the guide has no documented fallback for "delegation unavailable" or "shim lacks subcommand".
Suggested change (one concrete edit): add to §5.6.4 (or a short §5.6.8) a fallback rule — "if sub-agent dispatch is refused by the runtime, the Orchestrator executes the ticket in-session under the same verification mandate; if the local ci.sh lacks a documented subcommand, use the equivalent gh/git command from the Dev identity and record the substitution in PROGRESS.md."
Severity: slows-work
Workaround I used (if any): performed the compile fixes in-session; merged PR #27 with `gh pr merge 27 --merge` from the Dev account (`gh api user` = Perchemop) after `=== CI RESULT: SUCCESS ===`.
Confirm: did NOT violate clean-room / v0.1.0 gate / no-secrets / CI-before-client-try / no meta-guide push
