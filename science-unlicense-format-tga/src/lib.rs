//
// Public Domain - unlicense.science
//

use science_unlicense_encoding::api::store::{Format, Store, FormatRegistryEntry};

mod tgaformat;
mod tgareader;

pub use crate::tgaformat::TGAFormat;
use std::borrow::Borrow;


inventory::submit! {
    FormatRegistryEntry{name: String::from("tga")}
}

///
/// Crate initialisation
///
pub fn init() {
    science_unlicense_image::init();
    let entry = FormatRegistryEntry{name: String::from("tga")};

}

#[cfg(test)]
mod tests;
