//! A rendered frame: a grid of cells plus an optional message line. Both the
//! TUI and GUI frontends render the same Frame produced by the domain.

use crate::domain::cell::Cell;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Frame {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<Cell>>,
    pub message: String,
}

impl Frame {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![vec![Cell::empty(); width]; height],
            message: String::new(),
        }
    }

    pub fn cell(&self, x: usize, y: usize) -> Option<Cell> {
        self.cells.get(y).and_then(|row| row.get(x)).copied()
    }

    pub fn set(&mut self, x: usize, y: usize, cell: Cell) {
        if y < self.height && x < self.width {
            self.cells[y][x] = cell;
        }
    }

    /// Iterate over rows as (y, &[Cell]).
    pub fn rows(&self) -> impl Iterator<Item = (usize, &[Cell])> {
        self.cells.iter().enumerate().map(|(y, row)| (y, row.as_slice()))
    }
}
