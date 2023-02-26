use std::fmt;

use crate::frame::{column::Column, frame::Frame};

pub struct PrintHelper {}

macro_rules! index_format {
    () => {
        "{:<3} "
    };
}

macro_rules! cell_format {
    () => {
        "{:>15} "
    };
}

impl PrintHelper {
    pub fn print_headers(headers: &[String], f: &mut fmt::Formatter<'_>) {
        write!(f, index_format!(), "").unwrap();
        for c in headers {
            write!(f, cell_format!(), c).unwrap();
        }
        write!(f, "{}\n", "").unwrap();
    }

    pub fn print_columns(frame: &Frame, f: &mut fmt::Formatter<'_>) {
        let columns: &Vec<Column> = frame.columns();
        for r in 0..columns[0].rows().len() {
            write!(f, index_format!(), columns[0].rows()[r].index()).unwrap();
            for c in 0..columns.len() {
                write!(f, cell_format!(), columns[c].rows()[r].value(true)).unwrap();
            }
            write!(f, "{}\n", "").unwrap();
        }
        writeln!(f, "{}", "").unwrap();
    }

    pub fn print_column<'a>(column: &Column, f: &mut fmt::Formatter<'_>) {
        for y in 0..column.rows().len() {
            write!(f, index_format!(), column.rows()[y].index()).unwrap();
            write!(f, cell_format!(), column.rows()[y].value(true)).unwrap();
            write!(f, "{}\n", "").unwrap();
        }
        writeln!(f, "{}", "").unwrap();
    }
}
