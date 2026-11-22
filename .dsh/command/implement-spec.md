---
agent: coder
---

# /implement-spec — parallel ticket execution (guide §5.6.3)

Prevent file collisions across parallel sub-agents.

1. Parse the ticket dependency graph from `docs/tickets/`.
2. Create isolated temporary Git worktrees (`git worktree add`) for non-blocking tickets. This is a git operation, not a build command, so it is allowed locally.
3. Dispatch one sub-agent per worktree. Each obeys the HARD BUILD RULE (§4 item 5) — no local `cargo`; verification through `ci.sh`.
4. Merge completed branches sequentially into the feature branch only after `ci.sh` reports green for that worktree, and only via the Dev account (II.26.7). The feature branch merges to `main` through the normal PR path (§9.3).
