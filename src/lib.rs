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

pub trait Zero {
    type Item;
    /// zoor stands for zero or overide
    fn zoor() -> Self::Item;
}

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
