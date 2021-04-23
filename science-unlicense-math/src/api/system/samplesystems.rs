//
// Public Domain - unlicense.science
//

use crate::api::system::{UndefinedSystem};

///
/// Create a sample system.
/// The 10 first systems are in cache.
///
/// @param nbComponents
/// @return SampleSystem never null
///
pub fn create(nb_components: u32) -> UndefinedSystem{
    return UndefinedSystem::new(nb_components);
}