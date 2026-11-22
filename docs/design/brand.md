# Brand — Dust Trail

Owned by the Designer role (guide II.27.C). Original identity only — no
commercial look-alike branding, no copied trade dress (§2.2, §10).

## Name and message
- **Name:** Dust Trail — original, checked against existing projects, package
  registries and the WIPO Global Brand Database (II.19.2); decision recorded in
  `DECISIONS.md`.
- **One-paragraph message (II.22.1):** Dust Trail is a free, original
  Western-themed adventure game rendered entirely in ASCII. Playable as a
  desktop window (GUI) or terminal interface (TUI), fully offline, with real
  save/load, bounties, wanted levels and outlaw AI. No commercial trademarks
  or assets.

## Visual system
- **UI framework:** egui/eframe 0.37; all colours, spacing, radii and type come
  from the token module (`apps/dust-trail/src/theme/`) — semantic roles, not
  hex literals (II.30.6). CI enforces the raw-colour gate.
- **Rendering:** 3D ASCII isometric pipeline — shaded primitives rasterised to
  a 10-character glyph ramp (`render/glyph.rs`) under a fixed isometric camera.
- **Iconography:** original glyphs; no emoji as UI icons (§20 item 7).
- **Logo / icon set / installer art:** not yet designed — Designer backlog
  (II.27.C); describe marks here before any asset is drawn (§10).

## Voice and tone
- Plain language, inclusive wording ("they" for a single person), honest
  release notes (II.22.3). Product copy avoids the §20 anti-pattern list;
  em dashes are fine in process documents, avoided in product copy.
