use science_unlicense_common::api::registry;
use science_unlicense_encoding::api::store::FormatHandle;
use std::borrow::Borrow;

pub fn main() {
    formats2();
}

pub fn formats2() {

    let modules = registry::get_modules();

    let ite = modules.iter();
    for module in ite {
        println!("module {}", module.name);

        //search for ImageFormat types
        let q = module.entries.iter();
        for polymorph in q {
            let cdt = FormatHandle::cast(polymorph.borrow());
            match cdt {
                Some(format) => println!("found format : {}", format.get_identifier() ),
                _ => println!("not found")
            }
        }
    }

}
