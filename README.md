# Dust Trail

Dust Trail is a free, original Western-themed adventure game rendered entirely
in ASCII. You play as "Dust," a lone drifter in a frontier town, moving through
a top-down ASCII world, talking to the sheriff, chasing down the outlaw,
collecting loot, and saving your progress.

It is playable two ways:

* **TUI** — in your terminal (the default build).
* **GUI** — as a desktop window (`cargo run --features gui -- --gui`).

No commercial trademarks, logos, or assets are used. The world, art style, and
mechanics are original.

## What it does

* Explore a frontier town and surrounding wilderness drawn in ASCII.
* Move with WASD / arrow keys (or hjkl).
* Talk to the sheriff and the outlaw (press E).
* Collect loot and a horse.
* Save and load your game.
* Capture the outlaw to win.

## Install and run

Dust Trail is a Rust application. Builds are published on each release via
GitHub Actions; you can also build from source.

### From source

```bash
git clone https://github.com/Perchemop/dust-trail.git
cd dust-trail
cargo run               # TUI (terminal)
cargo run --features gui -- --gui   # GUI desktop window
```

The TUI runs in your terminal; quit with `Q` or `Ctrl-C`.

## Project status

This is a work in progress built in public, following the
[guide](~/guide.md) phased release process. See `ROADMAP.md` for milestones
and `CHANGELOG.md` for release history. The first functional test build is
v0.1.0.

## Licence

Dust Trail is dual-licensed under **MIT OR Apache-2.0**. Third-party
dependencies are listed in `ATTRIBUTION.md`.

## Contributing

See `CONTRIBUTING.md`. All work lands on `main` via a reviewed pull request
with CI green; no direct pushes. This project follows the clean-room rules in
`guide.md` — no proprietary source or assets are used.
