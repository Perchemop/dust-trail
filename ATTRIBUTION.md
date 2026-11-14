# Attribution — Third-Party Components

Dust Trail itself is **dual-licensed MIT OR Apache-2.0** (`LICENSE`);
all first-party source code, tests, docs, and branding are original work.

## Direct dependencies

| Crate | Version | Licence | Use |
|---|---|---|---|
| `eframe` | 0.37 | MIT OR Apache-2.0 | GUI app framework / windowing (`gui` feature) |
| `egui` | 0.37 | MIT OR Apache-2.0 | Immediate-mode UI (`gui` feature) |
| `crossterm` | 0.27 | MIT OR Apache-2.0 | Terminal TUI rendering and input |
| `directories` | 6.x | MIT OR Apache-2.0 | OS config/data directories (saves, crash reports) |
| `serde` / `serde_json` | 1.x | MIT OR Apache-2.0 | Game save serialization |
| `thiserror` | 2.x | MIT OR Apache-2.0 | Error types |
| `tracing` | 0.1 | MIT OR Apache-2.0 | Diagnostics |
| `ureq` | 3.x | MIT OR Apache-2.0 | HTTP client for update checks (sync) |
| `semver` | 1.x | MIT OR Apache-2.0 | Version comparison (update check) |
| `minisign-verify` | 0.3 | MIT | Release-signature verification (update check) |

Full transitive tree is resolved in `Cargo.lock`. AGPL/GPL dependencies are
prohibited; all crates above are permissively licensed.

## Fonts and assets

* The TUI uses the terminal's built-in font.
* The GUI uses egui's bundled default fonts (`epaint_default_fonts`): open
  licence fonts (OFL-1.1 / Ubuntu Font Licence v1.0) carried inside the egui
  dependency tree, permitted for bundling/redistribution by their terms.
* No third-party icons, images, or audio are bundled. All ASCII glyphs are
  original.

## Runtime component

None — Dust Trail is pure Rust and bundles no external binaries.
