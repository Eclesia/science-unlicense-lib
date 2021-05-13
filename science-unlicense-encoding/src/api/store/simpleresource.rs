//
// Public Domain - unlicense.science
//

use std::collections::HashMap;
use std::any::TypeId;
use std::any::Any;
use science_unlicense_common::api::Polymorph;

pub struct SimpleResource {
    traits: HashMap<TypeId, Box<dyn Any>>
}

impl SimpleResource {

    pub fn new() -> Self {
        return SimpleResource{
            traits : HashMap::new()
        }
    }

    pub fn put(&mut self, value : Box<dyn Any>) {
        let t = (*value).type_id();
        self.traits.insert(t, value);
    }
}

impl Polymorph for SimpleResource {

    fn get(&self, key: &TypeId) -> Option<&Box<dyn Any>> {
        return self.traits.get(key);
    }
}