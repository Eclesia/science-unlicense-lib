//
// Public Domain - unlicense.science
//

use science_unlicense_encoding::api::store::{FormatRegistryEntry, FormatHandle, SimpleResource};

mod tgaformat;
mod tgareader;

pub use crate::tgaformat::TGAFormat;
use science_unlicense_common::api::{Module, Polymorph};
use science_unlicense_common::api::registry;

///
/// Crate initialisation
///
pub fn init() {
    science_unlicense_image::init();
    let entry = FormatRegistryEntry{name: String::from("tga")};

    let f = TGAFormat::new();
    let handle = FormatHandle::new(Box::new(f));
    let mut pl = SimpleResource::new();
    pl.put(Box::new(handle));
    let b: Box<dyn Polymorph> = Box::new(pl);

    let module = Module {
        name: String::from("tga"),
        entries: vec![b]
    };
    registry::registerModule(module);
}

#[cfg(test)]
mod tests;
