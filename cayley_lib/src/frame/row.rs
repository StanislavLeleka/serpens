pub struct Row {
    index: usize,
    value: f64,
}

impl Row {
    pub fn new(index: usize, value: f64) -> Self {
        Self { index, value }
    }

    pub fn copy(&self) -> Row {
        Row::new(self.index.clone(), self.value.clone())
    }

    pub fn value(&self, round: bool) -> f64 {
        if round {
            return (self.value * 100.0).round() / 100.0;
        }
        self.value
    }

    pub fn set_value(&mut self, value: f64) {
        self.value = value;
    }

    pub fn index(&self) -> usize {
        self.index
    }
}
