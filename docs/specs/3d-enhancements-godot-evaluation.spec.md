# Feature: 3D Enhancements (Godot Capability Evaluation)

## Overview

Dust Trail is a 2D ASCII adventure game built with Pure Rust + egui/eframe + wgpu. The user wants to add 3D capabilities — terrain/voxel worlds, advanced 3D physics, VFX/particle systems, and navigation/pathfinding — while staying a 2D ASCII game (not pivoting to full 3D). This specification evaluates Godot ecosystem technologies (JoltPhysics, Terrain3D, Effekseer, VoxelTools, RecastNavigation) against Rust-native alternatives, and defines requirements for integrating 3D enhancements into the existing Rust/egui stack.

**Constraint:** The app must remain Rust + egui/eframe. Godot is evaluated only as a reference for capability patterns, not as a runtime dependency. All platforms (Windows x86_64, macOS arm64, Linux x86_64/arm64, Web/WASM) must be supported.

## Functional Requirements

### FR-001: 3D Physics Engine
While the game runs a turn-based simulation, when an entity moves or collides,
the system shall resolve 3D rigid-body physics using a Rust-native physics crate
(e.g. Rapier) with deterministic, fixed-tick step semantics.

### FR-002: Voxel / Chunked Terrain
While the world view is active, when terrain is generated,
the system shall render chunked voxel-based terrain using a Rust voxel library
(e.g. voxelize or custom mesh generation), supporting overhangs and multiple materials.

### FR-003: Particle / VFX System
While an action occurs (attack, spell, movement), when a visual effect is triggered,
the system shall render a particle effect using a Rust GPU particle system or
egui-based particle renderer, with support for burst, trail, and ambient emitters.

### FR-004: Navigation Mesh Pathfinding
While an entity needs to move to a target, when pathfinding is requested,
the system shall compute a navmesh path using a Rust navigation crate
(e.g. nightshade-navmesh or constructive), avoiding obstacles and terrain.

### FR-005: Platform Coverage
When the app is built for release, it shall produce installable artefacts for
Windows x86_64 (.msi/.exe), macOS arm64 (.dmg), Linux x86_64/arm64 (.deb/.rpm/.AppImage),
and Web/WASM, with SHA256SUMS.txt and signature verification.

### FR-006: Deterministic Fixed-Tick Timing
While the game simulates turns, when animation or physics frames are rendered,
the system shall use fixed-point integer-tick timing (per v2.1.0 spec AC-215b)
for cross-platform determinism, with animation frames keyed by u64 frame content hash.

### FR-007: Cache Key Deduplication
While assets are cached, when a cache entry is stored,
the system shall use CacheKey with animation_frame: u64 frame content hash,
deduplicate identical frames, and cap max_entries at 5000 (per v2.1.0 spec T-203).

## Non-Functional Requirements

### Performance
- Physics step: < 16ms per turn on target hardware
- Terrain chunk generation: async, non-blocking on the UI thread
- Particle system: 1000+ concurrent particles at 60 FPS
- Pathfinding: < 50ms for typical navmesh queries

### Security
- No embedded privileged tokens in binaries (per guide §II.19.4)
- Crash reports scrub secrets and paths before submission (per guide §II.29.2)
- Update check verifies SHA-256 + minisign signature before install (per guide §7.2)

### Accessibility
- WCAG AA contrast (4.5:1 body text, 3:1 UI boundaries) on all 3D-enhanced UI (per guide II.30.7)
- Keyboard completes every flow; Esc dismisses dialogs
- Icon-only controls carry AccessKit labels

### Maintainability
- One design path only (no multi-system mashup — guide II.30.14)
- Toolkit/dependency version bumps are their own slice, never hidden in a UX slice
- All CI runs via ci.sh; no local cargo builds (guide §4 item 5)

## Acceptance Criteria

### AC-001: Physics Integration
Given a Rust physics crate (Rapier) is integrated into the domain layer,
When an entity moves and collides with terrain,
Then collisions are resolved deterministically within one fixed tick,
And the result is identical across all supported platforms for the same input.

### AC-002: Terrain Rendering
Given a voxel/chunked terrain system is implemented,
When the player enters a new world area,
Then terrain chunks generate asynchronously without blocking the UI,
And overhangs and multiple materials render correctly.

### AC-003: VFX Particles
Given a particle system is implemented,
When an attack or spell effect triggers,
Then particles render with correct burst/trail behavior,
And the system handles 1000+ particles without UI stutter.

### AC-004: Pathfinding
Given a navmesh pathfinding crate is integrated,
When an entity requests a path to a target,
Then a valid path around obstacles is returned within 50ms,
And unreachable targets return a clear "no path" result.

### AC-005: Multi-Platform Release
Given a release is tagged,
Then installable artefacts exist for all four platforms,
And SHA256SUMS.txt + signature are attached and verify correctly,
And CI is green on all platforms before the tag is created.

### AC-006: Determinism
Given the same sequence of game inputs on two different platforms,
When the simulation runs for N turns,
Then the resulting game state is bit-identical on both platforms.

### AC-007: Clean-Room Compliance
Given all 3D enhancement code is written from scratch,
When the code is reviewed,
Then no proprietary Godot source, assets, or layouts are copied,
And all third-party crates have permissive licences recorded in ATTRIBUTION.md.

## Error Handling

| Error Condition | Response | User Message |
|-----------------|----------|--------------|
| Physics engine init fails | Fall back to simple AABB collision | "Physics unavailable — using simplified collision" |
| Terrain chunk generation fails | Show placeholder terrain | "Terrain still loading..." |
| Particle system exceeds limit | Drop oldest particles, continue | (silent, no user impact) |
| Pathfinding returns no path | Entity stays in place, show indicator | "No path to target" |
| Build artefact missing platform | Block release, show CI error | (CI gate, not user-facing) |
| Signature verification fails | Abort update, keep current version | "Update verification failed" |

## Implementation TODO

### Research & Evaluation
- [ ] Document Rapier 3D physics API and egui integration pattern
- [ ] Evaluate voxelize / voxel_worldgen for chunked terrain in egui
- [ ] Evaluate par_particle_life or custom GPU particle system
- [ ] Evaluate nightshade-navmesh / constructive for Rust navmesh
- [ ] Confirm all selected crates support WASM target
- [ ] Record licence + version in ATTRIBUTION.md and docs/resource-catalog.md

### Domain Layer (3D Core)
- [ ] Add physics world to core crate (headless, no display)
- [ ] Add fixed-tick step controller with u64 frame counter
- [ ] Add terrain chunk manager (async generation)
- [ ] Add navmesh pathfinder service
- [ ] Add particle effect service (data-driven, render-agnostic)

### UI Layer (egui/eframe)
- [ ] Create 3D viewport panel (egui + wgpu or custom ASCII 3D rasteriser)
- [ ] Integrate physics debug visualisation (gated behind Ctrl+Shift+C)
- [ ] Add particle rendering to viewport
- [ ] Add terrain chunk rendering to viewport
- [ ] Add keyboard shortcuts for all 3D actions

### CI & Release
- [ ] Add WASM build target to CI matrix
- [ ] Add all four platform release artefacts to release.yml
- [ ] Add SHA256SUMS.txt + minisign signing to release workflow
- [ ] Add secret scan + personal-data scan as required checks
- [ ] Update .ci-required.txt with new gates

### Testing
- [ ] Headless physics test (determinism across platforms)
- [ ] Terrain chunk generation test (async, non-blocking)
- [ ] Pathfinding test (obstacle avoidance, no-path case)
- [ ] Particle system test (count, burst, cleanup)
- [ ] Regression test for save migration (v2.0.x → time_of_day = 120)

## Out of Scope
- Full pivot to Godot engine (app stays Rust + egui)
- Using Godot as a runtime dependency
- 3D models with skeletal animation beyond primitive rasteriser scope (v2.1.0 scope)
- Real-time multiplayer networking
- Mobile platforms (iOS/Android)

## Open Questions
- [ ] Which Rust physics crate: Rapier vs nphysics vs jolt-rs bindings?
- [ ] Which Rust voxel crate: voxelize vs custom mesh generation?
- [ ] Does the particle system need GPU compute or is CPU sufficient?
- [ ] Should navmesh be pre-baked or generated at runtime?
- [ ] Human approval needed before implementing large 3D features (guide §2.3)
