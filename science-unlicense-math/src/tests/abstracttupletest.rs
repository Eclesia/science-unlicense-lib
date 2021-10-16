use crate::api::Tuple;

use std::sync::Mutex;
use std::sync::Arc;
use science_unlicense_common::api::number::TYPE_FLOAT32;
use science_unlicense_common::api::number::TYPE_FLOAT64;
use science_unlicense_common::api::asserts::assert_true;
use science_unlicense_common::api::asserts::assert_false;
use science_unlicense_common::api::asserts::assert_error;
use science_unlicense_common::api::asserts::assert_equals_i32;
use science_unlicense_common::api::asserts::assert_equals_f64;
use science_unlicense_common::api::asserts::NO_MESSAGE;

const TOLERANCE : f64 = 0.0000001;
const UNVALID_INDEX_EXPECTED : &str = "Accessing value our of tuple size must cause an InvalidIndexException";

pub trait AbstractTupleTest {

    ///
    /// List dimensions tested.
    ///
    fn get_supported_dimensions(&self) -> Vec<u32>;

    ///
    /// Created tuple must have all values at zero.
    ///
    fn create(&self, dim : u32) -> Box<dyn Tuple>;

    fn test_tuple(&self) {

        let supported_dimensions = self.get_supported_dimensions();

        for dim in supported_dimensions {
            let mut tuple = self.create(dim);
            assert_equals_i32(tuple.get_sample_count() as i32, dim as i32, NO_MESSAGE);
            self.test_all_value(&tuple, 0.0);
            assert_true(tuple.is_all(0.0), NO_MESSAGE);
            self.test_cell_get_set(&mut tuple);
            self.test_equality(&mut tuple);
        }

    }

    ///
    /// Test all tuple values equal the expected value.
    ///
    fn test_all_value(&self, tuple: &Box<dyn Tuple>, expected_value: f64) {
        let dim = tuple.get_sample_count();

        for c in 0..dim {
            let value = tuple.get(c);
            let mut msg = "Value different at [".to_owned();
            msg.push_str(&c.to_string());
            msg.push_str("]");
            assert_equals_f64(value, expected_value, TOLERANCE, &msg);
        }

        assert_true(tuple.is_all(expected_value), NO_MESSAGE);
        assert_false(tuple.is_all(expected_value + 1.0), NO_MESSAGE);
    }

    ///
    ///Test tuple value getters and setters.
    ///
    fn test_cell_get_set(&self, tuple: &mut Box<dyn Tuple>) {
        let dim = tuple.get_sample_count();
        for i in 0..dim {
            tuple.set(i, i as f64 + 1 as f64);
            let mut msg = "Value different at [".to_owned();
            msg.push_str(&i.to_string());
            msg.push_str("]");
            assert_equals_f64(tuple.get(i), i as f64 + 1.0, TOLERANCE, &msg);
        }

        //test shortcuts
        //TODO

        //test out of range
        //test higher index out of range
        let m = Mutex::new(tuple);
        assert_error(|| {
            let mut t = m.lock().unwrap();
            t.set(dim, 10.0);
        }, UNVALID_INDEX_EXPECTED);
    }

    fn test_equality(&self, tuple: &mut Box<dyn Tuple>) {
        let dim = tuple.get_sample_count();

        for i in 0..dim {
            tuple.set(i, i as f64 + 1 as f64);
        }

        //test equals itself
        assert_true(tuple.equals(&tuple, 0.0), NO_MESSAGE);

        //test equals a copy
        let copy = tuple.copy();
        assert_true(tuple.equals(&copy, 0.0), NO_MESSAGE);

        //test a equals a newly created tuple
        let mut newtuple = self.create(tuple.get_sample_count());
        assert_false(newtuple.equals(tuple, 0.0), NO_MESSAGE);
        newtuple.set_from_tuple(tuple);
        assert_true(newtuple.equals(tuple, 0.0), NO_MESSAGE);

        //if tuple.get_numeric_type() == TYPE_FLOAT32
        //|| tuple.get_numeric_type() == TYPE_FLOAT64 {
            //test NaN equality
            //copy.set(0, Double.NaN);
        //    assert_false(copy.equals(tuple, 0.0), NO_MESSAGE);
            //tuple.set(0, Double.NaN);
        //    assert_true(copy.equals(tuple, 0.0), NO_MESSAGE);
        //}

    }

    //pub fn is_same(left: &Object, right: &Object) -> bool {
    //    let r = ptr::eq(left.as_ref(), right.as_ref());
    //    println!("comparing: {:p} == {:p} -> {}", left, right, r);
    //    r
    //}
}
