//
// Public Domain - unlicense.science
//
use science_unlicense_math::api::system::SampleSystem;
use std::sync::Arc;

use science_unlicense_math::api::Tuple;

pub trait TupleSpace {

    ///
    /// Get description of tuple samples.
    ///
    fn get_sample_system(&self) -> &Arc<dyn SampleSystem>;

    fn get(&self, coordinate: Box<dyn Tuple>) -> Box<dyn Tuple>;

}
