use std::fmt::{self};

use crate::{
    matrix::matrix::Matrix,
    print_helper::PrintHelper,
    vector::{shape::Shape, vector::Vector},
};

impl fmt::Debug for Vector<i32> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Shape::Row = self.shape() {
            writeln!(f, "{:?}", self.elements()).unwrap()
        } else {
            PrintHelper::print_col(self.elements(), *self.elements().iter().max().unwrap())
        }
        Ok(())
    }
}

impl fmt::Debug for Vector<f64> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Shape::Row = self.shape() {
            writeln!(f, "{:?}", self.elements()).unwrap()
        } else {
            let max = self
                .elements()
                .iter()
                .fold(f64::NEG_INFINITY, |a, &b| a.max(b));
            PrintHelper::print_col(self.elements(), max)
        }
        Ok(())
    }
}

impl fmt::Debug for Matrix<f64> {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        PrintHelper::print_rows(self, self.max());
        Ok(())
    }
}

impl fmt::Debug for Matrix<i32> {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        PrintHelper::print_rows(self, self.max());
        Ok(())
    }
}
