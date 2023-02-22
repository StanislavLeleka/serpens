use super::{cell::Cell, column::Column};

pub struct Row {
    index: usize,
    cells: Vec<Cell>,
}

impl Row {
    pub fn new(index: usize, cells: Vec<Cell>) -> Self {
        Self { index, cells }
    }

    pub fn cells_mut(&mut self) -> &mut Vec<Cell> {
        &mut self.cells
    }

    pub fn cells(&self) -> &[Cell] {
        self.cells.as_ref()
    }
}
