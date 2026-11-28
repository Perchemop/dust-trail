# Research — ASCII isometric rendering, terminal UI, and "game feel"

**Purpose.** Original behavioural summaries + public source URLs for the
polish-within-ASCII direction. Per guide §2.3/§27.2, this is a summary of
*public behaviour and technique*, not copied text, code, or assets. It requires
the human gate before dependent implementation (guide §8.4).

**Scope note.** Dust Trail is a *genre* game (Western frontier adventure) in the
public ASCII/roguelike tradition. No commercial product's art, name, trade
dress, or source is used as implementation input.

| Area | Behaviour/technique observed | Public source URL |
|------|------------------------------|-------------------|
| Isometric projection | A 2:1 dimetric grid maps world (x,y) to screen as `sx=(x−y)·w/2`, `sy=(x+y)·h/2`; draw back-to-front (painter's algorithm) so nearer tiles overlap farther ones. | https://en.wikipedia.org/wiki/Isometric_video_game_graphics |
| Isometric projection (math) | 30° axonometric projection; the standard 2:1 pixel-art variant is a recognised convention, not a protected design. | https://en.wikipedia.org/wiki/Isometric_projection |
| ASCII rendering tradition | Text glyphs as a tile set; density/character choice conveys material and depth. Public-domain lineage (Rogue 1980, NetHack). | https://en.wikipedia.org/wiki/Roguelike |
| Terminal rendering | crossterm: raw mode, alternate screen, cursor control, RGB colour, non-blocking event poll. | https://docs.rs/crossterm |
| Terminal layout | ratatui: `TestBackend` allows headless rendering of a frame to a buffer for snapshot assertions. | https://docs.rs/ratatui/latest/ratatui/backend/struct.TestBackend.html |
| GUI headless evidence | egui_kittest: drives real egui widgets headlessly; AccessKit assertions + optional snapshot PNGs (CI artefacts). | https://docs.rs/egui_kittest |
| Game feel / juice | Input responsiveness, animation, screen shake, transitions and sound increase perceived quality beyond raw content ("Juice it or lose it"). | https://www.youtube.com/watch?v=Fy0aCDmgnxg |
| Screen shake | Short, decaying camera/offset shake as impact feedback; must respect reduced-motion. | https://www.youtube.com/watch?v=AJdEqssNZ-U |
| Colour & contrast | WCAG 2.2 contrast minima (4.5:1 body, 3:1 large/UI) as an accessibility floor. | https://www.w3.org/TR/WCAG22/#contrast-minimum |
| Reduced motion | Honour a user preference to disable non-essential motion. | https://www.w3.org/TR/WCAG22/#animation-from-interactions |

## Implications for Dust Trail (polish slice candidates)

1. **Isometric depth cues** — render tiles with material-specific glyphs and a
   subtle vertical offset for walls/props; keep the existing 2:1 iso mapping.
2. **Game feel** — add a short (<120 ms) move animation and a reduced-motion
   preference (guide II.30.7.7); optional screen-shake on the outlaw capture.
3. **Headless visual evidence** — add an `egui_kittest` harness for the GUI
   (seam refactor first, guide II.27.12), as CI artefacts, never committed
   baselines.
4. **Audio** — optional, feature-gated, off by default (no bundled licensed
   audio; generate simple tones or omit).

## Human gate

These notes are original summaries with URLs. Implementation that depends on
them waits for owner approval (guide §2.3, §8.4).
