use std::{
    collections::HashMap,
    ops::{self, Index},
};

use linear::matrix::matrix::Matrix;

use super::{column::Column, row::Row};

pub struct Frame {
    columns: Vec<Column>,
    headers: Vec<String>,
    columns_map: HashMap<String, usize>,
}

impl Frame {
    pub fn new(data: &Matrix, headers: Vec<String>) -> Frame {
        let mut columns: Vec<Column> = Vec::with_capacity(headers.len());
        for i in 0..headers.len() {
            let values: Vec<&f64> = data.get_column(i);
            let mut rows: Vec<Row> = Vec::with_capacity(values.len());

            for j in 0..values.len() {
                rows.push(Row::new(j, *values[j]));
            }

            let column: Column = Column::new(headers[i].clone(), rows);

            columns.push(column);
        }

        let mut columns_map: HashMap<String, usize> = HashMap::new();
        for c in 0..columns.len() {
            columns_map.insert(columns[c].name().to_string(), c);
        }

        Frame {
            columns,
            headers,
            columns_map,
        }
    }

    pub fn get_column(&self, name: String) -> Option<&Column> {
        match self.columns_map.get(&name) {
            Some(i) => Some(&self.columns[*i]),
            None => None,
        }
    }

    pub fn adjusted_new_column(&mut self, name: &str) -> &Column {
        let rows_len: usize = self.columns[0].rows().len();
        let mut rows: Vec<Row> = Vec::with_capacity(rows_len);
        for i in 0..rows_len {
            rows.push(Row::new(i, 0.0));
        }

        let column: Column = Column::new(name.to_string(), rows);

        self.columns.push(column);
        self.columns_map
            .insert(name.to_string(), self.columns.len() - 1);
        &self.columns[self.columns.len() - 1]
    }

    pub fn headers(&self) -> &[String] {
        self.headers.as_ref()
    }

    pub fn columns(&self) -> &Vec<Column> {
        &self.columns
    }
}

impl Index<&str> for Frame {
    type Output = Column;

    fn index(&self, index: &str) -> &Self::Output {
        match self.get_column(index.to_string()) {
            Some(col) => col,
            None => panic!("invlaid column name"),
        }
    }
}

#[cfg(test)]
mod test {
    use linear::matrix::matrix::Matrix;

    use crate::frame::column::Column;

    use super::Frame;

    #[test]
    fn test_frame_init() {
        let my_data = Matrix::matrix2d(vec![
            vec![0.0, 3.0],
            vec![10.0, 7.0],
            vec![20.0, 9.0],
            vec![30.0, 14.0],
            vec![40.0, 15.0],
        ]);
        let my_column_names = vec![String::from("temperature"), String::from("activity")];

        let frame: Frame = Frame::new(&my_data, my_column_names);

        println!("{:?}", frame);
    }

    #[test]
    fn get_frame_column() {
        let my_data = Matrix::matrix2d(vec![
            vec![0.0, 3.0],
            vec![10.0, 7.0],
            vec![20.0, 9.0],
            vec![30.0, 14.0],
            vec![40.0, 15.0],
        ]);
        let my_column_names = vec![String::from("temperature"), String::from("activity")];

        let frame: Frame = Frame::new(&my_data, my_column_names);
        let col = &frame["temperature"];

        println!("{:?}", col);
        println!("{:?}", &frame["activity"]);
    }

    #[test]
    fn test_adjusted_new_column() {
        let my_data = Matrix::matrix2d(vec![
            vec![0.0, 3.0],
            vec![10.0, 7.0],
            vec![20.0, 9.0],
            vec![30.0, 14.0],
            vec![40.0, 15.0],
        ]);
        let my_column_names = vec![String::from("temperature"), String::from("activity")];

        let mut frame: Frame = Frame::new(&my_data, my_column_names);
        let new_col: &Column = frame.adjusted_new_column("adjusted");

        println!("{:?}", new_col);
    }
}
