use std::fmt;

enum Dimension {
    One,
    Two,
    Three,
}

struct Shape(usize, usize);

pub struct Matrix {
    dim: Dimension,
    size: usize,
    shape: Shape,
    data: Vec<f64>,
}

impl Matrix {
    pub fn matrix(elements: Vec<f64>) -> Matrix {
        return Matrix {
            dim: Dimension::One,
            size: elements.len(),
            shape: Shape(1, elements.len()),
            data: elements,
        };
    }

    pub fn matrix2d(elements: Vec<Vec<f64>>) -> Matrix {
        let size: usize = elements[0].len();
        let shape: Shape = Shape(elements.len(), size);
        let mut data: Vec<f64> = vec![];
        elements
            .into_iter()
            .for_each(|mut row: Vec<f64>| data.append(&mut row));

        return Matrix {
            dim: Dimension::Two,
            shape,
            size,
            data,
        };
    }

    pub fn matrix3d(elements: Vec<Vec<Vec<f64>>>) -> Matrix {
        let size: usize = elements[0][0].len();
        let shape: Shape = Shape(elements.len(), size);
        let mut data: Vec<f64> = vec![];
        elements.into_iter().for_each(|row: Vec<Vec<f64>>| {
            row.into_iter()
                .for_each(|mut inner: Vec<f64>| data.append(&mut inner));
        });
        return Matrix {
            dim: Dimension::Three,
            shape,
            size,
            data,
        };
    }

    pub fn print(&self) {
        match self.dim {
            Dimension::One => println!("{:?}", self.data()),
            Dimension::Two => self.print2d(),
            Dimension::Three => self.print3d(),
        }
    }

    pub fn data(&self) -> &[f64] {
        self.data.as_ref()
    }

    fn print2d(&self) {
        println!("[");
        let mut skip: usize = 0;
        for _ in 0..self.shape.0 {
            let row: Vec<&f64> = self
                .data
                .iter()
                .skip(skip)
                .take(self.size)
                .into_iter()
                .collect();
            println!("{:?}", row);
            skip += self.size
        }
        println!("]")
    }

    fn print3d(&self) {
        println!("[");
        let mut skip: usize = 0;
        for _ in 0..self.shape.0 {
            println!("  [");
            for _ in 0..self.shape.1 {
                let row: Vec<&f64> = self
                    .data
                    .iter()
                    .skip(skip)
                    .take(self.size)
                    .into_iter()
                    .collect();
                println!("    {:?}", row);
                skip += self.size
            }
            println!("  ]");
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

    //println!("{:?}", matrix.shape);
    matrix.print();

    let elements: Vec<Vec<Vec<f64>>> = vec![
        vec![
            vec![1.2, 2.4, 3.5],
            vec![4.7, 6.1, 7.2],
            vec![7.0, 1.0, 7.5],
        ],
        vec![
            vec![1.2, 6.3, 3.5],
            vec![2.2, 4.1, 4.2],
            vec![5.4, 0.0, 9.5],
        ],
    ];
    let matrix: Matrix = Matrix::matrix3d(elements);
    //println!("{:?}", matrix.shape);

    matrix.print3d();
}
