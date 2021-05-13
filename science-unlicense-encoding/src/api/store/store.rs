//
// Public Domain - unlicense.science
//

use crate::api::store::Resource;
use crate::api::store::Format;
use science_unlicense_common::api::Polymorph;
use std::any::TypeId;
use science_unlicense_common::api::Logger;


pub struct StoreHandle {
    handle : Box<dyn Store>
}

impl StoreHandle {

    pub fn new(fs: Box<dyn Store>) -> StoreHandle {
        return StoreHandle{
            handle : fs
        };
    }

    pub fn cast<'a>(cdt: &'a dyn Polymorph) -> Option<&'a dyn Store>{
        let ti : TypeId = TypeId::of::<Self>();
        let x = cdt.get(&ti)?;
        match x.downcast_ref::<StoreHandle>() {
            Some(value) => Some(value.handle.as_ref()),
            None => None
        }
    }
}

pub trait Store : Resource {

    ///
    /// Get the original format definition.
    ///
    /// @return Format, never null
    ///
    fn get_format(&self) -> Box<dyn Format>;

    ///
    /// Get the store logger.
    ///
    /// @return Logger, never null.
    ///
    fn get_logger(&self) -> dyn Logger;

    ///
    /// Set store exception logger.
    /// Exception should always be send back to the caller, yet minor, expected errors
    /// or debugging informations can be logged.
    ///
    /// @param logger, never null
    ///
    fn set_logger(&self, logger: dyn Logger);

    ///
    /// Get the store root object, often a Path or a Stream but could be anything.
    ///
    /// @return input object, can be null.
    ///
    fn get_input(&self);

    ///
    /// Release any resource used by this store.
    /// The store should not be used anymore after this call.
    ///
    fn dispose(&self);

}