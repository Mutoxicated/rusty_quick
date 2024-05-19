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
    ( $expression:expr; $( $x:expr => $block:block ),+, _$else:block ) => {
        {
            match $expression {
                $( $x => $block, )*
                _ => $else,
            }
        }
    };
    ( $expression:expr; $( $x:expr ),+ ) => {
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
    ( $index:ident; $length:expr; $($token:tt)* ) => {
        {
            for $index in 0..$length {$($token)*}
        }
    };
    ( $index:ident; $start:expr, $length:expr; $($token:tt)* ) => {
        {
            for $index in $start..$length {$($token)*}
        }
    };
    ( $item:ident: $arr:expr; $($token:tt)* ) => {
        {
            for $item in &mut $arr {$($token)*}
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
    ($public:vis$ fn_name:ident, $ret:ty, $( $name:ident/$type:ty ),*, $block:block) => {
        $public fn $fn_name($($name:$type),*) -> $ret $block
    };
    ($public:vis $fn_name:ident, $ret:ty, $block:block) => {
        $public fn $fn_name() -> $ret $block
    };
    ($public:vis $fn_name:ident, $block:block) => {
        $public fn $fn_name() $block
    };
    ($public:vis $fn_name:ident) => {
        $public fn $fn_name() 
    };
}
/// Generates an implement statement
/// ## Params
/// 
/// (Optional param: *\[$trait:ty...\]*) you can write a trait to implement to the type
/// 1. *$t:ty* is the type being implemented upon
/// note: between 1. and 2. you must write *";"*
/// 2. you can write anything here that you want to implement
/// 
/// # Example: 
/// ```
/// pub struct ExampleStruct {
///     a: u32,
///     b: u8,
/// }
/// 
/// qimpl!{ExampleStruct; 
///     fn new(a:u32, b:u32) -> Self {
///         return Self{
///             a,
///             b,
///         }
///     }
/// }
/// 
/// pub trait ExampleTrait {
///     fn example();
/// }
/// 
/// qimpl!{[ExampleTrait] ExampleStruct; 
///     fn example() {
///         println!("This is an example!");
///     }
/// }
/// ```
#[macro_export]
macro_rules! qimpl {
    ($t:ident$(<$($wt:ident),*>)?; $($token:tt)*) => {
        impl$(<$($wt),*>)? $t$(<$($wt),*>)? {$($token)*}
    };

    ([$tr:ident] $t:ident$(<$($wt:ident),*>)?; $($token:tt)*) => {
        impl$(<$($wt),*>)? $tr for $t$(<$($wt),*> )? {$($token)*}
    };
}
#[macro_export]
macro_rules! qenum {
    ( $(/$t:tt)?$enum:ident; $( $name:ident$( ($( $wrapped:ty ),*) )? ),* ) => {
        $($t)? enum $enum {
            $( $name$( ($($wrapped),*) )? ),*
        }
    };
    ( [$( $derive:ty ),*] $(/$t:tt)?$enum:ident; $( $name:ident$( ($( $wrapped:ty ),*) )? ),* ) => {
        #[derive($($derive),*)]
        $($t)? enum $enum {
            $( $name$( ($($wrapped),*) )? ),*
        }
    };
}
#[macro_export]
macro_rules! qmod {
    ( $( $p:vis $t:ident ),* ) => {
        $( $p mod $t; )*
    };
}
#[macro_export]
macro_rules! quse {
    ( $( $p:vis $path:path ),* ) => {
        $( $p use $path; )*
    };
}
#[macro_export]
macro_rules! qvar {
    ( $( $name:ident$([$a:ty])? $(:$b:expr)? ),* ) => {
        $( let $name$(:$a)?$(=$b)?;)*
    };
}
#[macro_export]
macro_rules! qstruct {
    ( $v:vis $n:ident$(<$( $wt:ident ),*>)?; $( $fv:vis $fn:ident:$ft:ty ),* ) => {
        $v struct $n$(<$($wt)*>)?  {
            $( $fv $fn:$ft ),*
        }
    };
    //with where
    ( $v:vis $n:ident$(<$( $wt:ident ),*>)?; $( $fv:vis $fn:ident:$ft:ty ),* $( |$( $wht:ident:$whty:tt),* )? ) => {
        $v struct $n$(<$($wt)*>)? where $( $( $wht:$whty),* )? {
            $( $fv $fn:$ft ),*
        }
    };
    ( [ $($de:ty),+ ] $v:vis $n:ident$(<$($wt:ident),*>)?; $( $fv:vis $fn:ident:$ft:ty ),* ) => {
        #[derive( $($de),*)] 
        $v struct $n$(<$($wt)*>)? {
            $( $fv $fn:$ft ),*
        }
    };
    ( $v:vis $n:ident$(<$($wt:ident),*>)?; $( $fv:vis $fn:ident:$ft:ty ),* ) => {
        $v struct $n$(<$($wt)*>)? {
            $( $fv $fn:$ft ),*
        }
    };

}