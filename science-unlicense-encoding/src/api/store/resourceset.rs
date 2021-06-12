//
// Public Domain - unlicense.science
//


use crate::api::store::Resource;
use std::any::TypeId;
use science_unlicense_common::api::Polymorph;
use std::error::Error;

pub struct ResourceSetHandle {
    handle : Box<dyn ResourceSet>
}

impl ResourceSetHandle {

    pub fn new(fs: Box<dyn ResourceSet>) -> ResourceSetHandle {
        return ResourceSetHandle{
            handle : fs
        };
    }

    pub fn cast<'a>(cdt: &'a dyn Polymorph) -> Option<&'a dyn ResourceSet>{
        let ti : TypeId = TypeId::of::<Self>();
        let x = cdt.get(&ti)?;
        match x.downcast_ref::<ResourceSetHandle>() {
            Some(value) => Some(value.handle.as_ref()),
            None => None
        }
    }
}

pub trait ResourceSet {

    ///
    /// List resources available
    ///
    fn get_elements(&self) -> Result<Vec<Box<dyn Resource>>, Box<dyn Error>>;

    ///
    /// Find resource with given id.
    ///
    /// @param id, not null
    /// @return resource or null if no resource for given identifier found
    ///
    fn find(&self, id: &str) -> Result<Box<dyn Resource>, Box<dyn Error>>;
}