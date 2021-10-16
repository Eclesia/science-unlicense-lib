//
// Public Domain - unlicense.science
//
use crate::api::number::ArithmeticType;
use crate::api::number::Operand;

///
/// Arithmetic operations.
///
/// @author Bertrand COTE
/// @author Johann Sorel
///
pub trait Arithmetic {

    ///
    /// Arithmetic type description.
    ///
    /// @return Arithmetic type, never null
    ///
    fn get_type(&self) -> Box<dyn ArithmeticType>;

    ///
    /// Addition.
    ///
    /// @param other value
    /// @return this + other
    ///
    fn add(&self, other: Box<dyn Arithmetic>) -> Box<dyn Arithmetic>;

    ///
    /// Subtraction.
    ///
    /// @param other value
    /// @return this - other
    ///
    fn subtract(&self, other: Box<dyn Arithmetic>) -> Box<dyn Arithmetic>;

    ///
    /// Multiplication.
    ///
    /// @param other value
    /// @return this * other
    ///
    fn mult(&self, other: Box<dyn Arithmetic>) -> Box<dyn Arithmetic>;

    ///
    /// Division.
    ///
    /// @param other value
    /// @return this / other
    ///
    fn divide(&self, other: Box<dyn Arithmetic>) -> Box<dyn Arithmetic>;

    ///
    /// Zero: neutral element for additions.
    ///
    /// @return zero element
    ///
    fn zero(&self) -> Box<dyn Arithmetic>;

    ///
    /// Test if this is zero.
    ///
    /// @return true if this is zero
    ///
    fn is_zero(&self) -> bool;

    ///
    /// One: neutral element for multiplications.
    ///
    /// @return one element
    ///
    fn one(&self) -> Box<dyn Arithmetic>;

    ///
    /// Test if this is one.
    ///
    /// @return true if this is one
    ///
    fn is_one(&self) -> bool;

    ///
    /// Power.
    ///
    /// @param n power
    /// @return this ^ n
    ///
    fn pow(&self, n: i32) -> Box<dyn Arithmetic>;

    ///
    /// Apply given operand.
    ///
    /// @param opCode
    /// @return
    ///
    fn op(&self, operand: Operand) -> Box<dyn Arithmetic>;

}
