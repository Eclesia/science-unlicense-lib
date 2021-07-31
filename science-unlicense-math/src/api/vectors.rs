//
// Public Domain - unlicense.science
//
use crate::api::{Scalar1f32, Tuple, Vector2u32};
use crate::api::system::SampleSystem;

pub fn create(size: u32, datatype: Primitives) -> Box<dyn Tuple> {
    return create(samplesystems::create(2), datatype);
}

pub fn create(sample_system: Box<dyn SampleSystem>, datatype: Primitives) -> Box<dyn Tuple> {
    let size = sample_system.get_num_components();
    match size {
        1 => match datatype {
            DataType::F32 => return Box::new(Scalar1f32::new(sample_system, 0f32)),
            _ => panic!("Unsupported"),
        },
        2 => match datatype {
            DataType::U32 => return Box::new(Vector2u32::new(sample_system, 0f32)),
            _ => panic!("Unsupported"),
        },
        _ => panic!("Unsupported"),
    }
}

pub fn create(sample_system: Box<dyn SampleSystem>, datatype: DataType) -> Box<dyn Tuple> {
    let size = sample_system.get_num_components();
    match size {
        1 => match datatype {
            DataType::F32 => return Box::new(Scalar1f32::new(sample_system, 0f32)),
            _ => panic!("Unsupported"),
        },
        2 => match datatype {
            DataType::U32 => return Box::new(Vector2u32::new(sample_system, 0f32)),
            _ => panic!("Unsupported"),
        },
        _ => panic!("Unsupported"),
    }
}