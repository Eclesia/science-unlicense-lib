//
// Public Domain - unlicense.science
//
use crate::tuple::Tuple;
use crate::tuple::Vector;
use crate::tuple::MESSAGE_INVALID_COORD;
use science_unlicense_common::api::logging;
use std::panic::panic_any;
use crate::system::SampleSystem;
use crate::system::samplesystems;
use science_unlicense_common::api::number::ArithmeticType;
use science_unlicense_common::api::number::Arithmetic;
use science_unlicense_common::api::number;
use std::sync::Arc;

pub struct Scalar1f32 {
    pub system: Arc<dyn SampleSystem>,
    pub arttype: Arc<dyn ArithmeticType>,
    pub x: f32,
}

impl Scalar1f32 {

    pub fn new_empty() -> Self {
        let ss : Arc<dyn SampleSystem> = samplesystems::create(1);
        return Scalar1f32::new(ss, 0f32);
    }

    pub fn new_data(x: f32) -> Self {
        let ss : Arc<dyn SampleSystem> = samplesystems::create(1);
        return Scalar1f32::new(ss,  x);
    }

    pub fn new_system(system: Arc<dyn SampleSystem>) -> Self {
        let at : Arc<dyn ArithmeticType> =  Arc::new(number::TYPE_FLOAT32);
        return Scalar1f32 { system: system.clone(), arttype: at, x: 0f32 };
    }

    pub fn new(system: Arc<dyn SampleSystem>, x: f32) -> Self {
        let at : Arc<dyn ArithmeticType> =  Arc::new(number::TYPE_FLOAT32);
        return Scalar1f32 { system: system.clone(), arttype: at, x: x };
    }
}

impl Tuple for Scalar1f32 {

    fn get_sample_system(&self) -> &Arc<dyn SampleSystem> {
        return &self.system;
    }

    fn get_numeric_type(&self) -> &Arc<dyn ArithmeticType> {
        return &self.arttype;
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

    fn get_number(&self, _index: u32) -> Box<dyn Arithmetic> {
        panic!("todo");
    }

    fn set(&mut self, index: u32, value: f64) {
        match index {
            0 => self.x = value as f32,
            _ => panic_any(logging::format(MESSAGE_INVALID_COORD, &index)),
        }
    }

    fn set_number(&mut self, _index: u32, _value: Box<dyn Arithmetic>) {
        panic!("todo");
    }

}

impl Vector for Scalar1f32 {}
