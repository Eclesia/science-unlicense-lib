//
// Public Domain - unlicense.science
//
use crate::api::model::ImageModel;
use science_unlicense_math::api::Tuple;

pub trait Image {
    fn get_tuple(&self, coordinate: &impl Tuple, model: &impl ImageModel) -> Box<dyn Tuple>;
}
