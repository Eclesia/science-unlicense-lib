
pub const 

pub fn assert_equals_f64(result : f64, expected : f64, tolerance: f64) {
    assert_true(tolerance >= 0, "Tolerance must be positive");
    let diff = (result - expected).abs();
    if diff > tolerance {
        panic!("Expected {} but was {}. tolerance of {}.", expected, result, tolerance);
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