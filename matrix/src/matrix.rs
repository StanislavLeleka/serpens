use crate::{dimension::Dimension, printer::Printer, shape::Shape};
use rand::{rngs::ThreadRng, Rng};

pub struct Matrix {
    dim: Dimension,
    size: usize,
    shape: Shape,
    data: Vec<f64>,
}

impl Matrix {
    fn new(dim: Dimension, size: usize, shape: Shape, data: Vec<f64>) -> Self {
        Self {
            dim,
            size,
            shape,
            data,
        }
    }

    pub fn matrix(elements: Vec<f64>) -> Matrix {
        let size: usize = elements.len();
        let shape: Shape = Shape::new(1, size, 0);
        Self::new(Dimension::OneDim, size, shape, elements)
    }

    pub fn matrix2d(elements: Vec<Vec<f64>>) -> Matrix {
        let size: usize = elements[0].len();
        let shape: Shape = Shape::new(size, elements.len(), 0);
        let mut data: Vec<f64> = vec![];

        elements
            .into_iter()
            .for_each(|mut row: Vec<f64>| data.append(&mut row));

        Self::new(Dimension::TwoDim, size, shape, data)
    }

    pub fn matrix3d(elements: Vec<Vec<Vec<f64>>>) -> Matrix {
        let size: usize = elements[0][0].len();
        let shape: Shape = Shape::new(size, elements[0].len(), elements.len());
        let mut data: Vec<f64> = vec![];

        elements.into_iter().for_each(|row: Vec<Vec<f64>>| {
            row.into_iter()
                .for_each(|mut inner: Vec<f64>| data.append(&mut inner));
        });

        Self::new(Dimension::ThreeDim, size, shape, data)
    }

    pub fn random(size: usize, dim: Dimension) -> Matrix {
        let mut rng: ThreadRng = rand::thread_rng();
        match dim {
            Dimension::OneDim => Matrix::new(
                dim,
                size,
                Shape::new(size, 1, 0),
                (0..size)
                    .map(|_| rng.gen_range(0.0..(size as f64)))
                    .collect(),
            ),
            Dimension::TwoDim => Matrix::new(
                dim,
                size,
                Shape::new(size, size, 0),
                (0..(size * size))
                    .map(|_| rng.gen_range(0.0..((size * size) as f64)))
                    .collect(),
            ),
            Dimension::ThreeDim => Matrix::new(
                dim,
                size,
                Shape::new(size, size, size),
                (0..(size * size * size))
                    .map(|_| rng.gen_range(0.0..((size * size * size) as f64)))
                    .collect(),
            ),
        }
    }

    pub fn dim(&self) -> &Dimension {
        &self.dim
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn shape(&self) -> &Shape {
        &self.shape
    }

    pub fn data(&self) -> &[f64] {
        self.data.as_ref()
    }
}

#[test]
fn test_matrix_init() {
    let elements: Vec<f64> = vec![1.2, 3.4, 9.0, 8.3, 9.2];
    let matrix: Matrix = Matrix::matrix(elements);
    Printer::print_matrix(matrix);

    println!("-----------------");

    let elements: Vec<Vec<f64>> = vec![
        vec![1.2, 2.4, 3.5],
        vec![4.7, 6.1, 7.2],
        vec![7.0, 1.0, 7.5],
    ];
    let matrix: Matrix = Matrix::matrix2d(elements);
    Printer::print_matrix(matrix);

    println!("-----------------");

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
    Printer::print_matrix(matrix);
}

#[test]
fn test_random() {
    let matrix: Matrix = Matrix::random(4, Dimension::OneDim);
    Printer::print_matrix(matrix);

    let matrix: Matrix = Matrix::random(4, Dimension::TwoDim);
    Printer::print_matrix(matrix);

    let matrix: Matrix = Matrix::random(4, Dimension::ThreeDim);
    Printer::print_matrix(matrix);
}
