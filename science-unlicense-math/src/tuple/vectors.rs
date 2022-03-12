//
// Public Domain - unlicense.science
//
use crate::system::SampleSystem;
use crate::system::samplesystems;
use crate::tuple::Scalar1f32;
use crate::tuple::Tuple;
use crate::tuple::Vector2u32;
use science_unlicense_common::api::number::primitive::Primitives;
use science_unlicense_common::api::number::ArithmeticType;
use std::sync::Arc;

pub fn create_by_size<'a>(size: u32, datatype: Primitives) -> Box<dyn Tuple> {
    let s = samplesystems::create(size);
    return create_by_type(&s, datatype);
}

pub fn create_by_type(sample_system: &Arc<dyn SampleSystem>, datatype: Primitives) -> Box<dyn Tuple> {
    let size = sample_system.get_num_components();
    match size {
        1 => match datatype {
            Primitives::FLOAT32 => return Box::new(Scalar1f32::new_system(sample_system.clone())),
            _ => panic!("Unsupported 1"),
        },
        2 => match datatype {
            Primitives::UINT32 => return Box::new(Vector2u32::new_system(sample_system.clone())),
            _ => panic!("Unsupported "),
        },
        _ => panic!("Unsupported"),
    }
}

pub fn create(sample_system: &Arc<dyn SampleSystem>, datatype: &Arc<dyn ArithmeticType>) -> Box<dyn Tuple> {
    return create_by_type(sample_system, datatype.get_primitive_code());
}