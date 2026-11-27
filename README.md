# Dust Trail

Dust Trail is a free, original Western-themed adventure game rendered entirely
in ASCII. You play as "Dust," a lone drifter in a frontier town, moving through
a top-down ASCII world, talking to the sheriff, chasing down the outlaw,
collecting loot, and saving your progress.

Dust Trail runs as a desktop window (GUI). The terminal interface and its
`crossterm` dependency were removed in v2.0.0, so the window is the only
frontend.

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
cargo run            # opens the desktop window (GUI is the default feature)
cargo run -- --gui   # same, with the window explicitly forced
```

The app opens a desktop window; close it with the window's close button. The
retired `--tui` flag is rejected with an error explaining that the terminal
interface was removed in v2.0.0.

## Project status

This is a work in progress built in public, following the
[guide](~/guide.md) phased release process. See `ROADMAP.md` for milestones
and `CHANGELOG.md` for release history. The first functional test build is
v0.1.0.

## Licence

Dust Trail is dual-licensed under **MIT OR Apache-2.0**. Third-party
dependencies are listed in `ATTRIBUTION.md`.

## Privacy

Dust Trail includes an optional crash/bug reporting feature. When a panic
occurs, a scrubbed report is parked locally and shown on the next launch.

**How it works:**

1. **On panic:** A crash report containing your app version, OS, Rust version,
   and a scrubbed backtrace is written to a temporary file.
2. **On next launch:** The report is displayed for your review before any
   submission. You must explicitly choose to send it.
3. **Scrubbed:** All absolute file paths (e.g., `/tmp/test_user_123/project`) and
   sensitive values (tokens, API keys, passwords) are removed from the report.

**Submission options:**

- **With a GitHub token:** The report is submitted as a new issue to the
  [dust-trail issues page](https://github.com/Perchemop/dust-trail/issues).
  Your token is used only for this submission and is not stored locally.

- **Without a token:** A pre-filled issue URL opens in your browser, where
  you can submit manually.

**No auto-send:** Nothing leaves your machine without your explicit consent.
Reports are only submitted when you click "Send to GitHub".

## Contributing

See `CONTRIBUTING.md`. All work lands on `main` via a reviewed pull request
with CI green; no direct pushes. This project follows the clean-room rules in
`guide.md` — no proprietary source or assets are used.
# CI trigger
# Force rebuild
