//! Sprite type: cached multi-cell ASCII sprite with depth and anchor.

use crate::domain::world::Rgb;

/// Cached multi-cell ASCII sprite with per-cell depth and footprint anchor.
#[derive(Clone, Debug, PartialEq)]
pub struct Sprite {
    pub key: crate::render::model::ModelKey,
    pub width: usize,
    pub height: usize,
    /// Grid of (char, fg, bg) per cell.
    pub cells: Vec<Vec<(char, Rgb, Rgb)>>,
    /// Per-cell depth (z) matching each cell; used by composite for occlusion.
    pub depth: Vec<Vec<f32>>,
    /// Footprint anchor: screen offset of the sprite's origin (bottom-center by convention).
    pub anchor: (i32, i32),
}

impl Sprite {
    /// Create an empty 0×0 sprite (for degenerate/empty models).
    pub fn empty(key: crate::render::model::ModelKey) -> Self {
        Self {
            key,
            width: 0,
            height: 0,
            cells: Vec::new(),
            depth: Vec::new(),
            anchor: (0, 0),
        }
    }

    /// Create a placeholder '?' sprite for unknown models.
    pub fn placeholder(key: crate::render::model::ModelKey) -> Self {
        Self {
            key,
            width: 1,
            height: 1,
            cells: vec![vec![('?', Rgb(255, 255, 255), Rgb(0, 0, 0))]],
            depth: vec![vec![0.0]],
            anchor: (0, 0),
        }
    }

    /// Check if sprite is empty (0×0).
    pub fn is_empty(&self) -> bool {
        self.width == 0 || self.height == 0
    }
}

impl Default for Sprite {
    fn default() -> Self {
        Self::empty("default")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sprite_cells_depth_identical_dimensions() {
        let sprite = Sprite::placeholder("test");
        assert_eq!(sprite.cells.len(), sprite.depth.len());
        assert_eq!(sprite.cells[0].len(), sprite.depth[0].len());
    }

    #[test]
    fn anchor_defaults_to_zero_zero() {
        let sprite = Sprite::placeholder("test");
        assert_eq!(sprite.anchor, (0, 0));
    }

    #[test]
    fn empty_sprite_is_empty() {
        let sprite = Sprite::empty("test");
        assert!(sprite.is_empty());
        assert_eq!(sprite.width, 0);
        assert_eq!(sprite.height, 0);
    }

    #[test]
    fn placeholder_sprite_non_empty() {
        let sprite = Sprite::placeholder("test");
        assert!(!sprite.is_empty());
        assert_eq!(sprite.width, 1);
        assert_eq!(sprite.height, 1);
        assert_eq!(sprite.cells[0][0].0, '?');
    }
}