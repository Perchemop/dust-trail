# Progress

## Session 2026-11-14 — Initial scaffold

- Created `dust-trail` repo scaffold: workspace, shared crates
  (`crash-report`, `update-check`), app (`apps/dust-trail`).
- Domain layer: `color`, `cell`, `frame`, `tile`, `world`, `player`, `game`,
  `save` — pure, headless, with unit tests (movement, blocking, loot
  collection, save/load roundtrip, sheriff interaction).
- TUI frontend (`crossterm`): alternate screen, raw mode, per-frame render,
  WASD/hjkl/arrows, E interact, S save, L load, R restart, Q quit.
- GUI frontend (`egui/eframe 0.37`, feature-gated): toolbar (New/Save/Load/Quit),
  monospace ASCII grid with per-cell colour runs, keyboard input.
- `main.rs`: wires crash-report panic hook, parks/reviews parked reports,
  quiet fail-soft update-check stub; defaults to TUI, `--gui` launches GUI.
- CI: `ci.yml` (fmt, clippy, tests default+gui, release builds, secret/personal
  data scan, multi-OS build matrix, supply-chain, packaging-plan),
  `release.yml` (multi-OS build + draft release + SHA256SUMS).
- Tracking: README, CHANGELOG, ROADMAP, ATTRIBUTION, AGENTS.md, DECISIONS.md,
  `.ci-required.txt`, `dist-workspace.toml`, PR template, issue templates.
- Created GitHub repo Perchemop/dust-trail (public).

## Next
- `ci.sh run` to verify CI green.
- Fix any CI findings.
- Tag v0.1.0 and publish first release.
- Expand ROADMAP; human-gated research for v0.2.0.

## Session 2026-11-14 — v0.1.0 Released

- Tagged v0.1.0 at commit 373f0b7 (main HEAD)
- CI green on main push: check, Build (windows-x86_64), Build (macos-arm64),
  Build (linux-arm64), supply-chain
- Installer artifacts built by CI on 3 platforms:
  - dust-trail-linux-arm64.deb
  - dust-trail-macos-arm64.dmg
  - dust-trail-windows-x86_64.msi
- Release v0.1.0 published on GitHub:
  https://github.com/Perchemop/dust-trail/releases/tag/v0.1.0
  Assets: .deb, .dmg, .msi, SHA256SUMS.txt
- Release notes from CHANGELOG.md v0.1.0 section
