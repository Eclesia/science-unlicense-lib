//
// Public Domain - unlicense.science
//

use crate::api::store::FormatRegistryEntry;
use linkme::distributed_slice;

#[distributed_slice]
pub static FORMATS: [fn() -> FormatRegistryEntry] = [..];

#[distributed_slice(FORMATS)]
static LOCAL_FORMAT: fn() -> FormatRegistryEntry = format_local;


fn format_local() -> FormatRegistryEntry {
    let str : String = String::from("local2");
    return FormatRegistryEntry{name:str};
}

pub fn formats2() {
    for flag in FORMATS {
        println!("-{}", flag().name);
    }
}

pub fn formats() {
    for flag in inventory::iter::<FormatRegistryEntry> {
        println!("-{}", flag.name);
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
