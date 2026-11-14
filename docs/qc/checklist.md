# QC Checklist (guide II.21.4, II.30.8)

Run whenever the UI is touched. Record results in the QC run log.

- [ ] Keyboard-only walk-through: every flow completable without a mouse.
- [ ] Esc dismisses the topmost dialog.
- [ ] Screen-reader spot-check (Narrator/NVDA/VoiceOver).
- [ ] Reflow at 200% zoom and narrow window — no clipped or unreachable controls.
- [ ] High-contrast check in both themes.
- [ ] Contrast unit tests green (theme.rs: 4.5:1 body, 3:1 UI/boundary).
- [ ] Icon-only controls carry an AccessKit label / tooltip.
- [ ] Interactive targets ≥ 24 pt; primary/destructive ≥ 32 pt.
- [ ] Rendered evidence captured (shell light/dark, empty, error, dialog,
      narrow window) — artefacts reviewed, never committed.

# Shortcuts (docs/qc/shortcuts.md)

| Shortcut | Action |
|----------|--------|
| W / Up | Move north |
| S / Down | Move south |
| A / Left | Move west |
| D / Right | Move east |
| E | Interact with adjacent NPC/item |
| R | New game |
| S (shift) | Save game |
| L | Load game |
| Q / Esc | Quit |
| Ctrl-C | Quit |
