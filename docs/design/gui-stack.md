# GUI Stack (guide II.30.2)

Detected before any UI recommendation; recorded here per II.30.2.

| Field | Value |
|-------|-------|
| GUI toolkit (desktop) | egui 0.37 + eframe 0.37 |
| Resolved versions | From `Cargo.lock` (to be confirmed on first CI build) |
| GUI shell shape | Renders into `egui::Context` / `&mut Ui` via `eframe::App::update` |
| Theme module | `apps/dust-trail/src/theme/mod.rs` (semantic roles, light/dark/high-contrast) |
| Raw-colour guard | CI grep gate: no `Color32::from_*` outside `src/theme/` |
| Contrast tests | `src/theme/mod.rs` unit tests (4.5:1 text, 3:1 UI/boundary, all variants) |
| a11y / snapshot harness | None yet (kittest seam refactor planned for v0.2.0) |
| Layout model | Single panel: monospace ASCII grid + top toolbar + status line |

**Non-goals (II.30.14):** No Astryx/React/CSS design system in the native GUI;
no multiple design systems; no toolkit version bump inside a UX slice.