use std::ops::{self, Index};

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

    pub fn get_row(&self, index: usize) -> &Row {
        &self.rows[index]
    }

    pub fn copy(&mut self, column: Column) -> &Column {
        self.set_name(column.name);
        self.set_rows(column.rows);

        self
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

    pub fn add(&self, rhs: &Column) -> Column {
        let rows: Vec<Row> = self
            .rows
            .iter()
            .zip(rhs.rows.iter())
            .map(|(x, y)| {
                let v = x.value(false) + y.value(false);
                Row::new(x.index(), v)
            })
            .collect();

        Column {
            name: String::from("value"),
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

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_rows(&mut self, rows: Vec<Row>) {
        self.rows = rows;
    }
}

impl Index<usize> for Column {
    type Output = Row;

    fn index(&self, index: usize) -> &Self::Output {
        &self.rows[index]
    }
}
