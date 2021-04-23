//
// Public Domain - unlicense.science
//
use crate::api::{DataType, Scalar1f32, Tuple, Vector2u32};

pub fn create(size: u32, datatype: DataType) -> Box<dyn Tuple> {
    match size {
        1 => match datatype {
            DataType::F32 => return Box::new(Scalar1f32::new_empty()),
            _ => panic!("Unsupported"),
        },
        2 => match datatype {
            DataType::U32 => return Box::new(Vector2u32::new_empty()),
            _ => panic!("Unsupported"),
        },
        _ => panic!("Unsupported"),
    }
}
