use crate::{dimension::Dimension, matrix::Matrix};

pub struct Printer {}

impl Printer {
    pub fn print_matrix(matrix: &Matrix) {
        match matrix.dim() {
            Dimension::OneDim => println!("{:?}", matrix.data()),
            Dimension::TwoDim => Self::print_2d_matrix(matrix),
            Dimension::ThreeDim => Self::print_3d_matrix(matrix),
        }
    }

    fn print_2d_matrix(matrix: &Matrix) {
        let mut skip: usize = 0;
        Self::print_rows(matrix, matrix.shape().y(), &mut skip);
    }

    fn print_3d_matrix(matrix: &Matrix) {
        println!("[");

        let mut skip: usize = 0;
        for _ in 0..matrix.shape().z() {
            Self::print_rows(matrix, matrix.shape().y(), &mut skip);
        }

        print!("]");
    }

    fn print_rows(matrix: &Matrix, rows: usize, skip: &mut usize) {
        println!("[");

        for _ in 0..rows {
            let row: Vec<&f64> = matrix
                .data()
                .iter()
                .skip(*skip)
                .take(matrix.size())
                .into_iter()
                .collect();
            *skip += matrix.size();

            println!("  {:?}", row);
        }

        println!("]")
    }
}
