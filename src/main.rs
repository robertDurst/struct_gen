#[macro_use]
extern crate struct_gen;

fn main() {
    struct Example {}

    struct_gen!(Example);

    impl Example {
        fn exists(&self) {
            println!("I exist!");
        }
    }

    let e = Example::new();
    e.exists();
}
