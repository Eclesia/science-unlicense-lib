//
// Public Domain - unlicense.science
//

use crate::api::io::{Readable, Writable};


pub trait Path: Readable + Writable{

    fn canRead() -> bool;

    fn canWrite() -> bool;

    fn canSeek() -> bool;

    fn open() -> u32;

}
