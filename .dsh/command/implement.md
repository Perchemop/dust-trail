---
agent: coder
---

# /implement — one-ticket execution (guide §5.6.3)

Execute in a `/new`, single-ticket session.

1. Read the assigned ticket file from `docs/tickets/`.
2. Execute the `/tdd` loop for each acceptance criterion (each phase verified through `ci.sh`).
3. Run linters, type-checkers, and the full test suite via `ci.sh run "implement: <ticket>"` — not via raw bash.
4. Run `/code-review`.
5. Stage and create a clean commit via `ci.sh` (asserts the Dev identity per II.26.7). Direct `git commit` outside `ci.sh` is forbidden.
6. Open or update the PR via `ci.sh run`; merge only once required checks are green (Dev account).
