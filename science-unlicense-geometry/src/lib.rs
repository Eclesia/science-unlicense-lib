// Public Domain - unlicense.science

//! GEOMETRY CRATE
//!
//! for everything about geometries

pub mod api;

///
/// Crate initialisation
///
pub fn init() {
    science_unlicense_math::init();
}

#[cfg(test)]
mod tests;
