//! TUI renderer: renders a domain `Frame` to the terminal using crossterm.
//! Colours are grouped so consecutive same-colour cells share one escape
//! sequence.

use crate::domain::color::Rgb;
use crate::domain::frame::Frame;
use crossterm::style::{SetBackground, SetForeground};
use crossterm::{
    cursor::{MoveTo, Print},
    ExecutableCommand,
};
use std::io::{stdout, Stdout, Write};

/// Convert a domain RGB to a crossterm colour.
pub fn crossterm_color(col: Rgb) -> crossterm::style::Color {
    crossterm::style::Color::Rgb {
        r: col.r,
        g: col.g,
        b: col.b,
    }
}

/// Render a full frame to stdout. Returns the number of cells drawn.
pub fn render_frame(stdout: &mut Stdout, frame: &Frame) -> std::io::Result<usize> {
    stdout.execute(MoveTo(0, 0))?;
    let mut count: usize = 0;
    let mut current_fg: Option<Rgb> = None;
    let mut current_bg: Option<Rgb> = None;

    for (_y, row) in frame.rows() {
        for cell in row {
            if Some(cell.fg) != current_fg {
                current_fg = Some(cell.fg);
                stdout.execute(SetForeground(crossterm_color(cell.fg)))?;
            }
            if Some(cell.bg) != current_bg {
                current_bg = Some(cell.bg);
                stdout.execute(SetBackground(crossterm_color(cell.bg)))?;
            }
            stdout.execute(Print(cell.ch.to_string()))?;
            count += 1;
        }
        // Move to the start of the next line; keep colours persistent across
        // rows so a row's trailing colour carries over only when identical.
        stdout.execute(Print("\r\n"))?;
    }
    stdout.flush()?;
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crossterm_color_roundtrip() {
        let col = Rgb::new(1, 2, 3);
        match crossterm_color(col) {
            crossterm::style::Color::Rgb { r, g, b } => {
                assert_eq!((r, g, b), (1, 2, 3));
            }
            other => panic!("expected Rgb, got {other:?}"),
        }
    }
}
