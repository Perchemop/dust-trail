//! A rendered frame: a grid of cells plus an optional message line. Both the
//! TUI and GUI frontends render the same Frame produced by the domain.
//! v2.0.0: adds a depth buffer for occlusion-aware compositing.

use crate::domain::cell::Cell;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Frame {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<Cell>>,
    pub message: String,
    /// Depth buffer parallel to `cells`: stores the nearest depth written at each position.
    /// Reset to INFINITY at the start of each frame build.
    #[serde(skip)] // depth is runtime state, not persisted
    depth: Vec<Vec<f32>>,
}

impl Frame {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![vec![Cell::empty(); width]; height],
            message: String::new(),
            depth: vec![vec![f32::INFINITY; width]; height],
        }
    }

    pub fn cell(&self, x: usize, y: usize) -> Option<Cell> {
        self.cells.get(y).and_then(|row| row.get(x)).copied()
    }

    /// Flat write (unchanged semantics, kept for compatibility).
    pub fn set(&mut self, x: usize, y: usize, cell: Cell) {
        if y < self.height && x < self.width {
            self.cells[y][x] = cell;
        }
    }

    /// Depth-aware write: overwrites only if `depth` is nearer than the stored depth
    /// at (x, y); updates the depth buffer. No-op out of bounds.
    pub fn set_at_depth(&mut self, x: usize, y: usize, cell: Cell, depth: f32) {
        if y < self.height && x < self.width && depth < self.depth[y][x] {
            self.cells[y][x] = cell;
            self.depth[y][x] = depth;
        }
    }

    /// Depth buffer accessor (for tests).
    pub fn depth_at(&self, x: usize, y: usize) -> Option<f32> {
        if y < self.height && x < self.width {
            Some(self.depth[y][x])
        } else {
            None
        }
    }

    /// Reset the depth buffer to infinity (called at the start of each frame build).
    pub fn clear_depth(&mut self) {
        for row in &mut self.depth {
            row.fill(f32::INFINITY);
        }
    }

    /// Iterate over rows as (y, &[Cell]).
    pub fn rows(&self) -> impl Iterator<Item = (usize, &[Cell])> {
        self.cells
            .iter()
            .enumerate()
            .map(|(y, row)| (y, row.as_slice()))
    }
}
