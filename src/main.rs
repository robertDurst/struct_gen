#[macro_use]
extern crate struct_gen;

// Always have to bring traits into scope to use them.
// A bit weird here since the trait is used/implemented
// within the struct_gen macro.
use struct_gen::Zero; 

fn main() {
    struct_gen!(
        Example {
            height: i32 
            size:   f64
            word:   String
        }
    );

    let example_struct = Example::new();
    println!("{:#?}", example_struct);
}
