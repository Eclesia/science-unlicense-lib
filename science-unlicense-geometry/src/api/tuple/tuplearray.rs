// Public Domain - unlicense.science

use science_unlicense_math::api::Tuple;

pub trait TupleArray {
    fn get(&self, coordinate: Box<dyn Tuple>) -> Box<dyn Tuple>;
}
