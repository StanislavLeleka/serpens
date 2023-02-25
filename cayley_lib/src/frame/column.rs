use std::ops;

use super::row::Row;

pub struct Column {
    name: String,
    rows: Vec<Row>,
}

impl Column {
    /// Creates a new [`Column`].
    pub fn new(name: String, values: Vec<&f64>) -> Self {
        Self {
            name,
            rows: Self::create_rows(values),
        }
    }

    pub fn copy(&self, take_rows: usize) -> Column {
        let size: usize = take_rows % self.rows.len();
        let mut rows: Vec<Row> = Vec::with_capacity(size);
        for i in 0..size {
            rows.push(self.rows[i].copy());
        }

        Column {
            name: self.name.clone(),
            rows,
        }
    }

    pub fn copy_with_range(&self, left: usize, right: usize) -> Column {
        let mut rows: Vec<Row> = Vec::with_capacity(right - left);
        for i in left..right {
            rows.push(self.rows[i].copy());
        }

        Column {
            name: self.name.clone(),
            rows,
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn rows(&self) -> &[Row] {
        self.rows.as_ref()
    }

    pub fn rows_mut(&mut self) -> &mut Vec<Row> {
        &mut self.rows
    }

    fn create_rows(values: Vec<&f64>) -> Vec<Row> {
        let mut rows: Vec<Row> = Vec::with_capacity(values.len());
        for i in 0..values.len() {
            rows.push(Row::new(i, *values[i]));
        }
        rows
    }
}
