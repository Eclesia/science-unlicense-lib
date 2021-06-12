//
// Public Domain - unlicense.science
//

use science_unlicense_encoding::api::store::{Format, Store, FormatRegistryEntry};
use science_unlicense_encoding::api::store::formats::FORMATS;
use science_unlicense_encoding::api::io::Readable;
use std::error::Error;
use std::fs::File;
use linkme::distributed_slice;

pub struct TGAFormat {

}

#[distributed_slice(FORMATS)]
static TGA_FORMAT: fn() -> FormatRegistryEntry = format_tga;


fn format_tga() -> FormatRegistryEntry {
    let str : String = String::from("tga");
    return FormatRegistryEntry{name:str};
}

impl TGAFormat {

    pub fn new() -> Self{
        return TGAFormat{};
    }
}

impl Format for TGAFormat {

    fn get_identifier(&self) -> &str {
        return "tga";
    }

    fn get_short_name(&self) -> &str {
        return "tga";
    }

    fn get_long_name(&self) -> &str {
        todo!()
    }

    fn get_mime_types(&self) -> &Vec<&str> {
        todo!()
    }

    fn get_extensions(&self) -> &Vec<&str> {
        todo!()
    }

    fn get_signature(&self) -> &Vec<&Vec<u8>> {
        todo!()
    }

    fn is_subset_format(&self) -> bool {
        todo!()
    }

    fn get_resource_types(&self) -> &Vec<&str> {
        todo!()
    }

    fn can_decode(&self, input: &mut dyn Readable) -> Result<bool, Box<dyn Error>> {
        todo!()
    }

    fn search_end(&self, input: &mut dyn Readable, fullscan: bool) -> Result<u64, Box<dyn Error>> {
        todo!()
    }

    fn get_parameters(&self) {
        todo!()
    }

    fn open(&self, source: File) -> Result<Box<dyn Store>, Box<dyn Error>> {
        todo!()
    }
}
