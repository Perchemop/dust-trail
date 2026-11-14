# Architecture

Dust Trail follows a domain-first, command-centric architecture (guide §3).

## Layers

```
gui (egui/eframe, feature `gui`)   tui (crossterm)
        \                           /
         +-------- domain ----------
         |  game.rs (turn logic)   |
         |  world.rs (grid+entities)|
         |  player.rs, tile.rs     |
         |  cell.rs, frame.rs      |
         |  color.rs (domain RGB)  |
         |  iso.rs (world->screen isometric projection) |
         |  save.rs (JSON save)    |
         +-------------------------+
              crash-report (panic hook)
              update-check (GitHub Releases API)
```

* **Domain** (`apps/dust-trail/src/domain`): pure Rust, no UI deps, fully
  serialisable with serde, testable headlessly. Produces a `Frame` (grid of
  `Cell`s) each turn.
* **TUI** (`src/tui`): renders a `Frame` to the terminal via crossterm.
* **GUI** (`src/gui`, feature `gui`): renders a `Frame` as monospace ASCII in
  egui/eframe, colour runs per cell.
* **theme** (`src/theme`, feature `gui`): single source of semantic colour roles
  (light/dark/high-contrast), contrast math, and the one allowed
  `Color32::from_rgb` conversion.
* **crash-report / update-check**: shared suite crates (guide §7).

## Data flow

Input → `Game::move_player` / `Game::interact` → turn advances → NPCs act →
`Game::frame()` → frontend renders `Frame`. Save/load via JSON
(`SaveData`).

## Persistence

Single save slot `savegame.json` in the OS config dir (directories crate).

## Clean-room

All mechanics, glyphs, colours, and the name "Dust Trail" are original. No
commercial trademarks, logos, fonts, or exact layouts (guide Part I §2.2).
