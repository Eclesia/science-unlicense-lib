//
// Public Domain - unlicense.science
//
use science_unlicense_math::api::{Scalar1f32, Tuple, DataType};
use science_unlicense_math::api::vectors;

pub fn run() {
    let mut x = Scalar1f32::new(5.0f32);
    x.set(0, 2.0);

    let coord = vectors::create(2, DataType::F32);
    let x = coord.get(0);
    println!("hello")
}
