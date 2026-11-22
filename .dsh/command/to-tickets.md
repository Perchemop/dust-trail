---
agent: coder
---

# /to-tickets — ticket generation phase (guide §5.6.3)

Parse the specification and split it into atomic, sequential work units sized to fit inside the Smart Zone (< 150k tokens each).

- Ticket schema (saved to `docs/tickets/<id>.md`): Objective, target file paths, blocking dependencies, verifiable acceptance criteria.
- Ready-for-work: a ticket is ready only when its dependencies are merged to `main` and its acceptance criteria are objectively checkable by CI.
