// Public Domain - unlicense.science

use crate::api::Tuple;
use crate::api::Vector;

pub struct Scalar1f32 {
    pub x: f32,
}

impl Scalar1f32 {
    pub fn new(x: f32) -> Self {
        return Scalar1f32 { x: x };
    }
}

impl Tuple for Scalar1f32 {
    fn get_dimension(&self) -> u32 {
        return 1;
    }

    fn get(&self, index: u32) -> f64 {
        match index {
            0 => return self.x as f64,
            _ => panic!("Arg"),
        }
    }

    fn set(&mut self, index: u32, value: f64) {
        match index {
            0 => self.x = value as f32,
            _ => panic!("Arg"),
        }
    }
}

impl Vector for Scalar1f32 {}
