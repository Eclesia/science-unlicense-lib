use science_unlicense_common::api::registry;
use science_unlicense_encoding::api::store::Format;
use science_unlicense_common::api::reflection;
use std::any::Any;

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
            let p = polymorph.as_any();
            let cdt = reflection::cast_ref::<dyn Any,dyn Format>(p);
            match cdt {
                Some(format) => println!("found format : {}", format.get_identifier() ),
                None => println!("not found")
            }
        }
    }

}
