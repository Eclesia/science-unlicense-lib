//
// Public Domain - unlicense.science
//
mod arithmetic;
mod arithmetictype;
pub mod primitive;
mod float32;
mod float64;


pub use arithmetic::Arithmetic;
pub use arithmetic::Operand;
pub use arithmetictype::ArithmeticType;
pub use float32::Float32;
pub use float64::Float64;


pub const TYPE_FLOAT32 : Float32 = Float32{};
pub const TYPE_FLOAT64 : Float64 = Float64{};