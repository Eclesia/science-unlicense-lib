//
// Public Domain - unlicense.science
//

use crate::system::UndefinedSystem;
use crate::system::SampleSystem;
use std::sync::Arc;

///
/// Create a sample system.
/// The 10 first systems are in cache.
///
/// @param nbComponents
/// @return SampleSystem never null
///
pub fn create(nb_components: u32) -> Arc<dyn SampleSystem> {
    //let x : SampleSystem = UndefinedSystem::new(nb_components);
    return Arc::new(UndefinedSystem::new(nb_components));
}