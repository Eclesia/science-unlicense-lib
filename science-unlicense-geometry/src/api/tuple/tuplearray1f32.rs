//
// Public Domain - unlicense.science
//
use crate::api::tuple::TupleArray;
use crate::api::tuple::TupleSpace;
use science_unlicense_math::api::Scalar1f32;
use science_unlicense_math::api::Tuple;
use science_unlicense_math::api::system::SampleSystem;
use science_unlicense_math::api::system::samplesystems;
use std::sync::Arc;

pub struct TupleArray1f32 {
    system: Arc<dyn SampleSystem>,
    values: Vec<f32>,
}

impl TupleArray1f32 {
    pub fn new(size: u32) -> Self {
        let ss : Arc<dyn SampleSystem> = samplesystems::create(1);
        return TupleArray1f32 {
            system: ss,
            values: vec![0f32; size as usize],
        };
    }
}

impl TupleSpace for TupleArray1f32 {

    fn get_sample_system(&self) -> &Arc<dyn SampleSystem> {
        return &self.system;
    }

    fn get(&self, coordinate: &dyn Tuple) -> Box<dyn Tuple> {
        let x = self.values[coordinate.get(0) as usize];
        let s = Scalar1f32::new_data(x);
        return Box::new(s);
    }
}

impl TupleArray for TupleArray1f32 {
}
