use std::{
    collections::HashMap,
    ops::{Index, IndexMut},
};

use linear::matrix::matrix_old::Matrixold;

use super::column::Column;

pub struct Frame {
    columns: Vec<Column>,
    headers: Vec<String>,
    columns_map: HashMap<String, usize>,
}

impl Frame {
    /// Creates a new [`Frame`].
    pub fn new(data: &Matrixold, headers: Vec<String>) -> Frame {
        let columns: Vec<Column> = Self::create_columns(data, &headers);
        let columns_map = Self::create_columns_map(&columns);

        Frame {
            columns,
            headers,
            columns_map,
        }
    }

    /// .
    pub fn get_column(&self, name: String) -> Option<&Column> {
        match self.columns_map.get(&name) {
            Some(i) => Some(&self.columns[*i]),
            None => None,
        }
    }

    /// .
    pub fn get_column_mut(&mut self, name: String) -> Option<&mut Column> {
        match self.columns_map.get(&name) {
            Some(i) => Some(&mut self.columns[*i]),
            None => None,
        }
    }

    /// .
    pub fn adjust_new_column(&mut self, name: &str, default_value: f64) -> Option<&Column> {
        let column: Column = Column::new(
            name.to_string(),
            vec![&default_value; self.get_column_size()],
        );

        self.insert_column(column);
        self.get_column(name.to_string())
    }

    /// .
    ///
    /// # Panics
    ///
    /// Panics if .
    pub fn add(&mut self, column_name: &str, value: f64) {
        match self.get_column_mut(column_name.to_string()) {
            Some(column) => column
                .rows_mut()
                .iter_mut()
                .for_each(|r| r.set_value(r.value(false) + value)),
            None => panic!("invalid column name"),
        }
    }

    /// .
    pub fn head(&self, size: usize) -> Frame {
        self.clone(|c, a| c.copy_with_range(0, a[0]), [size, 0])
    }

    /// .
    pub fn range(&self, left: usize, right: usize) -> Frame {
        self.clone(|c, a| c.copy_with_range(a[0], a[1]), [left, right])
    }

    /// Returns the column size of this [`Frame`].
    pub fn get_column_size(&self) -> usize {
        match self.columns().first() {
            Some(column) => column.rows().len(),
            None => 0,
        }
    }

    /// Returns a reference to the headers of this [`Frame`].
    pub fn headers(&self) -> &[String] {
        self.headers.as_ref()
    }

    /// Returns a reference to the columns of this [`Frame`].
    pub fn columns(&self) -> &Vec<Column> {
        self.columns.as_ref()
    }

    /// .
    fn create_columns(data: &Matrixold, headers: &Vec<String>) -> Vec<Column> {
        let mut columns: Vec<Column> = Vec::with_capacity(headers.len());
        for i in 0..headers.len() {
            let values: Vec<&f64> = data.get_column(i);
            let column: Column = Column::new(headers[i].clone(), values);
            columns.push(column);
        }
        columns
    }

    /// .
    fn create_columns_map(columns: &Vec<Column>) -> HashMap<String, usize> {
        let mut columns_map: HashMap<String, usize> = HashMap::new();
        for c in 0..columns.len() {
            columns_map.insert(columns[c].name().to_string(), c);
        }
        columns_map
    }

    /// .
    pub fn insert_column(&mut self, column: Column) {
        let name: String = column.name().to_string();

        self.columns.push(column);
        self.headers.push(name.clone());
        self.columns_map.insert(name, self.columns.len() - 1);
    }

    /// Returns the copy of headers of this [`Frame`].
    fn clone_headers(&self) -> Vec<String> {
        self.headers.iter().map(|h| h.clone()).collect()
    }

    /// Returns the copy of columns map of this [`Frame`].
    fn clone_columns_map(&self) -> HashMap<String, usize> {
        let mut columns_map: HashMap<String, usize> = HashMap::new();
        self.columns_map.iter().for_each(|(k, v)| {
            columns_map.insert(k.clone(), v.clone());
        });
        columns_map
    }

    fn clone(&self, f: fn(&Column, [usize; 2]) -> Column, params: [usize; 2]) -> Frame {
        let coumns_copy: Vec<Column> = self.columns.iter().map(|c| f(c, params)).collect();
        let headers_copy: Vec<String> = self.clone_headers();
        let columns_map_copy: HashMap<String, usize> = self.clone_columns_map();

        Frame {
            columns: coumns_copy,
            headers: headers_copy,
            columns_map: columns_map_copy,
        }
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
    use linear::matrix::matrix_old::Matrixold;

    use crate::frame::column::Column;

    use super::Frame;

    #[test]
    fn test_frame_init() {
        let my_data = Matrixold::matrix2d(vec![
            vec![0.0, 3.011212121],
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
        let my_data = Matrixold::matrix2d(vec![
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
        let my_data = Matrixold::matrix2d(vec![
            vec![0.0, 3.0],
            vec![10.0, 7.0],
            vec![20.0, 9.0],
            vec![30.0, 14.0],
            vec![40.0, 15.0],
        ]);
        let my_column_names = vec![String::from("temperature"), String::from("activity")];

        let mut frame: Frame = Frame::new(&my_data, my_column_names);
        let mut new_col: Option<&Column> = frame.adjust_new_column("adjusted", 2.0);

        println!("{:?}", new_col.unwrap());

        new_col = frame.adjust_new_column("adjusted 2", 5.1);

        println!("{:?}", new_col.unwrap());

        let head = frame.head(3);

        println!("{:?}", head);

        let range = frame.range(2, 4);

        println!("{:?}", range);

        println!("{:?}", frame["temperature"]);

        let mut a12 = frame["temperature"].add(&frame["activity"]);
        a12.set_name(String::from("value11"));

        frame.insert_column(a12);

        println!("{:?}", frame);
    }
}
