use std::panic::panic_any;
use science_unlicense_math::tuple::MESSAGE_INVALID_COORD;
use science_unlicense_common::api::logging;

//
// Public Domain - unlicense.science
//
pub struct Extenti64 {
    values : Vec<i64>
}

impl Extenti64 {

    pub fn new_1d(size1 : i64) -> Self {
        return Extenti64{
            values : vec![size1]
        };
    }

    pub fn new_2d(size1 : i64, size2 : i64) -> Self {
        return Extenti64{
            values : vec![size1, size2]
        };
    }

    pub fn new_3d(size1 : i64, size2 : i64, size3 : i64) -> Self {
        return Extenti64{
            values : vec![size1, size2, size3]
        };
    }

    pub fn new_4d(size1 : i64, size2 : i64, size3 : i64, size4 : i64) -> Self {
        return Extenti64{
            values : vec![size1, size2, size3, size4]
        };
    }

    pub fn new_nd(sizes : Vec<i64>) -> Self {
        return Extenti64{
            values : sizes
        };
    }

    pub fn get_dimension(&self) -> u32 {
        return self.values.len() as u32;
    }

    pub fn get(&self, ordinate : u32) -> i64 {
        match self.values.get(ordinate as usize) {
            Some(v) => return *v,
            None => panic_any(logging::format(MESSAGE_INVALID_COORD, &ordinate))
        }
    }
}
