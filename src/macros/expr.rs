/// Generates a series of comparison expressions
/// ## Params
/// 1. *$op:tt* is the operator symbol
/// 
/// 2. *$expr:expr...* are the expressions to be compared with givne the *$op*
/// 
/// # Example: 
/// ```
/// let a = 1
/// let b = 98
/// 
/// if qop!(||, b, a) {
///     //do something
/// } else {
///     //do something else
/// }
/// ```
#[macro_export]
macro_rules! qop {
    ($op:tt, $last:expr, $( $expr:tt ),+) => {
        {
            $( $expr $op )* $last
        }
    };
}