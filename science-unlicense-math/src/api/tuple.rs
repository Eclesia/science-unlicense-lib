//
// Public Domain - unlicense.science
//
use crate::api::system::SampleSystem;
use crate::api::vectors;
use science_unlicense_common::api::number::ArithmeticType;
use science_unlicense_common::api::number::Arithmetic;
use std::sync::Arc;

///
/// A tuple is an ordered list of numeric values.
///
pub trait Tuple {

    ///
    /// Get description of tuple samples.
    ///
    fn get_sample_system(&self) -> &Arc<dyn SampleSystem>;

    ///
    /// Natural numeric type stored in this tuple.
    ///
    fn get_numeric_type(&self) -> &Arc<dyn ArithmeticType>;

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
    fn get_number(&self, index: u32) -> Box<dyn Arithmetic>;

    ///
    /// Copy a range of samples to a new tuple.
    ///
    /// * `start` - inclusive
    /// * `end` - exclusive
    /// 
    /// Indexes must be between 0 and tuple dimension, if not application will panic.
    ///
    fn get_range(&self, start: u32, end: u32) -> Box<dyn Tuple> {
        let mut tuple = vectors::create_by_size(end - start, self.get_numeric_type().get_primitive_code());
        let mut k = 0;
        for i in start..end {
            tuple.set(k, self.get(i));
            k += 1;
        }
        return tuple;
    }

    ///
    /// True if all values are set to given value.
    ///
    fn is_all(&self, value: f64) -> bool {
        for i in 0..self.get_sample_count() {
            if self.get(i) != value {
                return false;
            }
        }
        return true;
    }

    ///
    /// Check that all values are not NaN or Infinite.
    ///
    fn is_valid(&self) -> bool {
        for i in 0..self.get_sample_count() {
            if !self.get(i).is_finite() {
                return false;
            }
        }
        return true;
    }

    ///
    /// Get a copy of the tuple values as booleans..
    ///
    /// @return boolean array
    ///
    fn to_bool(&self) -> Vec<bool> {
        let size = self.get_sample_count();
        let mut array : Vec<bool> = Vec::with_capacity(size as usize);
        for i in 0..size {
            array.push(self.get(i) != 0f64);
        }
        return array;
    }

    ///
    /// Get a copy of the tuple values as booleans..
    ///
    /// @param buffer buffer to write into, not null
    /// @param offset offset in the buffer where to start writing
    ///
    fn to_bool_vec(&self, mut buffer: Vec<bool>, offset: u32) {
        let size = self.get_sample_count();
        for i in 0..size {
            buffer[(offset + i) as usize] = self.get(i) != 0f64;
        }
    }

    ///
    /// Get a copy of the tuple values as bytes..
    ///
    /// @return byte array
    ///
    fn to_i8(&self) -> Vec<i8> {
        let size = self.get_sample_count();
        let mut array : Vec<i8> = Vec::with_capacity(size as usize);
        for i in 0..size {
            array.push(self.get(i) as i8);
        }
        return array;
    }

    ///
    /// Get a copy of the tuple values as bytes..
    ///
    /// @param buffer buffer to write into, not null
    /// @param offset offset in the buffer where to start writing
    ///
    fn to_i8_vec(&self, mut buffer: Vec<i8>, offset: u32) {
        let size = self.get_sample_count();
        for i in 0..size {
            buffer[(offset + i) as usize] = self.get(i) as i8;
        }
    }

    ///
    /// Get a copy of the tuple values as shorts.
    ///
    /// @return short array
    ///
    fn to_i16(&self) -> Vec<i16> {
        let size = self.get_sample_count();
        let mut array : Vec<i16> = Vec::with_capacity(size as usize);
        for i in 0..size {
            array.push(self.get(i) as i16);
        }
        return array;
    }

    ///
    /// Get a copy of the tuple values as shorts.
    ///
    /// @param buffer buffer to write into, not null
    /// @param offset offset in the buffer where to start writing
    ///
    fn to_i16_vec(&self, mut buffer: Vec<i16>, offset: u32) {
        let size = self.get_sample_count();
        for i in 0..size {
            buffer[(offset + i) as usize] = self.get(i) as i16;
        }
    }

    ///
    /// Get a copy of the tuple values as ints.
    ///
    /// @return int array
    ///
    fn to_i32(&self) -> Vec<i32> {
        let size = self.get_sample_count();
        let mut array : Vec<i32> = Vec::with_capacity(size as usize);
        for i in 0..size {
            array.push(self.get(i) as i32);
        }
        return array;
    }

    ///
    /// Get a copy of the tuple values as ints.
    ///
    /// @param buffer buffer to write into, not null
    /// @param offset offset in the buffer where to start writing
    ///
    fn to_i32_vec(&self, mut buffer: Vec<i32>, offset: u32) {
        let size = self.get_sample_count();
        for i in 0..size {
            buffer[(offset + i) as usize] = self.get(i) as i32;
        }
    }

    ///
    /// Get a copy of the tuple values as longs.
    ///
    /// @return long array
    ///
    fn to_i64(&self) -> Vec<i64> {
        let size = self.get_sample_count();
        let mut array : Vec<i64> = Vec::with_capacity(size as usize);
        for i in 0..size {
            array.push(self.get(i) as i64);
        }
        return array;
    }

    ///
    /// Get a copy of the tuple values as longs.
    ///
    /// @param buffer buffer to write into, not null
    /// @param offset offset in the buffer where to start writing
    ///
    fn to_i64_vec(&self, mut buffer: Vec<i64>, offset: u32) {
        let size = self.get_sample_count();
        for i in 0..size {
            buffer[(offset + i) as usize] = self.get(i) as i64;
        }
    }

    ///
    /// Get a copy of the tuple values as floats.
    ///
    /// @return float array
    ///
    fn to_f32(&self) -> Vec<f32> {
        let size = self.get_sample_count();
        let mut array : Vec<f32> = Vec::with_capacity(size as usize);
        for i in 0..size {
            array.push(self.get(i) as f32);
        }
        return array;
    }

    ///
    /// Returns a copy of the array values casted to float.
    ///
    /// @param buffer buffer to write into, not null
    /// @param offset offset in the buffer where to start writing
    ///
    fn to_f32_vec(&self, mut buffer: Vec<f32>, offset: u32) {
        let size = self.get_sample_count();
        for i in 0..size {
            buffer[(offset + i) as usize] = self.get(i) as f32;
        }
    }

    ///
    /// Get a copy of the tuple values.
    ///
    /// @return double array
    ///
    fn to_f64(&self) -> Vec<f64> {
        let size = self.get_sample_count();
        let mut array : Vec<f64> = Vec::with_capacity(size as usize);
        for i in 0..size {
            array.push(self.get(i));
        }
        return array;
    }

    ///
    /// Copies and returns the array values.
    ///
    /// @param buffer buffer to write into, not null
    /// @param offset offset in the buffer where to start writing
    ///
    fn to_f64_vec(&self, mut buffer: Vec<f64>, offset: u32) {
        let size = self.get_sample_count();
        for i in 0..size {
            buffer[(offset + i) as usize] = self.get(i);
        }
    }

    ///
    /// Get a copy of the tuple values.
    ///
    /// @return number array
    ///
    fn to_number(&self) -> Vec<Box<dyn Arithmetic>> {
        let size = self.get_sample_count();
        let mut array : Vec<Box<dyn Arithmetic>> = Vec::with_capacity(size as usize);
        for i in 0..size {
            array.push(self.get_number(i));
        }
        return array;
    }

    ///
    /// Copies and returns the array values.
    ///
    /// @param buffer buffer to write into, not null
    /// @param offset offset in the buffer where to start writing
    ///
    fn to_number_vec(&self, mut buffer: Vec<Box<dyn Arithmetic>>, offset: u32) {
        let size = self.get_sample_count();
        for i in 0..size {
            buffer[(offset + i) as usize] = self.get_number(i);
        }
    }

    ///
    /// Copy this Tuple.
    ///
    /// @return new tuple copy
    ///
    fn copy(&self) -> Box<dyn Tuple> {
        let mut copy = vectors::create(self.get_sample_system(), self.get_numeric_type());
        let size = self.get_sample_count();
        for i in 0..size {
            copy.set(i, self.get(i));
        }
        return copy;
    }

    ///
    /// Test equality with a tolerance margin.
    ///
    /// @param obj
    /// @param tolerance
    /// @return true if tuple are equal
    ///
    fn equals(&self, obj: &Box<dyn Tuple>, tolerance: f64) -> bool {
        let size = self.get_sample_count();
        let size2 = obj.get_sample_count();
        if size != size2 {
            return false;
        }
        for i in 0..size {
            let x = self.get(i);
            let y = obj.get(i);
            let diff = (x - y).abs();
            if diff > tolerance {
                return false;
            }
        }
        return true;
    }

    ///
    /// Create a new Tuple of a different size but similar storage.
    /// The returned tuple must have at least the same NumericType.
    ///
    /// @param size
    /// @return
    ///
    fn create(&self, size: u32) -> Box<dyn Tuple> {
        return vectors::create_by_size(size, self.get_numeric_type().get_primitive_code());
    }

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
    fn set_all(&mut self, value :f64) {
        let size = self.get_sample_count();
        for i in 0..size {
            self.set(i, value);
        }
    }

    /**
     * Copy values from tuple.
     * If toCopy.values.length superior to this.values.length, then the only
     * this.values.length values of toCopy are copied.
     *
     * @param toCopy tuple to copy
     */
    fn set_from_tuple(&mut self, to_copy: &Box<dyn Tuple>) {
        let size = self.get_sample_count();
        for i in 0..size {
            self.set(i, to_copy.get(i));
        }
    }

    ///
    /// Copy values from array.
    /// If values.length superior to this.values.length, then the only
    /// this.values.length values of values are copied.
    ///
    fn set_from_f64(&mut self, values: Vec<f64>) {
        let size = self.get_sample_count();
        for i in 0..size {
            self.set(i, values[i as usize]);
        }
    }

    ///
    /// Copy values from array.
    /// If values.length superior to this.values.length, then the only
    /// this.values.length values of values are copied.
    ///
    fn set_from_f32(&mut self, values: Vec<f32>) {
        let size = self.get_sample_count();
        for i in 0..size {
            self.set(i, values[i as usize] as f64);
        }
    }

}
