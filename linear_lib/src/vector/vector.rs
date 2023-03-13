use std::ops::{AddAssign, Mul};

use rand::distributions::uniform::SampleUniform;

use crate::generator::Generator;

use super::shape::Shape;

pub struct Vector<T> {
    elements: Vec<T>,
    shape: Shape,
}

impl<T> Vector<T>
where
    T: Mul<Output = T> + AddAssign + Copy + Sized + PartialOrd + SampleUniform,
{
    pub fn new(elements: Vec<T>, shape: Shape) -> Self {
        Self { elements, shape }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn transpose(&mut self) {
        self.shape = if self.shape == Shape::Row {
            Shape::Col
        } else {
            Shape::Row
        }
    }

    pub fn random(low: T, high: T, size: usize, shape: Shape) -> Vector<T> {
        Vector {
            elements: Generator::random_elements(low, high, size),
            shape,
        }
    }

    pub fn elements(&self) -> &[T] {
        self.elements.as_ref()
    }

    pub fn shape(&self) -> &Shape {
        &self.shape
    }

    fn inner(&self, right: &Vector<T>, init: T) -> Result<T, &'static str> {
        if self.size() != right.size() {
            return Err("invalid vector size");
        }

        if self.shape != Shape::Row || right.shape != Shape::Col {
            return Err("invalid vectors shape");
        }

        let mut product: T = init;
        for i in 0..self.elements.len() {
            product += self.elements[i] * right.elements[i];
        }
        Ok(product)
    }
}

impl Vector<i32> {
    pub fn dot(&self, right: &Vector<i32>) -> Result<i32, &'static str> {
        return self.inner(right, 0);
    }
}

impl Vector<f64> {
    pub fn dot(&self, right: &Vector<f64>) -> Result<f64, &'static str> {
        return self.inner(right, 0.0);
    }
}

#[cfg(test)]
mod test {
    use crate::vector::shape::Shape;

    use super::Vector;

    static ELEMENTS: &'static [i32] = &[1, 3, 5, 2, 7];

    #[test]
    fn test_init() {
        let vec: Vector<i32> = Vector::new(ELEMENTS.to_vec(), Shape::Col);

        println!("{:?}", vec);

        assert_eq!(*vec.shape(), Shape::Col);
        assert_eq!(vec.elements.len(), ELEMENTS.len());
    }

    #[test]
    fn test_transpose() {
        let mut vec: Vector<i32> = Vector::new(ELEMENTS.to_vec(), Shape::Col);

        assert_eq!(*vec.shape(), Shape::Col);
        vec.transpose();
        assert_eq!(*vec.shape(), Shape::Row);

        println!("{:?}", vec);
    }

    #[test]
    fn test_dot() {
        let left: Vector<i32> = Vector::new(ELEMENTS.to_vec(), Shape::Row);
        let mut right: Vector<i32> = Vector::new(ELEMENTS.to_vec(), Shape::Col);

        let mut dot: Result<i32, &str> = left.dot(&right);
        assert_eq!(dot.unwrap(), 88);

        right.transpose();

        dot = left.dot(&right);
        assert_eq!(dot.unwrap_err(), "invalid vectors shape");
    }

    #[test]
    fn test_random() {
        let size: usize = 100;
        let low: i32 = 0;
        let high: i32 = 1000;
        let v: Vector<i32> = Vector::random(low, high, size, Shape::Col);

        println!("{:?}", v);

        assert_eq!(v.elements().len(), size);
        assert_eq!(*v.shape(), Shape::Col);

        for i in 0..size {
            if v.elements[i] < low || v.elements[i] >= high {
                panic!("{}", "invalid vector element")
            }
        }

        let low: f64 = 1.0;
        let high: f64 = 10.0;
        let v: Vector<f64> = Vector::random(low, high, size, Shape::Row);

        println!("{:?}", v);

        assert_eq!(v.elements().len(), size);
        assert_eq!(*v.shape(), Shape::Row);
    }
}
