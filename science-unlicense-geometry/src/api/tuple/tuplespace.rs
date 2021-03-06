//
// Public Domain - unlicense.science
//
use science_unlicense_math::system::SampleSystem;
use std::sync::Arc;

use science_unlicense_math::tuple::Tuple;

pub trait TupleSpace {

    ///
    /// Get description of tuple samples.
    ///
    fn get_sample_system(&self) -> &Arc<dyn SampleSystem>;

    fn get(&self, coordinate: &dyn Tuple) -> Box<dyn Tuple>;

}
