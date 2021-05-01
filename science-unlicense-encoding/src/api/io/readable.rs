//
// Public Domain - unlicense.science
//

use std::io::Read;

pub trait Readable {

    fn open_read(&self) -> dyn Read;

}