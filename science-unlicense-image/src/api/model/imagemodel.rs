// Public Domain - unlicense.science

use crate::api::Image;
use science_unlicense_geometry::api::tuple::TupleArray;

pub trait ImageModel {
    fn view(&self, image: &impl Image) -> Box<dyn TupleArray>;
}
