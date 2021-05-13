//
// Public Domain - unlicense.science
//

use crate::api::io::{Readable, Writable};


pub trait Path: Readable + Writable{

    fn can_read() -> bool;

    fn can_write() -> bool;

    fn can_seek() -> bool;

    fn open() -> u32;

}
