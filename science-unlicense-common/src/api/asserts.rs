
///
/// Empty text.
///
pub const NO_MESSAGE : &str = "";

///
/// Compare 32bits integer values.
///
pub fn assert_equals_i32(result : i32, expected : i32, message : &str) {
    if result != expected {
        panic!("Expected {} but was {}. {}", expected, result, message);
    }
}

///
/// Compare single precision values with a tolerance.
///
pub fn assert_equals_f32(result : f32, expected : f32, tolerance: f32, message : &str) {
    assert_true(tolerance >= 0.0f32, "Tolerance must be positive");
    let diff = (result - expected).abs();
    if diff > tolerance {
        panic!("Expected {} but was {}, tolerance of {}. {}", expected, result, tolerance, message);
    }
}

///
/// Compare double precision values with a tolerance.
///
pub fn assert_equals_f64(result : f64, expected : f64, tolerance: f64, message : &str) {
    assert_true(tolerance >= 0.0, "Tolerance must be positive");
    let diff = (result - expected).abs();
    if diff > tolerance {
        panic!("Expected {} but was {}, tolerance of {}. {}", expected, result, tolerance, message);
    }
}

pub fn assert_true(value : bool, message : &str) {
    if !value {
        panic!("Expected true but was {}. {}", value, message);
    }
}

pub fn assert_false(value : bool, message : &str) {
    if value {
        panic!("Expected false but was {}. {}", value, message);
    }
}