use std::{
    fmt::Display,
    ops::{AddAssign, Mul},
};

use rand::distributions::uniform::SampleUniform;

pub trait Num: AddAssign + Copy + Sized + Display + PartialOrd + SampleUniform {
    fn zero() -> Self;
    fn from_usize(u: usize) -> Self;
}

impl Num for i32 {
    fn zero() -> Self {
        0
    }

    fn from_usize(u: usize) -> Self {
        u as i32
    }
}

impl Num for f64 {
    fn zero() -> Self {
        0.0
    }

    fn from_usize(u: usize) -> Self {
        u as f64
    }
}
