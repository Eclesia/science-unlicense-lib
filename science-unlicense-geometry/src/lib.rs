// Public Domain - unlicense.science

//! GEOMETRY CRATE
//!
//! for everything about geometries

pub mod api {
    pub mod tuple {
        mod tuplearray; pub use tuplearray::TupleArray;
        mod tuplearray1f32; pub use tuplearray1f32::TupleArray1f32;
        mod tuplespace; pub use tuplespace::TupleSpace;
        pub mod tuplearrays;
    }
    
    mod extenti64; pub use extenti64::Extenti64;
    mod extentf64; pub use extentf64::Extentf64;
}

///
/// Crate initialisation
///
pub fn init() {
    science_unlicense_math::init();
}

#[cfg(test)]
mod tests;
