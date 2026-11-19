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