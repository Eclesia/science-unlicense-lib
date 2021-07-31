//
// Public Domain - unlicense.science
//

///
/// Constant of the different primitive types.
///
/// @author Johann Sorel
///

pub enum Primitives {
    UNKNOWNED,
    BITS1,
    BITS2,
    BITS4,
    INT8,
    UINT8,
    INT16,
    UINT16,
    INT24,
    UINT24,
    INT32,
    UINT32,
    INT64,
    UINT64,
    INT128,
    UINT128,
    FLOAT16,
    FLOAT32,
    FLOAT64,
    FLOAT128
}

pub struct Primitive {
    pub code: u32,
    pub nb_bytes: f32,
    pub nb_bits: u32
}

pub const UNKNOWNED: Primitive = Primitive {code: 0, nb_bytes:        0f32, nb_bits:  0};
pub const BITS1: Primitive     = Primitive {code: 1, nb_bytes:1f32/8f32, nb_bits:  1};
pub const BITS2: Primitive     = Primitive {code: 2, nb_bytes:2f32/8f32, nb_bits:  2};
pub const BITS4: Primitive     = Primitive {code: 3, nb_bytes:4f32/8f32, nb_bits:  4};
pub const INT8: Primitive      = Primitive {code: 4, nb_bytes:     1f32, nb_bits:  8};
pub const UINT8: Primitive     = Primitive {code: 5, nb_bytes:     1f32, nb_bits:  8};
pub const INT16: Primitive     = Primitive {code: 6, nb_bytes:     2f32, nb_bits: 16};
pub const UINT16: Primitive    = Primitive {code: 7, nb_bytes:     2f32, nb_bits: 16};
pub const INT24: Primitive     = Primitive {code: 8, nb_bytes:     3f32, nb_bits: 24};
pub const UINT24: Primitive    = Primitive {code: 9, nb_bytes:     3f32, nb_bits: 24};
pub const INT32: Primitive     = Primitive {code:10, nb_bytes:     4f32, nb_bits: 32};
pub const UINT32: Primitive    = Primitive {code:11, nb_bytes:     4f32, nb_bits: 32};
pub const INT64: Primitive     = Primitive {code:12, nb_bytes:     8f32, nb_bits: 64};
pub const UINT64: Primitive    = Primitive {code:13, nb_bytes:     8f32, nb_bits: 64};
pub const INT128: Primitive    = Primitive {code:14, nb_bytes:    16f32, nb_bits:128};
pub const UINT128: Primitive   = Primitive {code:15, nb_bytes:    16f32, nb_bits:128};
pub const FLOAT16: Primitive   = Primitive {code:16, nb_bytes:     2f32, nb_bits: 16};
pub const FLOAT32: Primitive   = Primitive {code:17, nb_bytes:     4f32, nb_bits: 32};
pub const FLOAT64: Primitive   = Primitive {code:18, nb_bytes:     8f32, nb_bits: 64};
pub const FLOAT128: Primitive  = Primitive {code:19, nb_bytes:    16f32, nb_bits:128};

