//
// Public Domain - unlicense.science
//
use crate::api::tuple::TupleArray;
use crate::api::tuple::TupleArray1f32;
use science_unlicense_common::api::number::primitive::Primitives;

pub fn create(size: u32, datatype: Primitives) -> Box<dyn TupleArray> {
    match size {
        1 => match datatype {
            Primitives::FLOAT32 => return Box::new(TupleArray1f32::new(size)),
            _ => panic!("Unsupported"),
        },
        _ => panic!("Unsupported"),
    }

}