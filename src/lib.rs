#![warn(missing_docs)]
//! # Struct_gen
//! Struct_gen automagically generates boilerplate code for structs.


/// `struct_gen!` is a macro for generating struct definitions and constructors.
/// 
/// # struct_gen
/// 
/// `struct_gen!` is the macro at the heart of this crate. It is responsible for generating
/// the boilerplate for a struct, from defining the struct to implementing its static
/// constructor method. Ultimately, it is desirable for this macro to be as abstract and as
/// flexible as possible, accepting struct's with:
/// * lifetimes
/// * smart pointers
/// * generics
/// * optional non-default/zero values per field in the constructor
/// * etc.
/// 
/// ## Example
/// ```rust
/// # #[macro_use]
/// # extern crate struct_gen;
/// # use struct_gen::Zero; 
/// # fn main() {
/// struct_gen!(
///     Example {
///         height: i32 
///         size:   f64
///         thing: char
///     }
/// );
/// # let example_struct = Example::new();
/// # assert_eq!(example_struct.height, 0);
/// # assert_eq!(example_struct.size, 0.0);
/// # assert_eq!(example_struct.thing, 0 as char);
/// # }
/// 

#[macro_export]
macro_rules! struct_gen (
    ($s:ident {$( $i: ident : $t: ty)*} ) => (
        #[derive(Debug)]
        struct $s {
            $(
                $i: $t,
            )*
        }
        
       impl $s {
            pub fn new() -> $s {
                $s {
                    $(
                        $i: <$t>::zoor(),
                    )*
                }
            }
       }
    );
);

/// `Zero` is a trait for defining the zoor method,
/// zero-or-override, defining a method that returns
/// the default/zero value for a given type.
/// 
/// # Zero
/// 
/// The `Zero` trait defines a way for a type to
/// return the zero, or default, value of itself.
/// This is used within the `struct_gen!` macro's constructor
/// generation method to construct a base struct type with
/// default values. Ultimately, there will be a way to take
/// an input and override these values, but for now only
/// a default is implemented.
/// 
/// In order for a user to make a custom type compatible
/// with the `struct_gen!` macro, they will need to implement
/// this trait -- done easily with the `impl_zero!` macro.
pub trait Zero {
    /// The Item here will be defined to be the same type as
    /// the trait that is implementing it.
    type Item;
    /// zoor stands for zero or overide
    fn zoor() -> Self::Item;
}

/// `impl_zero!` is a macro for implementing the `Zero` trait in an 
/// ergonomically friendly way.
/// 
/// # impl_zero
/// This macro is used to generate all the base default
/// cases for common/primitive types. It does this by 
/// implementing the `Zero` trait for these types, in an
/// ergonomatically friendly way:
/// ```no-run
/// impl_zero!(TYPE, DEFAULT);
/// ```
/// 
/// ## Example
/// ```no-run
/// impl_zero!(i32, 0);
/// ```
#[macro_export]
macro_rules!  impl_zero {
    ($t: ty, $e: expr) => {
        impl Zero for $t {
            type Item = $t;
            fn zoor() -> Self::Item {
                $e
            }
        }
    };
}


impl_zero!(bool, false);

// Define char as 0 in unicode aka null
impl_zero!(char, 0 as char);

impl_zero!(i8, 0);
impl_zero!(i16, 0);
impl_zero!(i32, 0);
impl_zero!(i64, 0);
impl_zero!(isize, 0);

impl_zero!(u8, 0);
impl_zero!(u16, 0);
impl_zero!(u32, 0);
impl_zero!(u64, 0);
impl_zero!(usize, 0);

impl_zero!(f32, 0.0);
impl_zero!(f64, 0.0);

#[cfg(test)]
mod test_struct_gen {
    use super::*;
    #[test]
    fn it_expands_to_empty_struct() {
        struct_gen!(
            Example {}
        );

        let _e = Example::new();
    }

    #[test]
    fn it_expands_to_multi_field_struct() {
        struct_gen!(
            Example {
                a: i32
                b: f64
                c: bool
            }
        );

        let e = Example::new();
        assert_eq!(e.a, 0);
        assert_eq!(e.b, 0.0);
        assert!(!e.c);
    }

    #[test]
    fn it_works_with_bool() {
        struct_gen!(
            Example {
                a: bool
            }
        );

        let e = Example::new();
        assert!(!e.a);
    }

    #[test]
    fn it_works_with_char() {
        struct_gen!(
            Example {
                a: char
            }
        );

        let e = Example::new();
        assert_eq!(e.a, 0 as char);
    }

    #[test]
    fn it_works_with_i8() {
        struct_gen!(
            Example {
                a: i8
            }
        );

        let e = Example::new();
        assert_eq!(e.a, 0);
    }

    #[test]
    fn it_works_with_i16() {
        struct_gen!(
            Example {
                a: i16
            }
        );

        let e = Example::new();
        assert_eq!(e.a, 0);
    }

    #[test]
    fn it_works_with_i32() {
        struct_gen!(
            Example {
                a: i32
            }
        );

        let e = Example::new();
        assert_eq!(e.a, 0);
    }

    #[test]
    fn it_works_with_i64() {
        struct_gen!(
            Example {
                a: i64
            }
        );

        let e = Example::new();
        assert_eq!(e.a, 0);
    }

    #[test]
    fn it_works_with_isize() {
        struct_gen!(
            Example {
                a: isize
            }
        );

        let e = Example::new();
        assert_eq!(e.a, 0);
    }

        #[test]
    fn it_works_with_u8() {
        struct_gen!(
            Example {
                a: u8
            }
        );

        let e = Example::new();
        assert_eq!(e.a, 0);
    }

    #[test]
    fn it_works_with_u16() {
        struct_gen!(
            Example {
                a: u16
            }
        );

        let e = Example::new();
        assert_eq!(e.a, 0);
    }

    #[test]
    fn it_works_with_u32() {
        struct_gen!(
            Example {
                a: u32
            }
        );

        let e = Example::new();
        assert_eq!(e.a, 0);
    }

    #[test]
    fn it_works_with_u64() {
        struct_gen!(
            Example {
                a: u64
            }
        );

        let e = Example::new();
        assert_eq!(e.a, 0);
    }

    #[test]
    fn it_works_with_usize() {
        struct_gen!(
            Example {
                a: usize
            }
        );

        let e = Example::new();
        assert_eq!(e.a, 0);
    }

    #[test]
    fn it_works_with_f32() {
        struct_gen!(
            Example {
                a: f32
            }
        );

        let e = Example::new();
        assert_eq!(e.a, 0.0);
    }

    #[test]
    fn it_works_with_f64() {
        struct_gen!(
            Example {
                a: f64
            }
        );

        let e = Example::new();
        assert_eq!(e.a, 0.0);
    }
}