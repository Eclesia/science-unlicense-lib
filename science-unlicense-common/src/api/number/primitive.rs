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
    pub u32 code;
    pub f32 nb_bytes;
    pub u32 nb_bits;
}

pub const UNKNOWNED = Primitive { 0,    0,  0};
pub const BITS1     = Primitive { 1,1f/8f,  1};
pub const BITS2     = Primitive { 2,2f/8f,  2};
pub const BITS4     = Primitive { 3,4f/8f,  4};
pub const INT8      = Primitive { 4,    1,  8};
pub const UINT8     = Primitive { 5,    1,  8};
pub const INT16     = Primitive { 6,    2, 16};
pub const UINT16    = Primitive { 7,    2, 16};
pub const INT24     = Primitive { 8,    3, 24};
pub const UINT24    = Primitive { 9,    3, 24};
pub const INT32     = Primitive {10,    4, 32};
pub const UINT32    = Primitive {11,    4, 32};
pub const INT64     = Primitive {12,    8, 64};
pub const UINT64    = Primitive {13,    8, 64};
pub const INT128    = Primitive {14,   16,128};
pub const UINT128   = Primitive {15,   16,128};
pub const FLOAT16   = Primitive {16,    2, 16};
pub const FLOAT32   = Primitive {17,    4, 32};
pub const FLOAT64   = Primitive {18,    8, 64};
pub const FLOAT128  = Primitive {19,   16,128};

}
