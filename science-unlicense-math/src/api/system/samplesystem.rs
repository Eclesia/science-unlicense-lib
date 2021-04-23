//
// Public Domain - unlicense.science
//

pub trait SampleSystem {

    ///
    /// Get number of samples defining this system.
    ///
    /// return number of sample values in the system.
    ///
    fn get_num_components(&self) -> u32;

}