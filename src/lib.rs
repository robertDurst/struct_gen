#[macro_export]
macro_rules! struct_gen (
    ($s:ident {$( $y: ident : $x: ty | $v: expr)*} ) => (
        struct $s {
            $(
                $y: $x,
            )*
        }
        
       impl $s {
            pub fn new() -> $s {
                $s {
                    $(
                        $y: $v,
                    )*
                }
            }
       }
    );
);