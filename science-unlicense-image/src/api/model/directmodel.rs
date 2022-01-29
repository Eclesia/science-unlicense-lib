//
// Public Domain - unlicense.science
//
use crate::api::Image;
use crate::api::model::ImageModel;
use science_unlicense_geometry::api::tuple::TupleArray;

pub struct DirectModel {
    nb_samples : u32    
}

impl DirectModel {

    pub fn new(nb_samples: u32) -> Self {
        return DirectModel {
            nb_samples: nb_samples
        };
    }
}

impl ImageModel for DirectModel {

    fn get_nb_samples(&self) -> u32 {
        return self.nb_samples;
    }

    fn read(&self, image: &Box<&dyn Image>) -> Box<&dyn TupleArray> {
        panic!("TODO");
    }

    fn read_write(&self, image: &Box<dyn Image>) -> Box<dyn TupleArray> {
        panic!("TODO");
    }
}