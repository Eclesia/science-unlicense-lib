//
// Public Domain - unlicense.science
//
mod image;
use science_unlicense_demo::init;

fn main() {
    init();
    image::listformats::main()
}
