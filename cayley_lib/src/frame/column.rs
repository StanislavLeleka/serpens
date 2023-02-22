use super::cell::Cell;

pub struct Column {
    title: String,
    cells: Vec<Cell>,
}

impl Column {
    pub fn new(title: String, cells: Vec<Cell>) -> Self {
        Self { title, cells }
    }

    pub fn cells_mut(&mut self) -> &mut Vec<Cell> {
        &mut self.cells
    }
}
