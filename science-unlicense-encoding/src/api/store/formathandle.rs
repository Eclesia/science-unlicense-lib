use crate::api::store::Format;
use science_unlicense_common::api::Polymorph;
use std::any::TypeId;

//
// Public Domain - unlicense.science
//
pub struct FormatHandle {
    handle : Box<dyn Format>
}

impl FormatHandle {

    pub fn new(fs: Box<dyn Format>) -> FormatHandle {
        return FormatHandle{
            handle : fs
        };
    }

    pub fn cast<'a>(cdt: &'a dyn Polymorph) -> Option<&'a dyn Format>{
        let ti : TypeId = TypeId::of::<Self>();
        let x = cdt.get(&ti)?;
        match x.downcast_ref::<FormatHandle>() {
            Some(value) => Some(value.handle.as_ref()),
            None => None
        }
    }
}