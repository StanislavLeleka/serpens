pub struct Shape(usize, usize, usize);

impl Shape {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self(x, y, z)
    }

    pub fn x(&self) -> usize {
        self.0
    }

    pub fn y(&self) -> usize {
        self.1
    }

    pub fn z(&self) -> usize {
        self.2
    }
}
