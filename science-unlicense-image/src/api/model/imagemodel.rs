//
// Public Domain - unlicense.science
//
use crate::api::Image;
use science_unlicense_geometry::api::tuple::TupleArray;

pub trait ImageModel {

    fn get_nb_samples(&self) -> u32;

    fn read(&self, image: &Box<&dyn Image>) -> Box<&dyn TupleArray>;

    fn read_write(&self, image: &Box<dyn Image>) -> Box<dyn TupleArray>;
}
