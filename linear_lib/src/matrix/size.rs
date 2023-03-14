pub struct Size {
    rows: usize,
    cols: usize,
}

impl Size {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self { rows, cols }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn set_rows(&mut self, rows: usize) {
        self.rows = rows;
    }

    pub fn set_cols(&mut self, cols: usize) {
        self.cols = cols;
    }
}

impl Clone for Size {
    fn clone(&self) -> Self {
        Self {
            rows: self.rows,
            cols: self.cols,
        }
    }
}
