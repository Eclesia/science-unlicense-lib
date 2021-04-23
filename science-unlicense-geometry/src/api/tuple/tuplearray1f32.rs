//
// Public Domain - unlicense.science
//
use crate::api::tuple::{TupleArray, TupleSpace};
use science_unlicense_math::api::Scalar1f32;
use science_unlicense_math::api::Tuple;

pub struct TupleArray1f32 {
    values: Vec<f32>,
}

impl TupleArray1f32 {
    pub fn new(size: u32) -> Self {
        return TupleArray1f32 {
            values: vec![0f32; size as usize],
        };
    }
}

impl TupleSpace for TupleArray1f32 {
    fn get(&self, coordinate: Box<dyn Tuple>) -> Box<dyn Tuple> {
        let x = self.values[coordinate.get(0) as usize];
        let s = Scalar1f32::new(x);
        return Box::new(s);
    }
}

impl TupleArray for TupleArray1f32 {}
