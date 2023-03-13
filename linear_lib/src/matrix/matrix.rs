use rand::distributions::uniform::SampleUniform;

use crate::generator::Generator;

use super::size::Size;

pub struct Matrix<T> {
    elements: Vec<T>,
    size: Size,
}

impl<T> Matrix<T>
where
    T: Copy + Sized + PartialOrd + SampleUniform,
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

    pub fn get_row(&self, row: usize) -> Vec<&T> {
        self.elements
            .iter()
            .skip(row * self.size.cols())
            .take(self.size.cols())
            .into_iter()
            .collect()
    }

    pub fn size(&self) -> &Size {
        &self.size
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

    fn fold(&self, init: f64, f: fn(f64, &f64) -> f64) -> f64 {
        self.elements.iter().fold(init, f)
    }
}

#[cfg(test)]
mod test {
    use super::Matrix;

    #[test]
    fn test_init() {
        let elements: Vec<Vec<f64>> = vec![vec![1.2, 2.4, 3.5], vec![4.7, 6.1, 7.2]];
        let matrix: Matrix<f64> = Matrix::new(&elements);

        println!("{:?}", matrix);

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
            println!("{:?}", row);

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

        println!("{:?}", matrix);

        assert_eq!(matrix.elements.len(), rows * cols);
        assert_eq!(matrix.size().rows(), rows);
        assert_eq!(matrix.size().cols(), cols);

        assert!(matrix.max() <= high);
    }

    fn get_default_matrix() -> Matrix<f64> {
        let elements: Vec<Vec<f64>> = vec![
            vec![1.2, 2.4, 3.5],
            vec![4.7, 6.1, 7.2],
            vec![7.0, 1.0, 7.5],
        ];
        Matrix::new(&elements)
    }
}
