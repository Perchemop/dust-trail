# Product

<!-- impeccable:product-schema 1 -->

## Platform

adaptive

## Stack

Rust: egui/eframe 0.37, eframe-wgpu; core domain crate, GUI crate feature-gated

## Users

Primary: Players who enjoy turn-based, exploration-focused ASCII games with a minimalist Western aesthetic. They play as a desktop window (GUI). They value hackability, readability, and zero network requirement.

## Product Purpose

Dust Trail is a free, original Western-themed adventure game rendered entirely in ASCII. The player takes the role of "Dust," a drifter in a frontier town, moving through a top-down ASCII world, talking to the sheriff, pursuing the outlaw, collecting loot, and saving progress. Success means a complete, playable adventure with real persistence, clean-room originality (no commercial trademarks or proprietary assets), and a polished UI that feels intentional—not generated.

## Positioning

A hackable, zero-dependency ASCII Western adventure that runs everywhere Rust runs. The mechanism is a command-centric domain core (headless, fully tested) with a thin GUI shell—distinct from games that bury logic in the render loop. The aesthetic is original ASCII, not a clone of any commercial roguelike's look.

## Operating Context

- Play sessions: 10–30 minutes, save/load anytime
- Input: keyboard-first (WASD/arrows/hjkl, E interact, F attack, M mount, U use item)
- Persistence: JSON save slots in platform config dir
- No network required for core gameplay; update-check and crash-report are optional, user-consented, soft-fail offline
- Frontend: GUI (egui/eframe), same domain core
- Platforms: Linux (primary), macOS, Windows; AppImage/.msi/.dmg via CI

## Capabilities and Constraints

- Domain: turn-based movement, NPC interaction (sheriff/outlaw/townsfolk), inventory (loot/horse), combat, wanted level, score, mount toggle
- Save/load: named slots, list existing, overwrite confirmation
- GUI: toolbar (New/Save/Load/Quit), right-aligned status (Wanted, HP, Inv, Turn, Score, Mount), inventory window, save/load dialogs, release notes modal, game-over overlay
- Clean-room: all code, strings, ASCII art, mechanics original; no RE, no proprietary assets
- Legal: MIT OR Apache-2.0; ATTRIBUTION.md for third-party crates
- Accessibility: WCAG 2.2 AA where feasible (keyboard complete, focus visible, contrast AA, reduced-motion preference)
- Theme: light/dark/high-contrast variants driven by semantic token roles
- Version display: subtle bottom-right button opens release-notes popup with token metrics

## Brand Commitments

- Name: "Dust Trail" (original, reserved)
- Voice: terse, frontier, respectful—never "vibecoded" (no emojis, no fake testimonials, no harsh gradients, no purple/black default)
- Visual identity: original ASCII glyphs; SVG icons for platform assets only; no Lucide, no stock icon sets
- Window title: "Dust Trail"
- No network requirement to play

## Evidence on Hand

- GUI frontend running in egui/eframe 0.37
- Domain core with save/load, turn loop, NPC AI, combat, inventory
- CI: fmt, clippy -D warnings, tests, release builds for Linux/macOS/Windows
- Release workflow: semver tags, GitHub Releases with SHA256SUMS.txt + minisign
- CHANGELOG.md, ROADMAP.md, PROGRESS.md, DECISIONS.md
- docs/design/brand.md, docs/design/gui-stack.md (egui 0.37 resolved)
- docs/capability-inventory.md, docs/parity.md
- docs/tokens.md (token metrics per release)
- contributors/people.toml (consent-verified credits)

## Product Principles

1. **Core first, shell thin** — domain logic is headless, tested, and independent of the renderer; GUI is the view.
2. **Originality over homage** — every glyph, string, and mechanic is ours; inspiration is behavioural only, never visual cloning.
3. **Keyboard completes every flow** — no mouse-only paths; focus is visible and sensible.
4. **Polish is evidence, not prose** — contrast tests, rendered captures, AccessKit assertions gate UI merges.
5. **Free, open, runnable** — no compiler needed to try; CI artefacts are the product.

## Accessibility & Inclusion

- WCAG 2.2 AA target for native desktop (egui + AccessKit on Windows/macOS)
- Keyboard-only: every flow completable; Esc dismisses dialogs; Enter activates default; focus ring visible
- Contrast: 4.5:1 body text, 3:1 large text/UI boundaries/focus rings, both themes + high-contrast variant
- Reduced motion: in-app preference (platform signal unavailable in egui); default off when unknown; transitions ≤120 ms
- Target size: ≥24 logical points interactive targets; primary/destructive ≥32
- Screen-reader: AccessKit roles/names/values on interactive widgets; icon-only controls carry labelled_by text
- Reflow: narrow window and 200% zoom no clipping; side panel collapses before canvas
- Limitations recorded in ACCESSIBILITY.md