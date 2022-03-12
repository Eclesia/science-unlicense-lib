//
// Public Domain - unlicense.science
//
use crate::system::SampleSystem;
use crate::system::samplesystems;
use crate::tuple::Tuple;
use crate::tuple::Vector;
use crate::tuple::MESSAGE_INVALID_COORD;
use science_unlicense_common::api::logging;
use science_unlicense_common::api::number::{ArithmeticType, Arithmetic};
use std::panic::panic_any;
use std::sync::Arc;

pub struct Vector2u32 {
    pub system: Arc<dyn SampleSystem>,
    pub x: u32,
    pub y: u32,
}

impl Vector2u32 {

    pub fn new_empty() -> Self {
        return Vector2u32::new_data(0, 0);
    }

    pub fn new_data(x: u32, y: u32) -> Self {
        return Vector2u32::new(samplesystems::create(2), x, y);
    }

    pub fn new_system(system: Arc<dyn SampleSystem>) -> Self {
        return Vector2u32 { system: system.clone(), x: 0, y: 0 };
    }

    pub fn new(system: Arc<dyn SampleSystem>, x: u32, y: u32) -> Self {
        return Vector2u32 { system:system, x: x, y: y };
    }
}

impl Tuple for Vector2u32 {
    fn get_sample_system(&self) -> &Arc<dyn SampleSystem> {
        todo!()
    }

    fn get_numeric_type(&self) -> &Arc<dyn ArithmeticType> {
        todo!()
    }

    fn get_sample_count(&self) -> u32 {
        return 1;
    }

    fn get(&self, index: u32) -> f64 {
        match index {
            0 => return self.x as f64,
            1 => return self.y as f64,
            _ => panic_any(logging::format(MESSAGE_INVALID_COORD, &index)),
        }
    }

    fn get_number(&self, _index: u32) -> Box<dyn Arithmetic> {
        todo!()
    }

    fn set(&mut self, index: u32, value: f64) {
        match index {
            0 => self.x = value as u32,
            1 => self.y = value as u32,
            _ => panic_any(logging::format(MESSAGE_INVALID_COORD, &index)),
        }
    }

    fn set_number(&mut self, _index: u32, _value: Box<dyn Arithmetic>) {
        todo!()
    }
}

impl Vector for Vector2u32 {}
