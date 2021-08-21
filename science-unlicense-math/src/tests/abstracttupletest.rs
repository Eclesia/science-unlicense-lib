use crate::api::Tuple;
use science_unlicense_common::api::asserts;
use std::panic::catch_unwind;
use science_unlicense_common::api::asserts::{NO_MESSAGE, assert_equals_f64};

const TOLERANCE : f64 = 0.0000001;
const UNVALID_INDEX_EXPECTED : &str = "Accessing value our of tuple size must cause an InvalidIndexException";

pub trait AbstractTupleTest {

    fn get_supported_dimensions(&self) -> Vec<u32>;

    ///
    /// Created tuple must have all values at zero.
    ///
    fn create(&self, dim : u32) -> Box<dyn Tuple>;

    fn test_tuple(&self) {

        let supported_dimensions = self.get_supported_dimensions();

        for dim in supported_dimensions {
            let mut tuple = self.create(dim);
            assert_eq!(tuple.get_sample_count(), dim);
            self.testAllValue(&tuple, 0.0);
            //assertTrue(tuple.isAll(0.0));
            self.testCellGetSet(&mut tuple);
            self.testEquality(&mut tuple);
        }

    }

    ///
    /// Test all tuple values equal the expected value.
    ///
    fn testAllValue(&self, tuple: &Box<dyn Tuple>, expectedValue: f64) {
        let dim = tuple.get_sample_count();

        for c in 0..dim {
            let value = tuple.get(c);
            assert_equals_f64(value, expectedValue, TOLERANCE, "Value different at ["+c+"]");
        }

        //assertTrue(tuple.isAll(expectedValue));
        //assertFalse(tuple.isAll(expectedValue+1));
    }

    ///
    ///Test tuple value getters and setters.
    ///
    fn testCellGetSet(&self, tuple: &mut Box<dyn Tuple>) {
        let dim = tuple.get_sample_count();
        for i in 0..dim {
            tuple.set(i, i as f64 + 1 as f64);
            //assertEquals(i+1, tuple.get(i), TOLERANCE);
        }

        //test shortcuts
        //TODO

        //test out of range
        //tuple.set(-1, 10.0);
        //panic!(UNVALID_INDEX_EXPECTED);
        //catch_unwind(|| tuple.set(dim, 10.0));
    }

    fn testEquality(&self, tuple: &mut Box<dyn Tuple>) {
        let dim = tuple.get_sample_count();

        for i in 0..dim {
            tuple.set(i, i as f64 + 1 as f64);
        }

        //test equals itself
        //assertEquals(tuple, tuple);

        //test equals a copy
        let copy = tuple.copy();
        //assertEquals(tuple, copy);

        //test a equals a newly created tuple
        let mut newtuple = self.create(tuple.get_sample_count());
        //assertFalse(newtuple.equals(tuple));
        newtuple.set_from_tuple(tuple);
        //assertEquals(tuple, copy);

        //if (tuple.getNumericType() == Float32.TYPE
        //    || tuple.getNumericType() == Float64.TYPE) {
        //    //test NaN equality
        //    copy.set(0, Double.NaN);
        //    //assertFalse(copy.equals(tuple));
        //    tuple.set(0, Double.NaN);
        //    //assertEquals(copy, tuple);
        //}

    }
}
