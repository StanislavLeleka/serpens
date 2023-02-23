use std::ops;

use super::row::Row;

pub struct Column {
    name: String,
    rows: Vec<Row>,
}

impl Column {
    pub fn new(name: String, rows: Vec<Row>) -> Self {
        Self { name, rows }
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn rows(&self) -> &[Row] {
        self.rows.as_ref()
    }

    pub fn set_rows(&mut self, rows: Vec<Row>) {
        self.rows = rows;
    }
}
