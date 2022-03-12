
use crate::api::Image;
use crate::api::MODEL_RAW;
use crate::api::MODEL_COLOR;
use crate::api::model::ImageModel;
use science_unlicense_math::tuple::Tuple;
use science_unlicense_geometry::api::Extenti64;
use science_unlicense_geometry::api::tuple::TupleArray;
use std::collections::HashMap;

pub struct DefaultImage {
    extent : Extenti64,
    models : HashMap<String, Box<dyn ImageModel>>
}

impl Image for DefaultImage {
        
    fn get_extent(&self) -> &Extenti64 {
        return &self.extent;
    }

    fn get_models(&self) -> Vec<&String> {
        let mut keys : Vec<&String> = Vec::new();
        for key in self.models.keys() {
            keys.push(key);
        }
        return keys;
    }

    fn get_raw_model(&self) -> &Box<dyn ImageModel> {
        match &self.models.get(MODEL_RAW) {
            Some(t) => {return *t;}
            None => {panic!("Raw model is mandatory");}
        };
    }

    fn get_color_model(&self) -> Option<&Box<dyn ImageModel>> {
        return self.models.get(MODEL_COLOR);
    }

    fn get_tuple_buffer<'a>(&self, model : &'a dyn ImageModel) -> Box<&'a dyn TupleArray> {
        return model.read(&Box::new(self));
    }

    fn get_tuple(&self, coordinate: &dyn Tuple, model: &dyn ImageModel) -> Box<dyn Tuple> {
        let tuple_array = self.get_tuple_buffer(model);
        return tuple_array.get(coordinate);
    }
}
