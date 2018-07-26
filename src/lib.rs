#[macro_export]
macro_rules! struct_gen (
    ($s:ident) => {
       impl $s {
            pub fn new() -> $s {
                println!("Making a new struct of type: {}", stringify!($s));
                $s {}
            }
       }
    };
);
