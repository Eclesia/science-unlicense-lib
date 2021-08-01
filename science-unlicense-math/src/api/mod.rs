//
// Public Domain - unlicense.science
//
mod scalar1f32;
mod tuple;
mod vector;
mod vector2u32;
pub mod system;
pub mod vectors;

pub use scalar1f32::Scalar1f32;
pub use tuple::Tuple;
pub use vector::Vector;
pub use vector2u32::Vector2u32;

pub static MESSAGE_INVALID_COORD: &'static str = "Invalid coordinate {}";
