//
// Public Domain - unlicense.science
//

use science_unlicense_encoding::api::store::{Format, Store};
use science_unlicense_encoding::api::io::Readable;
use std::error::Error;
use std::fs::File;
use crate::TGAReader;

pub struct TGAFormat {

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

    fn can_decode(&self, _input: &mut dyn Readable) -> Result<bool, Box<dyn Error>> {
        todo!()
    }

    fn search_end(&self, _input: &mut dyn Readable, _fullscan: bool) -> Result<u64, Box<dyn Error>> {
        todo!()
    }

    fn get_parameters(&self) {
        todo!()
    }

    fn open(&self, _source: File) -> Result<Box<dyn Store>, Box<dyn Error>> {
        let _reader = TGAReader::new();
        todo!()
    }
}
