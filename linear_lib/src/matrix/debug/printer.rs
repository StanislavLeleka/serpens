use std::fmt;

use crate::matrix::matrix::Matrix;

use super::print_helper::PrintHelper;

impl fmt::Debug for Matrix<f64> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        PrintHelper::print_rows(self, self.max());
        Ok(())
    }
}

impl fmt::Debug for Matrix<i32> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        PrintHelper::print_rows(self, self.max());
        Ok(())
    }
}
