//
// Public Domain - unlicense.science
//
pub static MESSAGE_INVALID_COORD: &'static str = "Invalid coordinate {}";

pub mod system {
    mod samplesystem; pub use samplesystem::SampleSystem;
    mod undefinedsystem; pub use undefinedsystem::UndefinedSystem;
    pub mod samplesystems;
}

mod scalar1f32; pub use scalar1f32::Scalar1f32;
mod tuple; pub use tuple::Tuple;
mod vector; pub use vector::Vector;
mod vector2u32; pub use vector2u32::Vector2u32;
pub mod vectors;







