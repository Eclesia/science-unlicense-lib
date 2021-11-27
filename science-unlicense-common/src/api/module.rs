//
// Public Domain - unlicense.science
//
use crate::api::Polymorph;

pub struct Module {
    pub name : String,
    pub entries : Vec<Box<dyn Polymorph>>
}

impl Module {
}