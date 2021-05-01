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
use std::sync::Arc;

pub struct Scalar1f32 {
    pub system: Arc<dyn SampleSystem>,
    pub x: f32,
}

impl Scalar1f32 {

    pub fn new_empty() -> Self {
        return Scalar1f32::new(Arc::new(samplesystems::create(1)), 0f32);
    }

    pub fn new_data(x: f32) -> Self {
        return Scalar1f32::new(Arc::new(samplesystems::create(1)),  x);
    }

    pub fn new(system: Arc<dyn SampleSystem>, x: f32) -> Self {
        return Scalar1f32 { system: system, x: x };
    }
}

impl Tuple for Scalar1f32 {
    fn get_sample_system(&self) -> Arc<dyn SampleSystem> {
        return self.system.clone();
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

    fn set(&mut self, index: u32, value: f64) {
        match index {
            0 => self.x = value as f32,
            _ => panic_any(logging::format(MESSAGE_INVALID_COORD, &index)),
        }
    }
}

impl Vector for Scalar1f32 {}
