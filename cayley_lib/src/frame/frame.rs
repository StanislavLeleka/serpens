use std::collections::HashMap;

use linear::matrix::matrix::Matrix;

use super::{cell::Cell, column::Column, row::Row};

pub struct Frame {
    columns_map: HashMap<String, Column>,
    rows_map: HashMap<usize, Row>,
}

impl Frame {
    pub fn frame(data: Matrix, columns: Vec<String>) -> Frame {
        let mut rows_map: HashMap<usize, Row> = Self::gen_rows_map(data.shape().y());
        let mut columns_map: HashMap<String, Column> = Self::gen_cols_map(&columns);

        for i in 0..columns.len() {
            for y in 0..data.shape().y() {
                let cell: Cell = Cell::new(y, i, data.get(y, i));
                let row: Option<&mut Row> = rows_map.get_mut(&y);
                let col: Option<&mut Column> = columns_map.get_mut(&columns[i]);

                match row {
                    Some(r) => (*r).cells_mut().push(cell),
                    None => panic!("invalid row index"),
                }
                match col {
                    Some(c) => (*c).cells_mut().push(cell),
                    None => panic!("invalid column name"),
                }
            }
        }

        Frame {
            columns_map,
            rows_map,
        }
    }

    fn gen_rows_map(size: usize) -> HashMap<usize, Row> {
        (0..size).fold(HashMap::new(), |mut m, i| {
            m.insert(i, Row::new(i, vec![]));
            m
        })
    }

    fn gen_cols_map(titles: &Vec<String>) -> HashMap<String, Column> {
        (0..titles.len()).fold(HashMap::new(), |mut m, i| {
            m.insert(titles[i].clone(), Column::new(titles[i].clone(), vec![]));
            m
        })
    }

    pub fn columns_map(&self) -> &HashMap<String, Column> {
        &self.columns_map
    }

    pub fn rows_map(&self) -> &HashMap<usize, Row> {
        &self.rows_map
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

        println!("{:?}", frame);
    }
}
