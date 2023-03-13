use std::fmt::{self, Display};

pub struct PrintHelper {}

impl PrintHelper {
    pub fn print_col<T>(elements: &[T], max: T)
    where
        T: Sized + Display + Copy,
    {
        if elements.len() == 0 {
            println!("[]");
            return;
        }

        let width: usize = max.to_string().len();

        let first = elements.first().unwrap();
        println!("⎡ {first:>width$} ⎤");

        for i in 1..(elements.len() - 1) {
            let element = elements[i];
            println!("| {element:>width$} |");
        }

        let last = elements.last().unwrap();
        println!("⎣ {last:>width$} ⎦");
    }
}
