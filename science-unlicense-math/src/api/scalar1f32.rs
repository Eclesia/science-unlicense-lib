//
// Public Domain - unlicense.science
//
use crate::api::Tuple;
use crate::api::Vector;
use crate::api::MESSAGE_INVALID_COORD;
use science_unlicense_common::api::logging;
use std::panic::panic_any;
use crate::api::system::SampleSystem;
use crate::api::system::samplesystems;
use science_unlicense_common::api::number::{ArithmeticType, Arithmetic};

pub struct Scalar1f32 {
    pub system: Box<dyn SampleSystem>,
    pub x: f32,
}

impl Scalar1f32 {

    pub fn new_empty() -> Self {
        return Scalar1f32::new(Box::new(samplesystems::create(1)), 0f32);
    }

    pub fn new_data(x: f32) -> Self {
        return Scalar1f32::new(Box::new(samplesystems::create(1)),  x);
    }

    pub fn new_system(system: Box<dyn SampleSystem>) -> Self {
        return Scalar1f32 { system: system, x: 0f32 };
    }

    pub fn new(system: Box<dyn SampleSystem>, x: f32) -> Self {
        return Scalar1f32 { system: system, x: x };
    }
}

impl Tuple for Scalar1f32 {

    fn get_sample_system(&self) -> Box<dyn SampleSystem> {
        return self.system.clone();
    }

    fn get_numeric_type(&self) -> Box<dyn ArithmeticType> {
        panic!("todo");
    }

    fn get_sample_count(&self) -> u32 {
        return 1;
    }

    fn get(&self, index: u32) -> f64 {
        match index {
            0 => return self.x as f64,
            _ => panic_any(logging::format(MESSAGE_INVALID_COORD, &index)),
        }
    }

    fn get_number(&self, index: u32) -> Box<dyn Arithmetic> {
        panic!("todo");
    }

    fn set(&mut self, index: u32, value: f64) {
        match index {
            0 => self.x = value as f32,
            _ => panic_any(logging::format(MESSAGE_INVALID_COORD, &index)),
        }
    }

    fn set_number(&mut self, index: u32, value: Box<dyn Arithmetic>) {
        panic!("todo");
    }

}

impl Vector for Scalar1f32 {}
