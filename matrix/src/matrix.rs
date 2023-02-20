use std::ops;

use crate::{dimension::Dimension, shape::Shape};
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

    pub fn range(left: usize, right: usize) -> Matrix {
        let mut data: Vec<f64> = vec![];
        for i in left..(right + 1) {
            data.push(i as f64);
        }
        Self::new(
            Dimension::OneDim,
            data.len(),
            Shape::new(data.len(), 1, 0),
            data,
        )
    }

    pub fn random(low: f64, high: f64, size: usize, dim: Dimension) -> Matrix {
        let mut shape: Shape = Shape::new(size, 1, 0);
        let data: Vec<f64>;

        match dim {
            Dimension::OneDim => data = Self::gen_range(low, high, size),
            Dimension::TwoDim => {
                shape = Shape::new(size, size, 0);
                data = Self::gen_range(low, high, size * size);
            }
            Dimension::ThreeDim => {
                shape = Shape::new(size, size, size);
                data = Self::gen_range(low, high, size * size * size);
            }
        }

        Matrix::new(dim, size, shape, data)
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

    pub fn set_data(&mut self, data: Vec<f64>) {
        self.data = data;
    }

    fn gen_range(low: f64, high: f64, size: usize) -> Vec<f64> {
        let mut rng: ThreadRng = rand::thread_rng();
        (0..size).map(|_| rng.gen_range(low..high)).collect()
    }
}

impl ops::Add<f64> for Matrix {
    type Output = Matrix;

    fn add(mut self, rhs: f64) -> Self::Output {
        let data: Vec<f64> = self.data().iter().map(|e| e + rhs).collect();
        self.set_data(data);
        self
    }
}

impl ops::Add<Matrix> for Matrix {
    type Output = Matrix;

    fn add(mut self, rhs: Matrix) -> Self::Output {
        if self.data().len() != rhs.data().len() {
            panic!("dimensions of matrices should be same")
        }

        self.set_data(
            self.data()
                .iter()
                .zip(rhs.data())
                .map(|(a, b)| a + b)
                .collect(),
        );
        self
    }
}

impl ops::Mul<f64> for Matrix {
    type Output = Matrix;

    fn mul(mut self, rhs: f64) -> Self::Output {
        let data: Vec<f64> = self.data().iter().map(|e| e * rhs).collect();
        self.set_data(data);
        self
    }
}

#[cfg(test)]
mod test {
    use crate::{dimension::Dimension, matrix::Matrix};

    #[test]
    fn test_matrix_init() {
        let elements: Vec<f64> = vec![1.2, 3.4, 9.0, 8.3, 9.2];
        let matrix: Matrix = Matrix::matrix(elements);
        println!("{:?}", &matrix);

        println!("-----------------");

        let elements: Vec<Vec<f64>> = vec![
            vec![1.2, 2.4, 3.5],
            vec![4.7, 6.1, 7.2],
            vec![7.0, 1.0, 7.5],
        ];
        let matrix: Matrix = Matrix::matrix2d(elements);
        println!("{:?}", &matrix);

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
        println!("{:?}", &matrix);
    }

    #[test]
    fn test_random() {
        let matrix: Matrix = Matrix::random(-2.0, 2.0, 4, Dimension::OneDim);
        println!("{:?}", &matrix);

        let matrix: Matrix = Matrix::random(-2.0, 2.0, 4, Dimension::TwoDim);
        println!("{:?}", &matrix);

        let matrix: Matrix = Matrix::random(-2.0, 2.0, 4, Dimension::ThreeDim);
        println!("{:?}", &matrix);
    }

    #[test]
    fn test_add() {
        let mut matrix: Matrix = Matrix::random(-2.0, 2.0, 4, Dimension::TwoDim);
        println!("{:?}", &matrix);

        matrix = matrix + 2.0;
        println!("{:?}", &matrix);

        matrix = matrix * 3.0;
        println!("{:?}", &matrix);

        matrix = matrix + Matrix::random(-2.0, 2.0, 4, Dimension::TwoDim);
        println!("{:?}", &matrix);
    }

    #[test]
    fn test_range() {
        let matrix: Matrix = Matrix::range(6, 20);
        println!("{:?}", &matrix);
    }
}
