use std::ops::{Add, Div, Mul};

use crate::generator::Generator;
use crate::num::Num;

use super::size::Size;

pub struct Matrix<T> {
    elements: Vec<T>,
    size: Size,
}

impl<T> Matrix<T>
where
    T: Mul<Output = T> + Add<Output = T> + Num,
{
    pub fn new(data: &Vec<Vec<T>>) -> Self {
        Matrix {
            elements: Self::to_row_major(data),
            size: Self::get_size(data),
        }
    }

    pub fn random(low: T, high: T, rows: usize, cols: usize) -> Matrix<T> {
        let mut elements: Vec<T> = vec![];
        for _row in 0..rows {
            elements.append(&mut Generator::random_elements(low, high, cols));
        }

        Matrix {
            elements,
            size: Size::new(rows, cols),
        }
    }

    pub fn transpose(&self) -> Matrix<T> {
        let mut elements: Vec<T> = vec![T::zero(); self.elements.len()];
        let rows: usize = self.size().rows();
        let cols: usize = self.size().cols();

        for i in 0..rows {
            for j in 0..cols {
                elements[j * rows + i] = self.elements[i * cols + j];
            }
        }

        Matrix {
            elements,
            size: Size::new(rows, cols),
        }
    }

    pub fn product(&self, right: &Matrix<T>) -> Result<Matrix<T>, &'static str> {
        if self.size().cols() * right.size().cols() != right.elements.len() {
            return Err("product is not defined");
        }

        let elements: Vec<T> = vec![T::zero(); self.size().rows() * right.size().cols()];
        let rows = elements.len() / right.size().cols();
        let mut matrix = Matrix {
            elements,
            size: Size::new(rows, right.size().cols()),
        };

        for row in 0..rows {
            for col in 0..matrix.size().cols() {
                let mut cell: T = T::zero();
                for i in 0..right.size().rows() {
                    cell += *self.get(row, i).unwrap() * *right.get(i, col).unwrap();
                }
                matrix.set(row, col, cell);
            }
        }

        Ok(matrix)
    }

    pub fn equals(&self, matrix: &Matrix<T>) -> bool {
        if self.elements.len() != matrix.elements.len() {
            return false;
        }

        for i in 0..self.elements.len() {
            if self.elements[i] != matrix.elements[i] {
                return false;
            }
        }
        true
    }

    pub fn get_row(&self, row: usize) -> Vec<&T> {
        self.elements
            .iter()
            .skip(row * self.size.cols())
            .take(self.size.cols())
            .into_iter()
            .collect()
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.elements.get(self.get_index(row, col))
    }

    pub fn set(&mut self, row: usize, col: usize, val: T) {
        let index: usize = self.get_index(row, col);
        self.elements[index] = val;
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn sum(&self) -> T {
        self.fold(T::zero(), |mut a, &b| {
            a += b;
            a
        })
    }

    pub fn mean(&self) -> T
    where
        T: Div<Output = T>,
    {
        self.sum() / T::from_usize(self.elements.len())
    }

    fn to_row_major(data: &Vec<Vec<T>>) -> Vec<T> {
        data.iter().fold(vec![], |mut res, el| {
            let mut row = el.clone();
            res.append(&mut row);
            res
        })
    }

    fn get_size(data: &Vec<Vec<T>>) -> Size {
        let rows: usize = data.len();
        let cols: usize = data.first().unwrap().len();
        Size::new(rows, cols)
    }

    fn get_index(&self, row: usize, col: usize) -> usize {
        row * self.size().cols() + col
    }

    fn fold(&self, init: T, f: fn(T, &T) -> T) -> T {
        self.elements.iter().fold(init, f)
    }
}

impl Matrix<i32> {
    pub fn max(&self) -> i32 {
        self.elements.iter().max().unwrap().clone()
    }

    pub fn min(&self) -> i32 {
        self.elements.iter().min().unwrap().clone()
    }
}

impl Matrix<f64> {
    pub fn max(&self) -> f64 {
        self.fold(f64::NEG_INFINITY, |a, &b| a.max(b))
    }

    pub fn min(&self) -> f64 {
        self.fold(f64::INFINITY, |a, &b| a.min(b))
    }
}

#[cfg(test)]
mod test {
    use super::Matrix;

    #[test]
    fn test_init() {
        let elements: Vec<Vec<f64>> = vec![vec![1.2, 2.4, 3.5], vec![4.7, 6.1, 7.2]];
        let matrix: Matrix<f64> = Matrix::new(&elements);

        assert_eq!(matrix.size().rows(), 2);
        assert_eq!(matrix.size().cols(), 3);
        assert_eq!(
            matrix.elements.len(),
            matrix.size().rows() * matrix.size().cols()
        );
    }

    #[test]
    fn test_get_row() {
        let matrix: Matrix<f64> = get_default_matrix();
        for r in 0..matrix.size().rows() {
            let row = matrix.get_row(r);
            assert_eq!(row.len(), matrix.size().cols());
        }
    }

    #[test]
    fn test_random() {
        let low: i32 = 1;
        let high: i32 = 1000;
        let rows: usize = 50;
        let cols: usize = 15;

        let matrix: Matrix<i32> = Matrix::random(low, high, rows, cols);

        assert_eq!(matrix.elements.len(), rows * cols);
        assert_eq!(matrix.size().rows(), rows);
        assert_eq!(matrix.size().cols(), cols);
        assert!(matrix.max() <= high);
    }

    #[test]
    fn test_get() {
        let matrix: Matrix<f64> = get_default_matrix();

        assert_eq!(*matrix.get(0, 0).unwrap(), 1.2);
        assert_eq!(*matrix.get(1, 1).unwrap(), 6.1);
        assert_eq!(*matrix.get(2, 2).unwrap(), 7.5);
    }

    #[test]
    fn test_set() {
        let mut matrix: Matrix<f64> = get_default_matrix();

        matrix.set(0, 0, 999.999);
        matrix.set(1, 2, 777.999);
        matrix.set(2, 1, 1.0);

        assert_eq!(*matrix.get(0, 0).unwrap(), 999.999);
        assert_eq!(*matrix.get(1, 2).unwrap(), 777.999);
        assert_eq!(*matrix.get(2, 1).unwrap(), 1.0);
    }

    #[test]
    fn test_sum() {
        let matrix: Matrix<f64> = get_default_matrix();
        assert!(matrix.sum() - 40.6 < f64::EPSILON);
    }

    #[test]
    fn test_mean() {
        let matrix: Matrix<f64> = get_default_matrix();
        assert!(matrix.mean() - (40.6 / 9.0) < f64::EPSILON);
    }

    #[test]
    fn test_transpose() {
        let matrix: Matrix<f64> = get_default_matrix();
        let tm: Matrix<f64> = matrix.transpose();
        let dtm: Matrix<f64> = get_default_matrix_transposed();

        for i in 0..tm.size().rows() {
            for j in 0..tm.size().cols() {
                assert_eq!(tm.get(i, j), dtm.get(i, j));
            }
        }
    }

    #[test]
    fn test_equals() {
        let matrix1: Matrix<f64> = get_default_matrix();
        let matrix2: Matrix<f64> = get_default_matrix();

        assert!(matrix1.equals(&matrix2))
    }

    #[test]
    fn test_product() {
        let matrix1: Matrix<i32> = Matrix::new(&vec![vec![1, 5], vec![2, 3], vec![1, 7]]);
        let matrix2: Matrix<i32> = Matrix::new(&vec![vec![1, 2, 3, 7], vec![5, 2, 8, 1]]);
        let res: Matrix<i32> = matrix1.product(&matrix2).unwrap();

        assert!(res.equals(&get_default_matrix_product()))
    }

    fn get_default_matrix() -> Matrix<f64> {
        let elements: Vec<Vec<f64>> = vec![
            vec![1.2, 2.4, 3.5],
            vec![4.7, 6.1, 7.2],
            vec![7.0, 1.0, 7.5],
        ];
        Matrix::new(&elements)
    }

    fn get_default_matrix_transposed() -> Matrix<f64> {
        let elements: Vec<Vec<f64>> = vec![
            vec![1.2, 4.7, 7.0],
            vec![2.4, 6.1, 1.0],
            vec![3.5, 7.2, 7.5],
        ];
        Matrix::new(&elements)
    }

    fn get_default_matrix_product() -> Matrix<i32> {
        let elements: Vec<Vec<i32>> = vec![
            vec![26, 12, 43, 12],
            vec![17, 10, 30, 17],
            vec![36, 16, 59, 14],
        ];
        Matrix::new(&elements)
    }
}
