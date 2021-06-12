// Public Domain - unlicense.science

pub mod api;

///
/// Crate initialisation
///
pub fn init() {
    //science_unlicense_common::init(); Done by Math module already
    science_unlicense_math::init();
}

#[cfg(test)]
mod tests;
