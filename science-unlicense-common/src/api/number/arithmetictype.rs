//
// Public Domain - unlicense.science
//

///
/// An arithmetic type defines a value class which can support the different arithmetic operations.
///
/// author Johann Sorel
///
pub trait ArithmeticType {

    ///
    /// Create a new number instance with it's initial value.
    ///
    fn create(&self, value: Box<dyn Arithmetic>) -> Box<dyn Arithmetic>;

    ///
    /// Returns the Numeric class this type represent.
    ///
    /// @return Class of type Number
    ///
    fn get_value_type(&self) -> TypeId;

    ///
    /// @return Code from Primitive.*
    ///
    fn get_primitive_code(&self) -> Primitive;
}
