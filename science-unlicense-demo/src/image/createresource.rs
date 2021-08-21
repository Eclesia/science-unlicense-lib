//
// Public Domain - unlicense.science
//

use std::any::{Any, TypeId};
use science_unlicense_encoding::api::store::{SimpleResource, ResourceSetHandle, ResourceSet, Resource};
use science_unlicense_common::api::Polymorph;
use std::error::Error;

pub fn main() {

    let mut res = SimpleResource::new();
    let t = MyFE{};
    let h = ResourceSetHandle::new(Box::new(t));
    res.put(Box::new(h));

    let b = ResourceSetHandle::cast(&res);

    match b {
        Some(_rs) => println!("found"),
        _ => println!("not found")
    }
}

struct MyFE {
}

impl Polymorph for MyFE {
    fn get(&self, _key: &TypeId) -> Option<&Box<dyn Any>> {
        todo!()
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