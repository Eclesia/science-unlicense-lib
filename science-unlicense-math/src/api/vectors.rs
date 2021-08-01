//
// Public Domain - unlicense.science
//
use crate::api::{Scalar1f32, Tuple, Vector2u32};
use crate::api::system::SampleSystem;
use crate::api::system::samplesystems;
use science_unlicense_common::api::number::primitive::Primitives;
use science_unlicense_common::api::number::ArithmeticType;

pub fn create_by_size(size: u32, datatype: Primitives) -> Box<dyn Tuple> {
    return create_by_type(Box::new(samplesystems::create(size)), datatype);
}

pub fn create_by_type(sample_system: Box<dyn SampleSystem>, datatype: Primitives) -> Box<dyn Tuple> {
    let size = sample_system.get_num_components();
    match size {
        1 => match datatype {
            Primitives::FLOAT32 => return Box::new(Scalar1f32::new_system(sample_system)),
            _ => panic!("Unsupported"),
        },
        2 => match datatype {
            Primitives::UINT32 => return Box::new(Vector2u32::new_system(sample_system)),
            _ => panic!("Unsupported"),
        },
        _ => panic!("Unsupported"),
    }
}

pub fn create(sample_system: Box<dyn SampleSystem>, datatype: Box<dyn ArithmeticType>) -> Box<dyn Tuple> {
    return create_by_type(sample_system, datatype.get_primitive_code());
}