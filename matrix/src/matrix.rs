pub struct Matrix {
    dim: usize,
    size: usize,
    data: Vec<f64>,
}

impl Matrix {
    pub fn matrix(elements: Vec<f64>) -> Matrix {
        return Matrix {
            dim: 1,
            size: elements.len(),
            data: elements,
        };
    }

    pub fn matrix2d(elements: Vec<Vec<f64>>) -> Matrix {
        let size: usize = elements[0].len();
        let mut data: Vec<f64> = vec![];
        elements
            .into_iter()
            .for_each(|mut row: Vec<f64>| data.append(&mut row));

        return Matrix { dim: 2, size, data };
    }

    pub fn matrix3d(elements: Vec<Vec<Vec<f64>>>) -> Matrix {
        let size: usize = elements[0][0].len();
        let mut data: Vec<f64> = vec![];
        elements.into_iter().for_each(|row: Vec<Vec<f64>>| {
            row.into_iter()
                .for_each(|mut inner: Vec<f64>| data.append(&mut inner));
        });
        return Matrix { dim: 3, size, data };
    }

    pub fn print(&self) {
        match self.dim {
            1 => println!("{:?}", self.data()),
            2 => self.print2d(),
            _ => println!("Invalid matrix type!"),
        }
    }

    pub fn data(&self) -> &[f64] {
        self.data.as_ref()
    }

    fn print2d(&self) {
        println!("[");
        let rows: usize = self.data.len() / self.size;
        let mut skip: usize = 0;
        for _i in 0..rows {
            let row: Vec<&f64> = self
                .data
                .iter()
                .skip(skip)
                .take(self.size)
                .into_iter()
                .collect();
            print!(" ");
            println!("{:?}", row);
            skip += self.size
        }
        println!("]")
    }

    fn print3d(&self) {
        print!("[");
        let arrays: usize = self.data.len() / self.dim;
        for a in 0..arrays {
            let array: Vec<&f64> = self.data.iter().skip(0).take(arrays).into_iter().collect();
        }
        print!("]");
    }
}

#[test]
fn test_matrix_init() {
    let elements: Vec<Vec<f64>> = vec![
        vec![1.2, 2.4, 3.5],
        vec![4.7, 6.1, 7.2],
        vec![7.0, 1.0, 7.5],
    ];
    let matrix: Matrix = Matrix::matrix2d(elements);

    matrix.print();
}
