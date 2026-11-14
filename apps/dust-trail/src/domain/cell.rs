//! A single rendered cell: a glyph plus foreground/background colours.

use crate::domain::color::Rgb;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Cell {
    pub ch: char,
    pub fg: Rgb,
    pub bg: Rgb,
}

impl Cell {
    pub fn new(ch: char, fg: Rgb, bg: Rgb) -> Self {
        Self { ch, fg, bg }
    }

    pub fn empty() -> Self {
        Self::new(' ', Rgb::BLACK, Rgb::BLACK)
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self::empty()
    }
}
