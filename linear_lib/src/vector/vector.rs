use super::shape::Shape;

pub struct Vector<T> {
    elements: Vec<T>,
    shape: Shape,
}

impl<T> Vector<T> {
    pub fn new(elements: Vec<T>, shape: Shape) -> Self {
        Self { elements, shape }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn transpose(&mut self) {
        if self.shape == Shape::Row {
            self.shape = Shape::Col
        } else {
            self.shape = Shape::Row
        }
    }
}

impl Vector<i32> {
    pub fn dot(&self, right: &Vector<i32>) -> Result<i32, &'static str> {
        if self.size() != right.size() {
            return Err("invalid vector size");
        }

        let mut prod: i32 = 0;
        for i in 0..self.elements.len() {
            prod += self.elements[i] * right.elements[i];
        }

        Ok(prod)
    }
}

#[cfg(test)]
mod test {
    use crate::vector::shape::Shape;

    use super::Vector;

    #[test]
    fn test_init() {
        let elements: Vec<i32> = vec![1, 3, 5, 2, 7];
        let vec: Vector<i32> = Vector::new(elements, Shape::Row);
    }

    #[test]
    fn test_dot() {
        let elements: Vec<i32> = vec![1, 3, 5, 2, 7];
        let vec: Vector<i32> = Vector::new(elements, Shape::Row);
        let dot: Result<i32, &str> = vec.dot(&vec);

        match dot {
            Ok(d) => {
                println!("{:?}", d)
            }
            Err(e) => {
                println!("{}", e)
            }
        }
    }
}
