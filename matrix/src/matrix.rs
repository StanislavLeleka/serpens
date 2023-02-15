pub struct Matrix<T> {
    elements: Vec<T>,
}

impl<T> Matrix<T> {
    pub fn elements(&self) -> &[T] {
        self.elements.as_ref()
    }
}
