//
// Public Domain - unlicense.science
//

use std::any::Any;
use std::any::TypeId;

pub trait Polymorph {
    fn get(&self, key: &TypeId) -> Option<&Box<dyn Any>>;
}