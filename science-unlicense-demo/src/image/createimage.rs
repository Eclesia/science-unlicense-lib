//
// Public Domain - unlicense.science
//
use science_unlicense_math::tuple::{Scalar1f32, Tuple};
use science_unlicense_math::tuple::vectors;
use science_unlicense_common::api::number::primitive::Primitives;

pub fn main() {
    let mut x = Scalar1f32::new_data(5.0f32);
    x.set(0, 2.0);

    let coord = vectors::create_by_size(2, Primitives::FLOAT32);
    let _x = coord.get(0);
    println!("hello")
}
