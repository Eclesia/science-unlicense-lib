//
// Public Domain - unlicense.science
//
use crate::api::system::SampleSystem;

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
    fn get_numeric_type(&self) -> ArithmeticType;

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
    fn get_range(&self, start: u32, end: u32) -> Box<dyn Tuple>;

    ///
    /// True if all values are set to given value.
    ///
    fn is_all(&self, value: f64) -> boolean;

    ///
    /// Check that all values are not NaN or Infinites.
    ///
    fn is_valid(&self) -> boolean;

    /**
     * Get a copy of the tuple values as booleans..
     *
     * @return boolean array
     */
    boolean[] toBoolean(&self);

    /**
     * Get a copy of the tuple values as booleans..
     *
     * @param buffer buffer to write into, not null
     * @param offset offset in the buffer where to start writing
     */
    void toBoolean(&self, boolean[] buffer, int offset);

    /**
     * Get a copy of the tuple values as bytes..
     *
     * @return byte array
     */
    byte[] toByte(&self);

    /**
     * Get a copy of the tuple values as bytes..
     *
     * @param buffer buffer to write into, not null
     * @param offset offset in the buffer where to start writing
     */
    void toByte(&self, byte[] buffer, int offset);

    /**
     * Get a copy of the tuple values as shorts.
     *
     * @return short array
     */
    short[] toShort(&self);

    /**
     * Get a copy of the tuple values as shorts.
     *
     * @param buffer buffer to write into, not null
     * @param offset offset in the buffer where to start writing
     */
    void toShort(&self, short[] buffer, int offset);

    /**
     * Get a copy of the tuple values as ints.
     *
     * @return int array
     */
    int[] toInt(&self);

    /**
     * Get a copy of the tuple values as ints.
     *
     * @param buffer buffer to write into, not null
     * @param offset offset in the buffer where to start writing
     */
    void toInt(&self, int[] buffer, int offset);

    /**
     * Get a copy of the tuple values as longs.
     *
     * @return long array
     */
    long[] toLong(&self);

    /**
     * Get a copy of the tuple values as longs.
     *
     * @param buffer buffer to write into, not null
     * @param offset offset in the buffer where to start writing
     */
    void toLong(&self, long[] buffer, int offset);

    /**
     * Get a copy of the tuple values as floats.
     *
     * @return float array
     */
    float[] toFloat(&self);

    /**
     * Returns a copy of the array values casted to float.
     *
     * @param buffer buffer to write into, not null
     * @param offset offset in the buffer where to start writing
     */
    void toFloat(&self, float[] buffer, int offset);

    /**
     * Get a copy of the tuple values.
     *
     * @return double array
     */
    double[] toDouble(&self);

    /**
     * Copies and returns the array values.
     *
     * @param buffer buffer to write into, not null
     * @param offset offset in the buffer where to start writing
     */
    void toDouble(&self, double[] buffer, int offset);

    /**
     * Get a copy of the tuple values.
     *
     * @return number array
     */
    Arithmetic[] toNumber(&self);

    /**
     * Copies and returns the array values.
     *
     * @param buffer buffer to write into, not null
     * @param offset offset in the buffer where to start writing
     */
    void toNumber(&self, Arithmetic[] buffer, int offset);

    /**
     * Copy this Tuple.
     *
     * @return new tuple copy
     */
    Tuple copy(&self);

    /**
     * Test equality with a tolerance margin.
     *
     * @param obj
     * @param tolerance
     * @return true if tuple are equal
     */
    boolean equals(&self, Tuple obj, double tolerance);

    /**
     * Create a new Tuple of a different size but similar storage.
     * The returned tuple must have at least the same NumericType.
     *
     * @param size
     * @return
     */
    TupleRW create(&self, int size);

    ///
    /// Set value at given ordinate.
    /// 
    /// Index must be between 0 and tuple dimension, if not application will panic.
    ///
    fn set(&mut self, index: u32, value: f64);


    ///
    ///Set all values to given value.
    ///
    fn set_all(&mut self, value :f64);

    ///
    /// Set value at given ordinate.
    /// 
    /// Index must be between 0 and tuple dimension, if not application will panic.
    /// 
    fn set(&mut self, index: u32, value: Box<dyn Arithmetic>);

    /**
     * Copy values from tuple.
     * If toCopy.values.length superior to this.values.length, then the only
     * this.values.length values of toCopy are copied.
     *
     * @param toCopy tuple to copy
     */
    void set(&mut self, Tuple toCopy);

    ///
    /// Copy values from array.
    /// If values.length superior to this.values.length, then the only
    /// this.values.length values of values are copied.
    ///
    fn set(&mut self, values: Vec<f64>);

    ///
    /// Copy values from array.
    /// If values.length superior to this.values.length, then the only
    /// this.values.length values of values are copied.
    ///
    fn set(&mut self, values: Vec<f32>);

    ///
    /// Copy this Tuple.
    /// 
    fn copy() -> Box<dyn Tuple>;

}
