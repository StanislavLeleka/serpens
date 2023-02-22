#[derive(Copy, Clone)]
pub struct Cell {
    row: usize,
    col: usize,
    value: f64,
}

impl Cell {
    pub fn new(row: usize, col: usize, value: f64) -> Self {
        Self { row, col, value }
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn col(&self) -> usize {
        self.col
    }
}
