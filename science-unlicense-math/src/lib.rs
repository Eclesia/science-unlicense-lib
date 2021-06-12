// Public Domain - unlicense.science

pub mod api;

///
/// Crate initialisation
///
pub fn init() {
    science_unlicense_common::init();
}

#[cfg(test)]
mod tests;
