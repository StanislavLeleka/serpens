use std::fmt::{self};

use crate::vector::{shape::Shape, vector::Vector};

use super::print_helper::PrintHelper;

impl fmt::Debug for Vector<i32> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Shape::Row = self.shape() {
            println!("{:?}", self.elements())
        } else {
            PrintHelper::print_col(self.elements(), *self.elements().iter().max().unwrap())
        }
        Ok(())
    }
}

impl fmt::Debug for Vector<f64> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Shape::Row = self.shape() {
            println!("{:?}", self.elements())
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
