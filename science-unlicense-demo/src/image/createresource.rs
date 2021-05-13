//
// Public Domain - unlicense.science
//

use std::any::{Any, TypeId};
use science_unlicense_encoding::api::store::{SimpleResource, Store, ResourceSetHandle, ResourceSet, Resource};
use science_unlicense_common::api::Polymorph;

fn main() {

    let mut res = SimpleResource::new();
    let t = MyFE{};
    let h = ResourceSetHandle::new(Box::new(t));
    res.put(Box::new(h));

    let b = ResourceSetHandle::cast(&res);

    match b {
        Some(tt) => println!("found"),
        _ => println!("not found")
    }
}

struct MyFE {
}

impl Polymorph for MyFE {
    fn get(&self, key: &TypeId) -> Option<&Box<dyn Any>> {
        todo!()
    }
}

impl Resource for MyFE {

}

impl ResourceSet for MyFE {

    fn get_elements(&self) -> Result<Vec<Box<dyn Resource>>, String> {
        todo!()
    }

    fn find(&self, id: &str) -> Result<Box<dyn Resource>, String> {
        todo!()
    }
}