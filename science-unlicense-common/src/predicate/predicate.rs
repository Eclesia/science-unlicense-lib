//
// Public Domain - unlicense.science
//

use crate::predicate::Expression;

///
/// A predicate is an expression which necessarly return a Boolean
/// value. They are mainly used to filter objects.
///
/// @author Johann Sorel
/// @author Yann D'Isanto
///
pub trait Predicate<T> : Expression<T,bool> {

    /**
     * Constant predicate, always evaluating to TRUE.
     */
    //public static final Predicate TRUE = new Predicate() {
    //    public Boolean evaluate(Object candidate) {
    //        return Boolean.TRUE;
    //    }
    //    public String toString() {
    //        return "Predicate.TRUE";
    //    }
    //};

    /**
     * Constant predicate, always evaluating to FALSE.
     */
    //public static final Predicate FALSE = new Predicate() {
    //    public Boolean evaluate(Object candidate) {
    //        return Boolean.FALSE;
    //    }
    //    public String toString() {
    //        return "Predicate.FALSE";
    //    }
    //};

    /**
     * Evaluate given object in context.
     * Result is necessarly a Boolean value (not null).
     *
     * @param candidate, object to evaluate
     * @param context the evaluation context, can be null
     * @return Boolean, never null
     */
    fn evaluate(&self, candidate : T) -> bool;

}