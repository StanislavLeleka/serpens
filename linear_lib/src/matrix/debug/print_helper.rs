use std::fmt::{Debug, Display};

use rand::distributions::uniform::SampleUniform;

use crate::matrix::matrix::Matrix;

pub struct PrintHelper {}

impl PrintHelper {
    pub fn print_rows<T>(matrix: &Matrix<T>, max: T)
    where
        T: Debug + Display + Copy + Sized + PartialOrd + SampleUniform,
    {
        let width: usize = max.to_string().len();
        for r in 0..matrix.size().rows() {
            let row: Vec<&T> = matrix.get_row(r);

            let first = row.first().unwrap();
            if r == 0 {
                print!("⎡ {first:>width$} |");
            } else if r == matrix.size().rows() - 1 {
                print!("⎣ {first:>width$} |");
            } else {
                print!("| {first:>width$} |");
            }

            for i in 1..(row.len() - 1) {
                let element = row[i];
                print!(" {element:>width$} |");
            }

            let last = row.last().unwrap();
            if r == 0 {
                println!(" {last:>width$} ⎤");
            } else if r == matrix.size().rows() - 1 {
                println!(" {last:>width$} ⎦");
            } else {
                println!(" {last:>width$} |");
            }
        }
    }
}
