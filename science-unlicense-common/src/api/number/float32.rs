//
// Public Domain - unlicense.science
//
use crate::api::number::Arithmetic;
use crate::api::number::ArithmeticType;
use crate::api::number::primitive::Primitives;
use std::any::TypeId;

pub struct Float32 {

}

impl ArithmeticType for Float32 {

    fn create(&self, _value: Box<dyn Arithmetic>) -> Box<dyn Arithmetic> {
        panic!("todo");
    }

    fn get_value_type(&self) -> TypeId {
        panic!("todo");
    }

    fn get_primitive_code(&self) -> Primitives {
        return Primitives::FLOAT32;
    }
}