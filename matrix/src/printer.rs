use std::fmt;

use crate::{dimension::Dimension, matrix::Matrix};

impl fmt::Debug for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn print_2d_matrix(matrix: &Matrix, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut skip: usize = 0;
            print_rows(matrix, matrix.shape().y(), &mut skip, f)
        }

        fn print_3d_matrix(matrix: &Matrix, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}\n", "[").unwrap();

            let mut skip: usize = 0;
            for _ in 0..matrix.shape().z() {
                print_rows(matrix, matrix.shape().y(), &mut skip, f).unwrap();
            }

            write!(f, "{}", "]")
        }

        fn print_rows(
            matrix: &Matrix,
            rows: usize,
            skip: &mut usize,
            f: &mut fmt::Formatter<'_>,
        ) -> fmt::Result {
            write!(f, "{}\n", "[").unwrap();

            for _ in 0..rows {
                let row: Vec<&f64> = matrix
                    .data()
                    .iter()
                    .skip(*skip)
                    .take(matrix.size())
                    .into_iter()
                    .collect();

                *skip += matrix.size();
                write!(f, " {:?}\n", row).unwrap();
            }

            write!(f, "{}\n", "]")
        }

        match self.dim() {
            Dimension::OneDim => write!(f, "{:?}", self.data()),
            Dimension::TwoDim => print_2d_matrix(self, f),
            Dimension::ThreeDim => print_3d_matrix(self, f),
        }
    }
}
