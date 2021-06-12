// Public Domain - unlicense.science

pub mod api;

///
/// Crate initialisation
///
pub fn init() {
    science_unlicense_geometry::init();
}

#[cfg(test)]
mod tests;
