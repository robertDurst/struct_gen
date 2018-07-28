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

impl_zero!(i8, 0);
impl_zero!(i16, 0);
impl_zero!(i32, 0);
impl_zero!(i64, 0);

impl_zero!(u8, 0);
impl_zero!(u16, 0);
impl_zero!(u32, 0);
impl_zero!(u64, 0);

impl_zero!(f32, 0.0);
impl_zero!(f64, 0.0);

impl_zero!(String, String::from(""));
