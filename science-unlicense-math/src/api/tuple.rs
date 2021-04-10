// Public Domain - unlicense.science

pub trait Tuple {
    /// Size of the Tuple
    fn get_dimension(&self) -> u32;

    /// index must be between 0 and tuple dimension
    fn get(&self, index: u32) -> f64;

    /// index must be between 0 and tuple dimension
    fn set(&mut self, index: u32, value: f64);
}
