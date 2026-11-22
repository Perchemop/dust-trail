---
agent: plan
subtask: true
---

# /code-review — read-only audit (guide §5.6.3; the Reviewer role, §5.3)

Audit all code changes before committing or merging.

- **Axis 1 — Architectural alignment:** compliance with the specification document, `AGENTS.md` conventions, and separation of concerns.
- **Axis 2 — Tactical correctness:** off-by-one errors, unhandled null states, race conditions, missing test coverage.
- **Clean-room axis:** no proprietary source, assets, or RE material; public research notes are original summaries + URLs only (§2.2).
- **Secrets axis:** no tokens, keys, or credentials in the diff, the PR text, or the commit metadata (II.29).
