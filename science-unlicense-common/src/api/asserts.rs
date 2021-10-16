use std::panic;
use std::panic::UnwindSafe;

///
/// Empty text.
///
pub const NO_MESSAGE : &str = "";

///
/// Compare 8bits integer values.
///
pub fn assert_equals_b(result : bool, expected : bool, message : &str) {
    if result != expected {
        panic!("Expected {} but was {}. {}", expected, result, message);
    }
}

///
/// Compare 8bits integer values.
///
pub fn assert_equals_i8(result : i8, expected : i8, message : &str) {
    if result != expected {
        panic!("Expected {} but was {}. {}", expected, result, message);
    }
}

///
/// Compare 8bits unsigned integer values.
///
pub fn assert_equals_u8(result : u8, expected : u8, message : &str) {
    if result != expected {
        panic!("Expected {} but was {}. {}", expected, result, message);
    }
}

///
/// Compare 16bits integer values.
///
pub fn assert_equals_i16(result : i16, expected : i16, message : &str) {
    if result != expected {
        panic!("Expected {} but was {}. {}", expected, result, message);
    }
}

///
/// Compare 16bits unsigned integer values.
///
pub fn assert_equals_u16(result : u16, expected : u16, message : &str) {
    if result != expected {
        panic!("Expected {} but was {}. {}", expected, result, message);
    }
}

///
/// Compare 32bits integer values.
///
pub fn assert_equals_i32(result : i32, expected : i32, message : &str) {
    if result != expected {
        panic!("Expected {} but was {}. {}", expected, result, message);
    }
}

///
/// Compare 32bits unsigned integer values.
///
pub fn assert_equals_u32(result : u32, expected : u32, message : &str) {
    if result != expected {
        panic!("Expected {} but was {}. {}", expected, result, message);
    }
}

///
/// Compare 64bits integer values.
///
pub fn assert_equals_i64(result : i64, expected : i64, message : &str) {
    if result != expected {
        panic!("Expected {} but was {}. {}", expected, result, message);
    }
}

///
/// Compare 64bits unsigned integer values.
///
pub fn assert_equals_u64(result : u64, expected : u64, message : &str) {
    if result != expected {
        panic!("Expected {} but was {}. {}", expected, result, message);
    }
}

///
/// Compare 128bits integer values.
///
pub fn assert_equals_i128(result : i128, expected : i128, message : &str) {
    if result != expected {
        panic!("Expected {} but was {}. {}", expected, result, message);
    }
}

///
/// Compare 128bits unsigned integer values.
///
pub fn assert_equals_u128(result : u128, expected : u128, message : &str) {
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

///
/// Assert value is true.
///
pub fn assert_true(value : bool, message : &str) {
    if !value {
        panic!("Expected true but was {}. {}", value, message);
    }
}

///
/// Assert value is false.
///
pub fn assert_false(value : bool, message : &str) {
    if value {
        panic!("Expected false but was {}. {}", value, message);
    }
}

///
/// Assert given lambda will panic.
/// This functions ensure the stack is not printed.
/// 
pub fn assert_error<F: FnOnce() -> R + UnwindSafe, R>(f: F, message : &str) {
    // do not print the stack, we expect it.
    let prev_hook = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));

    let res = panic::catch_unwind(f);
    match res {
        Ok(_v) => panic!("Expected an error {}", message),
        Err(_e) => (),
    }
    panic::set_hook(prev_hook);
}