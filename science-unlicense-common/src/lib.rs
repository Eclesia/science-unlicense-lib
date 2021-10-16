// Public Domain - unlicense.science

pub mod api {
    pub mod model {
        pub mod doc {
            mod documenttype; pub use documenttype::DocumentType;
            mod fieldtype; pub use fieldtype::FieldType;
        }
        mod multiplicitytype; pub use multiplicitytype::MultiplicityType;
        mod presentable; pub use presentable::Presentable;
        mod valuetype; pub use valuetype::ValueType;
    }
    pub mod number {
        pub const TYPE_FLOAT32 : Float32 = Float32{};
        pub const TYPE_FLOAT64 : Float64 = Float64{};

        mod arithmetic; pub use arithmetic::Arithmetic;
        mod operand; pub use operand::Operand;
        mod arithmetictype; pub use arithmetictype::ArithmeticType;
        mod float32; pub use float32::Float32;
        mod float64; pub use float64::Float64;
        pub mod primitive;
    }
    mod logger; pub use logger::Logger;
    mod polymorph; pub use polymorph::Polymorph;
    mod module; pub use module::Module;
    mod messageerror; pub use messageerror::MessageError;
    pub mod logging;
    pub mod registry;
    pub mod asserts;
    
}

///
/// Crate initialisation
///
pub fn init() {
    //does nothing for now
}

#[cfg(test)]
mod tests {

}
