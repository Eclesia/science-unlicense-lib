// Public Domain - unlicense.science

use crate::api::{Scalar1f32, Tuple};

#[test]
pub fn test() {
    let v = Scalar1f32 { x: 2f32 };
    let x = v.get(0);
    let y = v.get(1);
}
