# Decisions

All product and process decisions are recorded here. A decision is never
revisited without a new decision entry.

## 2026-11-14 — Product name: "Dust Trail"

**Decision:** The app is named "Dust Trail". The name is original and does not
reference "Red Dead Redemption" or any Rockstar Games trademark, logo, or trade
dress, in accordance with guide Part I §2.2 and Part II §19.2.

**Rationale:** Clean-room rule requires an original product identity. The
Western theme (frontier, outlaw, sheriff, horse, dust) is a genre, not a
protectable expression; the specific name, world, art style, and mechanics are
original.

**Open item:** Trademark clearance (WIPO / registry search) is pending before
the first public release. If a conflict is found, the name will be changed via
a new decision entry.

## 2026-11-14 — Dual frontends: TUI (crossterm) + GUI (egui/eframe 0.37)

**Decision:** Ship both a terminal TUI (default build) and an egui GUI
(feature-gated behind `gui`), both rendering the same domain `Frame`.

**Rationale:** The user requested "tui and gui". ASCII art renders naturally in
a terminal; the GUI mirrors it in monospace. Sharing one domain keeps logic
consistent and testable.

## 2026-11-14 — Licence: MIT OR Apache-2.0

**Decision:** Dual licence MIT OR Apache-2.0, per guide Part I §2.2 item 9.

## 2026-11-14 — Single save slot

**Decision:** v0.1.0 uses a single save slot (`savegame.json` in the config
dir). Multiple save slots are planned for v0.2.0.
