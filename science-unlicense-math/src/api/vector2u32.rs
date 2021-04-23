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

pub struct Vector2u32 {
    pub system: Box<dyn SampleSystem>,
    pub x: u32,
    pub y: u32,
}

impl Vector2u32 {
    pub fn new_empty() -> Self {
        return Vector2u32::new_data(0, 0);
    }

    pub fn new_data(x: u32, y: u32) -> Self {
        return Vector2u32::new(Box::new(samplesystems::create(2)), x, y);
    }

    pub fn new(system: Box<dyn SampleSystem>, x: u32, y: u32) -> Self {
        return Vector2u32 { system:system, x: x, y: y };
    }
}

impl Tuple for Vector2u32 {
    fn get_sample_system(&self) -> Box<dyn SampleSystem> {
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

    fn set(&mut self, index: u32, value: f64) {
        match index {
            0 => self.x = value as u32,
            1 => self.y = value as u32,
            _ => panic_any(logging::format(MESSAGE_INVALID_COORD, &index)),
        }
    }
}

impl Vector for Vector2u32 {}
