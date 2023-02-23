pub struct Row {
    index: usize,
    value: f64,
}

impl Row {
    pub fn new(index: usize, value: f64) -> Self {
        Self { index, value }
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn set_value(&mut self, value: f64) {
        self.value = value;
    }
}
