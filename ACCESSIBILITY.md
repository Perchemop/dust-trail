# Accessibility

Dust Trail targets WCAG 2.2 Level AA where feasible (guide II.21).

## Contrast
Contrast is a unit test, not an opinion (II.30.7.1). `src/theme.rs` asserts
4.5:1 for body text, 3:1 for UI boundaries and focus rings, across light, dark,
and high-contrast variants. Runs headless in CI; a merge gate.

## Colour is never the only signal
Errors carry text and an icon/glyph; selection carries a fill and a marker;
disabled carries reduced emphasis plus a reason.

## Keyboard
Every flow is completable from the keyboard: WASD/arrows to move, E to
interact, S to save, L to load, R to restart, Q/Esc to quit. Focus follows
reading order; Esc dismisses the topmost dialog.

## Icon-only controls
The GUI toolbar buttons carry text labels (no icon-only controls in v0.1.0).

## Target size
Not applicable to a text grid; toolbar buttons are ≥ 24 pt.

## Reduced motion
egui has no cross-platform reduced-motion query. Animations are off by default
and under ~120 ms. Limitation recorded honestly (II.30.7.7).

## Limitation
The TUI renders to the terminal; screen-reader support depends on the terminal
emulator. The GUI uses egui/eframe; AccessKit integration is a v0.2.0 goal
(requires a seam refactor first, per II.27.12).
