//
// Public Domain - unlicense.science
//
use science_unlicense_math::api::{Scalar1f32, Tuple};
use science_unlicense_math::api::vectors;
use science_unlicense_common::api::number::primitive::Primitives;

pub fn run() {
    let mut x = Scalar1f32::new_data(5.0f32);
    x.set(0, 2.0);

    let coord = vectors::create_by_size(2, Primitives::FLOAT32);
    let x = coord.get(0);
    println!("hello")
}
