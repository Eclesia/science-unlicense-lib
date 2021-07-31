//
// Public Domain - unlicense.science
//
use crate::api::system::SampleSystem;
use crate::api::Vectors;
use science_unlicense_common::api::number::ArithmeticType;
use science_unlicense_common::api::number::Arithmetic;

///
/// A tuple is an ordered list of numeric values.
///
pub trait Tuple {

    ///
    /// Get description of tuple samples.
    ///
    fn get_sample_system(&self) -> Box<dyn SampleSystem>;

    ///
    /// Natural numeric type stored in this tuple.
    ///
    fn get_numeric_type(&self) -> Box<dyn ArithmeticType>;

    ///
    /// Size of the Tuple
    ///
    fn get_sample_count(&self) -> u32;

    ///
    /// Get one sample at index.
    /// Index must be between 0 and tuple dimension, if not application will panic.
    ///
    fn get(&self, index: u32) -> f64;

    ///
    /// Index must be between 0 and tuple dimension, if not application will panic.
    ///
    fn get_number(&self, index: u32) -> f64;

    ///
    /// Copy a range of samples to a new tuple.
    ///
    /// * `start` - inclusive
    /// * `end` - exclusive
    /// 
    /// Indexes must be between 0 and tuple dimension, if not application will panic.
    ///
    fn get_range(&self, start: u32, end: u32) -> Box<dyn Tuple> {
        Vectors::create(self.get_sample_system(), self.get_data);
    }

    ///
    /// True if all values are set to given value.
    ///
    fn is_all(&self, value: f64) -> bool;

    ///
    /// Check that all values are not NaN or Infinites.
    ///
    fn is_valid(&self) -> bool;

    ///
    /// Get a copy of the tuple values as booleans..
    ///
    /// @return boolean array
    ///
    fn to_bool(&self) -> Vec<bool>;

    ///
    /// Get a copy of the tuple values as booleans..
    ///
    /// @param buffer buffer to write into, not null
    /// @param offset offset in the buffer where to start writing
    ///
    fn to_bool_vec(&self, buffer: Vec<bool>, offset: u32);

    ///
    /// Get a copy of the tuple values as bytes..
    ///
    /// @return byte array
    ///
    fn to_i8(&self) -> Vec<i8>;

    ///
    /// Get a copy of the tuple values as bytes..
    ///
    /// @param buffer buffer to write into, not null
    /// @param offset offset in the buffer where to start writing
    ///
    fn to_i8_vec(&self, buffer: Vec<i8>, offset: u32);

    ///
    /// Get a copy of the tuple values as shorts.
    ///
    /// @return short array
    ///
    fn to_i16(&self) -> Vec<i16>;

    ///
    /// Get a copy of the tuple values as shorts.
    ///
    /// @param buffer buffer to write into, not null
    /// @param offset offset in the buffer where to start writing
    ///
    fn to_i16_vec(&self, buffer: Vec<i16>, offset: u32);

    ///
    /// Get a copy of the tuple values as ints.
    ///
    /// @return int array
    ///
    fn to_i32(&self) -> Vec<i32>;

    ///
    /// Get a copy of the tuple values as ints.
    ///
    /// @param buffer buffer to write into, not null
    /// @param offset offset in the buffer where to start writing
    ///
    fn to_i32_vec(&self, buffer: Vec<i32>, offset: u32);

    ///
    /// Get a copy of the tuple values as longs.
    ///
    /// @return long array
    ///
    fn to_i64(&self) -> Vec<i64>;

    ///
    /// Get a copy of the tuple values as longs.
    ///
    /// @param buffer buffer to write into, not null
    /// @param offset offset in the buffer where to start writing
    ///
    fn to_i64_vec(&self, buffer: Vec<i64>, offset: u32);

    ///
    /// Get a copy of the tuple values as floats.
    ///
    /// @return float array
    ///
    fn to_f32(&self) -> Vec<f32>;

    ///
    /// Returns a copy of the array values casted to float.
    ///
    /// @param buffer buffer to write into, not null
    /// @param offset offset in the buffer where to start writing
    ///
    fn to_f32_vec(&self, buffer: Vec<f32>, offset: u32);

    ///
    /// Get a copy of the tuple values.
    ///
    /// @return double array
    ///
    fn to_f64(&self) -> Vec<f64>;

    ///
    /// Copies and returns the array values.
    ///
    /// @param buffer buffer to write into, not null
    /// @param offset offset in the buffer where to start writing
    ///
    fn to_f64_vec(&self, buffer: Vec<f64>, offset: u32);

    ///
    /// Get a copy of the tuple values.
    ///
    /// @return number array
    ///
    fn to_number(&self) -> Vec<Box<dyn Arithmetic>>;

    ///
    /// Copies and returns the array values.
    ///
    /// @param buffer buffer to write into, not null
    /// @param offset offset in the buffer where to start writing
    ///
    fn to_number_vec(&self, buffer: Vec<Box<dyn ArithmeticType>>, offset: u32);

    ///
    /// Copy this Tuple.
    ///
    /// @return new tuple copy
    ///
    fn copy(&self) -> Box<dyn Tuple>;

    ///
    /// Test equality with a tolerance margin.
    ///
    /// @param obj
    /// @param tolerance
    /// @return true if tuple are equal
    ///
    fn equals(&self, obj: Box<dyn Tuple>, tolerance: f64) -> bool;

    ///
    /// Create a new Tuple of a different size but similar storage.
    /// The returned tuple must have at least the same NumericType.
    ///
    /// @param size
    /// @return
    ///
    fn create(&self, size: u32) -> Box<dyn Tuple>;

    ///
    /// Set value at given ordinate.
    /// 
    /// Index must be between 0 and tuple dimension, if not application will panic.
    ///
    fn set(&mut self, index: u32, value: f64);

    ///
    /// Set value at given ordinate.
    /// 
    /// Index must be between 0 and tuple dimension, if not application will panic.
    /// 
    fn set_number(&mut self, index: u32, value: Box<dyn Arithmetic>);

    ///
    ///Set all values to given value.
    ///
    fn set_all(&mut self, value :f64);

    /**
     * Copy values from tuple.
     * If toCopy.values.length superior to this.values.length, then the only
     * this.values.length values of toCopy are copied.
     *
     * @param toCopy tuple to copy
     */
    fn set_from_tuple(&mut self, toCopy: Box<dyn Tuple>);

    ///
    /// Copy values from array.
    /// If values.length superior to this.values.length, then the only
    /// this.values.length values of values are copied.
    ///
    fn set_from_f64(&mut self, values: Vec<f64>);

    ///
    /// Copy values from array.
    /// If values.length superior to this.values.length, then the only
    /// this.values.length values of values are copied.
    ///
    fn set_from_f32(&mut self, values: Vec<f32>);

}
