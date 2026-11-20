# Token usage ledger - Dust Trail

Measured token consumption for the Dust Trail project, one row per release.

## How these numbers are measured

Every figure in this file comes from the project's own OpenCode session
database. Nothing here is estimated, rounded up, or copied from a vendor
claim.

- **Source of truth:** the local OpenCode SQLite database (`session_v2` for the
  session-level totals, `session_message` for the per-message usage records).
- **Attribution rule:** a session counts as Dust Trail work when its first user
  message names the Dust Trail project or repository, or when it contains at
  least five tool references to the Dust Trail repository path. Sessions that
  explicitly name another project are excluded. The latest measurement included
  35 sessions and excluded 28.
- **Release attribution:** tokens are bucketed by the timestamp of each
  individual message against the release tag timestamps, not by session start
  time. The genesis session spans several days, so session-level bucketing
  would put its whole total in the wrong release.
- **Timezone:** the machine's working timezone (UTC+10:30, ACDT). Release tag
  timestamps are local times, so the same offset is applied to them.
- **Cross-check:** the per-message total is compared against the independent
  `session_v2` session-level total. The latest measurement differed by
  6,287,073 tokens, or 0.248%, which is the cost of records
  that carry a session total but no per-message token breakdown. The largest
  single-session difference was 5,874,604 tokens.
- **A note on the newest row:** the `v1.5.1` row is a snapshot taken before the
  release commit. Work that continued after the snapshot is not counted there,
  so the figure is a lower bound for that release.

Measured at 2026-11-20T06:44:10.440706+10:30.

## Ledger

| Version | Date       | Tokens this release | Tokens cumulative | Notes                           |
|---------|------------|---------------------|-------------------|---------------------------------|
| v0.1.0   | 2026-11-15 |         160_734_746 |       160_734_746 | First functional build          |
| v0.2.0   | 2026-11-15 |         122_405_803 |       283_140_549 | 3D world, GUI window, save/load |
| v0.3.0   | 2026-11-16 |         470_759_088 |       753_899_637 | Bounties, decay, wanted levels  |
| v0.4.0   | 2026-11-16 |         207_213_976 |       961_113_613 | Item system and inventory       |
| v1.0.0   | 2026-11-18 |       1_013_643_893 |     1_974_757_506 | First stable release            |
| v1.1.0   | 2026-11-18 |          42_556_446 |     2_017_313_952 | Multi-save slots                |
| v1.2.0   | 2026-11-18 |          46_601_134 |     2_063_915_086 | Combat system                   |
| v1.3.0   | 2026-11-18 |          30_161_368 |     2_094_076_454 | UI polish and item system       |
| v1.4.0   | 2026-11-18 |          37_820_864 |     2_131_897_318 | Inventory UI                    |
| v1.5.0   | 2026-11-18 |         109_096_393 |     2_240_993_711 | Smarter AI and inventory UI     |
| v1.5.1   | 2026-11-20 | **PENDING**         | **PENDING**       | Release infrastructure slice; pending CI run 44701830027 |

## Totals

- **Total tokens since the project began (verified):** 2,240,993,711 (through v1.5.0)
- **Tokens in the current release (v1.5.1):** **PENDING** — awaiting next green CI run after fixing the latest failed CI run

## Verified measurement sources

The above numbers through v1.5.0 are cross-checked against measured OpenCode session data. The v1.5.1 row cannot be finalized until after the next green CI run (run ID pending) and subsequent release publication.

## Composition of the total

| Component    | Tokens         |
|--------------|----------------|
| Cache reads  |  2,461,686,041 |
| Input        |     60,029,135 |
| Reasoning    |      2,640,616 |
| Output       |      1,609,457 |
| Cache writes |              0 |

The bulk of the total is cache reads. That is the cost of repeatedly sending
the project's own growing context back to the model between turns, not the cost
of generating new content.

## Reproduction

The measurement is produced by a read-only Python script over the OpenCode
database (`file:...?mode=ro`). The script and its intermediate output are not
committed; the method above is enough to reproduce the figures.
