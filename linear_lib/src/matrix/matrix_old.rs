use super::{dimension::Dimension, shape::Shape};
use rand::{rngs::ThreadRng, Rng};
use std::ops;

pub struct Matrixold {
    dim: Dimension,
    size: usize,
    shape: Shape,
    data: Vec<f64>,
}

impl Matrixold {
    fn new(dim: Dimension, size: usize, shape: Shape, data: Vec<f64>) -> Self {
        Self {
            dim,
            size,
            shape,
            data,
        }
    }

    pub fn matrix(elements: Vec<f64>) -> Matrixold {
        let size: usize = elements.len();
        let shape: Shape = Shape::new(1, size, 0);
        Self::new(Dimension::OneDim, size, shape, elements)
    }

    pub fn matrix2d(elements: Vec<Vec<f64>>) -> Matrixold {
        let size: usize = elements[0].len();
        let shape: Shape = Shape::new(size, elements.len(), 0);
        let mut data: Vec<f64> = vec![];

        elements
            .into_iter()
            .for_each(|mut row: Vec<f64>| data.append(&mut row));

        Self::new(Dimension::TwoDim, size, shape, data)
    }

    pub fn matrix3d(elements: Vec<Vec<Vec<f64>>>) -> Matrixold {
        let size: usize = elements[0][0].len();
        let shape: Shape = Shape::new(size, elements[0].len(), elements.len());
        let mut data: Vec<f64> = vec![];

        elements.into_iter().for_each(|row: Vec<Vec<f64>>| {
            row.into_iter()
                .for_each(|mut inner: Vec<f64>| data.append(&mut inner));
        });

        Self::new(Dimension::ThreeDim, size, shape, data)
    }

    pub fn range(left: usize, right: usize) -> Matrixold {
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

    pub fn random(low: f64, high: f64, size: (usize, usize)) -> Matrixold {
        let mut shape: Shape = Shape::new(size.0, size.1, 0);
        let dim: Dimension = if size.1 > 1 {
            Dimension::TwoDim
        } else {
            Dimension::OneDim
        };

        let mut data: Vec<f64> = vec![];

        if let (_, 1) = size {
            data = Self::gen_range(low, high, size.0)
        } else {
            shape = Shape::new(size.0, size.1, 0);
            (0..size.1).for_each(|_r| {
                data.append(&mut Self::gen_range(low, high, size.0));
            });
        }

        Matrixold::new(dim, size.0, shape, data)
    }

    pub fn get(&self, row: usize, col: usize) -> f64 {
        let index: usize = row * self.shape.x() + col;
        self.data[index]
    }

    pub fn set(&mut self, row: usize, col: usize, val: f64) {
        let index: usize = row * self.shape.x() + col;
        self.data[index] = val;
    }

    pub fn get_row(&self, row: usize) -> Vec<&f64> {
        self.data()
            .iter()
            .skip(row * self.size)
            .take(self.size)
            .into_iter()
            .collect()
    }

    pub fn get_column(&self, col: usize) -> Vec<&f64> {
        let mut column: Vec<&f64> = vec![&0.0; self.shape.y()];
        for i in 0..self.shape.y() {
            let index: usize = col + (i * self.size);
            column[i] = &self.data[index];
        }
        column
    }

    pub fn dot(&self, matrix: &Matrixold) -> Matrixold {
        if self.dim != matrix.dim {
            panic!("dimensions of matrices should be same");
        }

        match self.dim {
            Dimension::OneDim => {
                let product: f64 = Self::vectors_dot(self.data(), matrix.data());
                Matrixold::new(Dimension::OneDim, 1, Shape::new(1, 1, 0), vec![product])
            }
            Dimension::TwoDim => todo!(),
            Dimension::ThreeDim => todo!(),
        }
    }

    pub fn mean(&self) -> f64 {
        match self.dim {
            Dimension::OneDim => {
                self.data.iter().fold(0.0, |mut s, v| {
                    s += v;
                    s
                }) / (self.data.len() as f64)
            }
            Dimension::TwoDim => todo!(),
            Dimension::ThreeDim => todo!(),
        }
    }

    pub fn sum(&self) -> f64 {
        self.data.iter().fold(0.0, |mut s, v| {
            s += v;
            s
        })
    }

    pub fn transpose(&self) -> Matrixold {
        let mut transposed = self.copy();
        transposed.set_data(vec![0.0; self.data.len()]);

        for r in 0..self.shape.y() {
            for c in 0..self.shape.x() {
                transposed.set(r, c, self.get(c, r));
            }
        }

        transposed
    }

    pub fn dot_matrix(left: &Matrixold, right: &Matrixold) -> Matrixold {
        let mut copy: Matrixold = left.copy();
        copy.set_data(vec![0.0; left.data.len()]);

        for i in 0..left.shape().y() {
            for j in 0..right.shape().x() {
                for k in 0..left.shape().x() {
                    let val: f64 = copy.get(i, j) + left.get(i, k) * right.get(k, j);
                    copy.set(i, j, val);
                }
            }
        }
        copy
    }

    pub fn dot_vector(matrix: &Matrixold, vector: &Matrixold) -> Matrixold {
        let mut copy: Matrixold = vector.copy();
        copy.set_data(vec![0.0; vector.data.len()]);

        for i in 0..matrix.shape().y() {
            for j in 0..vector.shape().x() {
                for k in 0..matrix.shape().x() {
                    let val: f64 = copy.get(i, j) + matrix.get(i, k) * vector.get(k, j);
                    copy.set(i, j, val);
                }
            }
        }

        copy
    }

    pub fn copy(&self) -> Matrixold {
        let data: Vec<f64> = self.data.iter().map(|d| d.clone()).collect::<Vec<f64>>();
        Matrixold::new(
            self.dim.clone(),
            self.size.clone(),
            Shape::new(
                self.shape.x().clone(),
                self.shape.y().clone(),
                self.shape.z().clone(),
            ),
            data,
        )
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

    fn vectors_dot(a: &[f64], b: &[f64]) -> f64 {
        let mut product: f64 = 0.0;
        for i in 0..a.len() {
            product += a[i] * b[i];
        }
        return product;
    }
}

impl ops::Add<f64> for Matrixold {
    type Output = Matrixold;

    fn add(mut self, rhs: f64) -> Self::Output {
        let data: Vec<f64> = self.data().iter().map(|e| e + rhs).collect();
        self.set_data(data);
        self
    }
}

impl ops::Add<Matrixold> for Matrixold {
    type Output = Matrixold;

    fn add(mut self, rhs: Matrixold) -> Self::Output {
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

impl ops::Mul<f64> for Matrixold {
    type Output = Matrixold;

    fn mul(mut self, rhs: f64) -> Self::Output {
        let data: Vec<f64> = self.data().iter().map(|e| e * rhs).collect();
        self.set_data(data);
        self
    }
}

#[cfg(test)]
mod test {
    use crate::matrix::{dimension::Dimension, matrix_old::Matrixold};

    #[test]
    fn test_matrix_init() {
        let elements: Vec<f64> = vec![1.2, 3.4, 9.0, 8.3, 9.2];
        let matrix: Matrixold = Matrixold::matrix(elements);
        println!("{:?}", &matrix);

        println!("-----------------");

        let elements: Vec<Vec<f64>> = vec![
            vec![1.2, 2.4, 3.5],
            vec![4.7, 6.1, 7.2],
            vec![7.0, 1.0, 7.5],
        ];
        let matrix: Matrixold = Matrixold::matrix2d(elements);
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
        let matrix: Matrixold = Matrixold::matrix3d(elements);
        println!("{:?}", &matrix);
    }

    #[test]
    fn test_transpose() {
        let elements: Vec<Vec<f64>> = vec![
            vec![1.2, 2.4, 3.5],
            vec![4.7, 6.1, 7.2],
            vec![7.0, 1.0, 7.5],
        ];
        let mut matrix: Matrixold = Matrixold::matrix2d(elements);
        println!("{:?}", &matrix);
        let transposed: Matrixold = matrix.transpose();
        println!("{:?}", transposed);
    }

    #[test]
    fn test_dot_matrix() {
        let left: Matrixold = Matrixold::matrix2d(vec![
            vec![1.2, 2.4, 3.5],
            vec![4.7, 6.1, 7.2],
            vec![7.0, 1.0, 7.5],
        ]);
        let right: Matrixold = Matrixold::matrix2d(vec![
            vec![1.2, 6.3, 3.5],
            vec![2.2, 4.1, 4.2],
            vec![5.4, 0.0, 9.5],
        ]);
        let p: Matrixold = Matrixold::dot_matrix(&left, &right);
        println!("{:?}", p);
    }

    #[test]
    fn test_dot_vector() {
        let matrix: Matrixold = Matrixold::matrix2d(vec![
            vec![1.2, 2.4, 3.5],
            vec![4.7, 6.1, 7.2],
            vec![7.0, 1.0, 7.5],
        ]);
        let vector: Matrixold = Matrixold::matrix(vec![1.2, 6.3, 3.5]);
        let p: Matrixold = Matrixold::dot_vector(&matrix, &vector);
        println!("{:?}", p);
    }

    #[test]
    fn test_random() {
        let matrix: Matrixold = Matrixold::random(-2.0, 2.0, (4, 1));
        println!("{:?}", &matrix);

        let matrix: Matrixold = Matrixold::random(-2.0, 2.0, (4, 4));
        println!("{:?}", &matrix);

        //let matrix: Matrix = Matrix::random(-2.0, 2.0, 4, Dimension::ThreeDim);
        //println!("{:?}", &matrix);
    }

    #[test]
    fn test_add() {
        let mut matrix: Matrixold = Matrixold::random(-2.0, 2.0, (4, 4));
        println!("{:?}", &matrix);

        matrix = matrix + 2.0;
        println!("{:?}", &matrix);

        matrix = matrix * 3.0;
        println!("{:?}", &matrix);

        matrix = matrix + Matrixold::random(-2.0, 2.0, (4, 4));
        println!("{:?}", &matrix);
    }

    #[test]
    fn test_range() {
        let matrix: Matrixold = Matrixold::range(6, 20);
        println!("{:?}", &matrix);
    }

    #[test]
    fn test_get() {
        let elements: Vec<Vec<f64>> = vec![
            vec![0.0, 3.0],
            vec![10.0, 7.0],
            vec![20.0, 9.0],
            vec![30.0, 14.0],
            vec![40.0, 15.0],
        ];
        let matrix: Matrixold = Matrixold::matrix2d(elements);
        println!("{:?}", matrix.get(4, 0));
    }

    #[test]
    fn test_get_row() {
        let elements: Vec<Vec<f64>> = vec![
            vec![0.0, 3.0],
            vec![10.0, 7.0],
            vec![20.0, 9.0],
            vec![30.0, 14.0],
            vec![40.0, 15.0],
        ];
        let matrix: Matrixold = Matrixold::matrix2d(elements);

        println!("{:?}", matrix.get_row(0));
        println!("{:?}", matrix.get_row(1));
        println!("{:?}", matrix.get_row(2));
    }

    #[test]
    fn test_get_column() {
        let elements: Vec<Vec<f64>> = vec![
            vec![0.0, 3.0],
            vec![10.0, 7.0],
            vec![20.0, 9.0],
            vec![30.0, 14.0],
            vec![40.0, 15.0],
        ];
        let matrix: Matrixold = Matrixold::matrix2d(elements);

        println!("{:?}", matrix.get_column(0));
        println!("{:?}", matrix.get_column(1));
    }
}
