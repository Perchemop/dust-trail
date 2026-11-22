---
agent: coder
---

# /tdd — test-driven loop (guide §5.6.3, reconciled by §5.6.4)

Never write production code without a failing test. Never alter tests to pass buggy implementation code.

- **Red Phase:** write a unit or integration test for a single behaviour, then verify the test fails via `ci.sh run "TDD red: <behaviour>"`. The CI run must show the new test failing for the expected reason.
- **Green Phase:** write the minimum production code required to pass, verified via `ci.sh run "TDD green: <behaviour>"`.
- **Refactor Phase:** remove duplication and refine types while keeping all tests passing, verified via `ci.sh run "TDD refactor: <behaviour>"`.
- Batch up to 5 red tests per ticket per red run (§5.6.4). A batched "TDD" claim without a recorded red-run failure is a defect: CHANGELOG and PROGRESS cite the red run's CI run ID.
