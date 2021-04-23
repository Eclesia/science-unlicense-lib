//
// Public Domain - unlicense.science
//
use science_unlicense_math::api::DataType;
use crate::api::tuple::{TupleArray1f32, TupleArray};

pub fn create(size: u32, datatype: DataType) -> Box<dyn TupleArray> {
    match size {
        1 => match datatype {
            DataType::F32 => return Box::new(TupleArray1f32::new(size)),
            _ => panic!("Unsupported"),
        },
        _ => panic!("Unsupported"),
    }

}