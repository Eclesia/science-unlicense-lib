//
// Public Domain - unlicense.science
//

use std::io::Write;

pub trait Writable {

    fn open_write(&self) -> dyn Write;
}