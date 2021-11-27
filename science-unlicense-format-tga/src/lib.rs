//
// Public Domain - unlicense.science
//
mod tgaformat; pub use crate::tgaformat::TGAFormat;
mod tgareader; pub use crate::tgareader::TGAReader;

use science_unlicense_common::api::Module;
use science_unlicense_common::api::registry;
use science_unlicense_encoding::api::store::Format;

///
/// Crate initialisation
///
pub fn init() {
    science_unlicense_image::init();

    let f = TGAFormat::new();

    let module = Module {
        name: String::from("tga"),
        entries: vec![Box::new(f)]
    };
    registry::register_module(module);

    //declare explicite reflection
    traitcast::traitcast!(struct TGAFormat: Format);
}

#[cfg(test)]
mod tests {
    
}
