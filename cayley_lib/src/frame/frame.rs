use std::{
    collections::{HashMap, HashSet},
    ops,
};

use linear::matrix::matrix::Matrix;

use super::row::Row;

pub struct Frame {
    rows: Vec<Row>,
    rows_map: HashMap<String, Vec<Row>>,
}

impl Frame {
    pub fn frame(data: Matrix, columns: Vec<String>) -> Frame {
        let mut rows_map: HashMap<String, Vec<Row>> = HashMap::new();

        for i in 0..columns.len() {
            let mut rows: Vec<Row> = vec![];
            for y in 0..data.shape().y() {
                rows.push(Row::new(columns[i].clone(), data.get(i, y), 0));
            }
            rows_map.insert(columns[i].clone(), rows);
        }

        Frame {
            rows: vec![],
            rows_map,
        }
    }

    pub fn rows(&self) -> &[Row] {
        self.rows.as_ref()
    }

    pub fn rows_map(&self) -> &HashMap<String, Vec<Row>> {
        &self.rows_map
    }
}

impl ops::Index<String> for Frame {
    type Output = Row;

    fn index(&self, index: String) -> &Self::Output {
        if !self.rows_map.contains_key(&index) {
            panic!("key not found")
        }

        todo!()
    }
}

#[cfg(test)]
mod test {
    use linear::matrix::matrix::Matrix;

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

        let frame: Frame = Frame::frame(my_data, my_column_names);
    }
}
