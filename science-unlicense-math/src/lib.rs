// Public Domain - unlicense.science

///
/// Systems are definitions of structures of numbers.
/// For example coordinate systems or 3d modeling normals.
/// 
pub mod system {
    mod samplesystem; pub use samplesystem::SampleSystem;
    mod undefinedsystem; pub use undefinedsystem::UndefinedSystem;
    pub mod samplesystems;
}

///
/// Tuples are structures of numbers with a defined sample system.
/// Vectors are a sub-type of tuples with mathematical capabilities.
/// 
pub mod tuple {
    pub static MESSAGE_INVALID_COORD: &'static str = "Invalid coordinate {}";

    mod scalar1f32; pub use scalar1f32::Scalar1f32;
    mod tuple; pub use tuple::Tuple;
    mod vector; pub use vector::Vector;
    mod vector2u32; pub use vector2u32::Vector2u32;
    pub mod vectors;
}

///
/// Crate initialisation
///
pub fn init() {
    science_unlicense_common::init();
}

#[cfg(test)]
mod tests {
    mod abstracttupletest;
    mod scalar1f32test;
}
