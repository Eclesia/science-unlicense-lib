//
// Public Domain - unlicense.science
//

use crate::api::store::FormatRegistryEntry;
use science_unlicense_common::api::registry;

fn format_local() -> FormatRegistryEntry {
    let str : String = String::from("local2");
    return FormatRegistryEntry{name:str};
}

pub fn formats2() {
}

pub fn formats() {

    let modules = registry::getModules();

    let ite = modules.iter();
    for module in ite {
        //search for ImageFormat types
        let q = module.entries.iter();
        for poly in q {
        }
        println!("found {}", module.name);
    }

}


pub static SINGLETON : FormatRegistry = FormatRegistry {list: vec![]};

pub struct FormatRegistry {
    list: Vec<FormatRegistryEntry>
}

impl FormatRegistry {

    pub fn add(&mut self, entry: FormatRegistryEntry) {
        self.list.push(entry);
        //let mut data = LIST2.lock().unwrap();
        //data.push(entry);
    }
}
