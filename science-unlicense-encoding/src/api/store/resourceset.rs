//
// Public Domain - unlicense.science
//
use crate::api::store::Resource;
use std::error::Error;

pub trait ResourceSet : Resource {

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