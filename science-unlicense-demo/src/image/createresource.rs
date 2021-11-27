//
// Public Domain - unlicense.science
//

use std::any::{Any};
use science_unlicense_encoding::api::store::{ResourceSet, Resource};
use science_unlicense_common::api::Polymorph;
use std::error::Error;

pub fn main() {

    let t = MyFE{};

    let b = science_unlicense_common::api::reflection::cast_ref::<MyFE,dyn ResourceSet>(&t);

    match b {
        Some(_rs) => println!("found"),
        _ => println!("not found")
    }
}

struct MyFE {
}

impl Polymorph for MyFE {
    
    fn as_any(&self) -> &dyn Any {
        return self;
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        return self;
    }
}

impl Resource for MyFE {

}

impl ResourceSet for MyFE {

    fn get_elements(&self) -> Result<Vec<Box<dyn Resource>>, Box<dyn Error>> {
        todo!()
    }

    fn find(&self, _id: &str) -> Result<Box<dyn Resource>, Box<dyn Error>> {
        todo!()
    }
}