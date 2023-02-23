use std::fmt;

use crate::{
    frame::{column::Column, frame::Frame},
    print_helper::PrintHelper,
};

impl<'a> fmt::Debug for Frame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        PrintHelper::print_headers(&self.headers(), f);
        PrintHelper::print_columns(&self, f);

        Ok(())
    }
}

impl<'a> fmt::Debug for Column {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        PrintHelper::print_headers(&[self.name().to_string()], f);
        PrintHelper::print_column(self, f);

        Ok(())
    }
}
