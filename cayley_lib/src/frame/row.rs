pub struct Row {
    name: String,
    value: f64,
    index: usize,
}

impl Row {
    pub fn new(name: String, value: f64, index: usize) -> Self {
        Self { name, value, index }
    }
}
