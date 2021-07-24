//
// Public Domain - unlicense.science
//


//public static final int OP_SIN   = 1;
//public static final int OP_COS   = 2;
//public static final int OP_TAN   = 3;
//public static final int OP_ASIN  = 4;
//public static final int OP_ACOS  = 5;
//public static final int OP_ATAN  = 6;
//public static final int OP_LOG   = 7;
//public static final int OP_LOG10 = 8;
//public static final int OP_SQRT  = 9;
//public static final int OP_CQRT  = 10;

pub enum Operand {
    SIN
}

///
/// Arithmetic operations.
///
/// @author Bertrand COTE
/// @author Johann Sorel
///
pub trait Arithmetic {


    /**
     * Arithmetic type description.
     *
     * @return Arithmetic type, never null
     */
    fn get_type() -> Box<dyn ArithmeticType>;

    /**
     * Addition.
     *
     * @param other value
     * @return this + other
     */
    Arithmetic add(Arithmetic other);

    /**
     * Subtraction.
     *
     * @param other value
     * @return this - other
     */
    Arithmetic subtract(Arithmetic other);

    /**
     * Multiplication.
     *
     * @param other value
     * @return this * other
     */
    Arithmetic mult(Arithmetic other);

    /**
     * Division.
     *
     * @param other value
     * @return this / other
     */
    Arithmetic divide(Arithmetic other);

    /**
     * Zero: neutral element for additions.
     *
     * @return zero element
     */
    Arithmetic zero();

    /**
     * Test if this is zero.
     *
     * @return true if this is zero
     */
    boolean isZero();

    /**
     * One: neutral element for multiplications.
     *
     * @return one element
     */
    Arithmetic one();

    /**
     * Test if this is one.
     *
     * @return true if this is one
     */
    boolean isOne();

    /**
     * Power.
     *
     * @param n power
     * @return this ^ n
     */
    Arithmetic pow(int n);

    /**
     * Apply given operand.
     *
     * @param opCode
     * @return
     */
    Arithmetic op(int opCode);

}
