// Public Domain - unlicense.science

use crate::api::{Scalar1f32, Tuple};
use crate::tests::abstracttupletest::AbstractTupleTest;

struct Scalar1f32Test {

}

impl AbstractTupleTest for Scalar1f32Test {
    fn get_supported_dimensions(&self) -> Vec<u32> {
        return vec![1];
    }

    fn create(&self, dim: u32) -> Box<dyn Tuple> {
        match dim {
            1 => return Box::new(Scalar1f32::new_empty()),
            _ => panic!("Unexpected")
        }
    }
}

#[test]
pub fn test() {
    Scalar1f32Test{}.test_tuple();
}
