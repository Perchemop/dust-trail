//! RGB colour used by the domain. Frontends convert to their own colour type
//! (crossterm Color for TUI, egui::Color32 for GUI). Keeping colour in the
//! domain keeps the domain pure and lets both frontends render identical frames.

/// An sRGB colour.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub const BLACK: Self = Self { r: 0, g: 0, b: 0 };
    pub const WHITE: Self = Self {
        r: 255,
        g: 255,
        b: 255,
    };
    pub const GRAY: Self = Self {
        r: 160,
        g: 160,
        b: 160,
    };
    pub const DARK_GRAY: Self = Self {
        r: 80,
        g: 80,
        b: 80,
    };
    pub const RED: Self = Self {
        r: 220,
        g: 50,
        b: 50,
    };
    pub const GREEN: Self = Self {
        r: 80,
        g: 180,
        b: 80,
    };
    pub const BLUE: Self = Self {
        r: 70,
        g: 130,
        b: 200,
    };
    pub const YELLOW: Self = Self {
        r: 230,
        g: 210,
        b: 80,
    };
    pub const CYAN: Self = Self {
        r: 100,
        g: 210,
        b: 220,
    };
    pub const BROWN: Self = Self {
        r: 160,
        g: 100,
        b: 60,
    };
    pub const ORANGE: Self = Self {
        r: 230,
        g: 140,
        b: 50,
    };
    pub const MAGENTA: Self = Self {
        r: 210,
        g: 90,
        b: 200,
    };
}
