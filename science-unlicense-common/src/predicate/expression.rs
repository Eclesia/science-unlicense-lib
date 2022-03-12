//
// Public Domain - unlicense.science
//
pub trait Expression<T,R> {

    ///
    /// Evaluation given object in context.
    ///
    /// @param candidate, object to evaluate
    /// @return result of the evaluation
    ///
    fn evaluate(&self, candidate: T) -> R;

}