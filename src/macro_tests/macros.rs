/// Generates a match statement
/// ## Params
/// 1. *$expression:expr* is the expression that we're gonna compare to the other expressions
/// 
/// 2. *$x:expr...* are the expressions that we're matching against the expression
/// 
/// You have to add at least one expression to match against
/// 
/// # Example: 
/// ```
/// let a = 0;
/// let mut test = qmatch!(a,1,0); // true
/// test = qmatch!(a,1,3); // false
/// 
/// println!("Hello, world! {test}");
/// ```
#[macro_export]
macro_rules! qmatch {
    ( $expression:expr, $( $x:expr ),+ ) => {
        {
            match $expression {
                $( $x => true, )*
                _ => false,
            }
        }
    };
}
/// Generates a for loop statement
/// 
/// This macro has two overloads
/// 
/// ## Overload 1
/// 1. *$index:ident* is the name of the index
/// 
/// note: between 1. and 2. you must add *"/"* (so as to indicate an index)
/// 
/// 2. *$length:expr* is the length of the for loop
/// 3. $block:block is the block of the for loop
/// 
/// ## Overload 2
/// 1. *$item:ident* s the name of the item
/// 
/// note: between 1. and 2. you must add *","* (so as to indicate an item)
/// 
/// 2. *$arr:expr* is the array of the for loop
/// 3. $block:block is the block of the for loop
/// 
/// # Example: 
/// ```
/// let mut list = [1,2,3,4];
///
/// qfor!(item, list, {
///     *item = 0;
/// });
/// qfor!(i / list.len(), {
///     list[i] = 0;
/// });
/// ```
#[macro_export]
macro_rules! qfor {
    ( $index:ident / $length:expr, $block:block ) => {
        {
            for $index in 0..$length $block
        }
    };
    ( $item:ident,  $arr:expr, $block:block ) => {
        {
            for $item in &mut $arr $block
        }
    };
}
/// Generates a function declaration statement
/// ## Params
/// 1. *$fn_name:ident* is the function name
/// 
/// 2. *$ret:ty* is the return type
/// 
/// 3. *($name:ident/$type:ty)...* are the parameters with a given *$name* and a given *$type*
/// 
/// 4. *$block:block* is the body of the function
/// 
/// # Example: 
/// ```
/// qfn!(example_function, String, p1/bool, {
///     if p1 {
///         return String::from("true");
///     }else {
///         return String::from("false");
///     }
/// });
/// 
/// println!("{}", example_function(true));
/// ```
#[macro_export]
macro_rules! qfn {
    ($fn_name:ident, $ret:ty, $( $name:ident/$type:ty ),*, $block:block) => {
        pub fn $fn_name($($name:$type),*) -> $ret $block
    }
}
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
    ($op:tt, $last:expr, $( $expr:expr ),+) => {
        {
            $( $expr $op )* $last
        }
    };
}
#[macro_export]
macro_rules! qimpl {
    () => {
        
    };
}
#[macro_export]
macro_rules! qenum {
    ( $enum:ident; $( $name:ident$( ($( $wrapped:ty ),*) )? ),* ) => {
        pub enum $enum {
            $( $name$( ($($wrapped),*) )? ),*
        }
    };
    ( [$( $derive:ty ),*] $enum:ident; $( $name:ident$( ($( $wrapped:ty ),*) )? ),* ) => {
        #[derive($($derive),*)]
        pub enum $enum {
            $( $name$( ($($wrapped),*) )? ),*
        }
    };
}

#[macro_export]
macro_rules! qmod {
    ( $( $(/$p:tt)?$t:ident ),* ) => {
        $( $($p)? mod $t; )*
    };
}