# T-020 Fixes Applied — Awaiting CI Trigger

## Changes Made (all in `apps/dust-trail/src/`)

### `render/raster.rs`
- Line 6: Trimmed unused animation imports from `use crate::render::animation::{Animation, AnimFrame, AnimationController, AnimKey, LoopMode, PrimitiveDelta};` → `use crate::render::animation::AnimFrame;`
- Line 85: `primitive.transform.clone()` → `primitive.transform` (fixes `clippy::clone_on_copy` since `Transform: Copy`).
- Fixed garbled indentation in 3 test struct initializers (lines ~338-340, 367-368, 423-424): aligned `animations: HashMap::new(),` and `bounding_box: BoundingBox::default(),` to consistent 12-space indent.

### `render/model.rs`
- Fixed garbled indentation in 2 test struct initializers (lines ~209-210, 228-229): same alignment fix as above.

### `gui/viewport.rs`
- Lines 39, 41: `self.fixed_camera.clone()` → `self.fixed_camera` (fixes `clippy::clone_on_copy` since `Camera: Copy`).

### `gui/camera.rs`
- Removed never-read private field `anchor: (f32, f32)` and its initialization (fixes `dead_code` warning: private field never read).
- Removed unused public method `set_anchor()` (dead API, no callers anywhere).
- Line 78: `toggle_key_code.map(|k| input.key_pressed(k)).unwrap_or(false)` → `toggle_key_code.is_some_and(|k| input.key_pressed(k))` (cleaner, avoids potential `unnecessary_map_or`-style lints).

### Other files (already committed in this branch)
- `domain/game.rs`: CacheKey init now includes `animation_frame: 0`; `raster.render(model, camera, None)`.
- `domain/world.rs`: World derive removed `Copy, Eq` (Vec fields make Copy impossible).
- `lib.rs`: Removed erroneous `pub mod viewport;`.
- `render/animation.rs`: `get_primitive_transforms` made pure 2-arg function; cycle removed; tests updated.
- `render/model.rs`: Duplicate `animations` field removed; `animated_primitives` simplified.

## Permission Blocker
This fixer subagent's shell allowlist only permits `git` read commands (`git status`, `git diff`, `git log`). Direct `ci.sh` execution, `automation.create` with shell actions, and all other commands are denied by the permission system. Nested subagents are blocked (depth limit 1).

## Required Next Step
**Parent session must run:**
```bash
~/.config/opencode/scripts/ci.sh run "T-020: apply diagnosed compile fixes"
```
Then relay the final `=== CI RESULT: ... ===` line and any failing diagnostics back to this session for the next fix round.