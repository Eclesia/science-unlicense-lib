//
// Public Domain - unlicense.science
//
use crate::api::system::SampleSystem;

pub struct UndefinedSystem {
    nb_components : u32
}

impl UndefinedSystem {

    pub fn new(size: u32) -> Self {
        return UndefinedSystem { nb_components: size };
    }

}

impl SampleSystem for UndefinedSystem {

    fn get_num_components(&self) -> u32 {
        return self.nb_components;
    }
}
