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

    fn as_any(&self) -> &dyn Any {
        return self;
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        return self;
    }
 
}