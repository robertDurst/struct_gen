#[macro_use]
extern crate struct_gen;

fn main() {
    struct_gen!(
        Example {
                data: String > "String".to_string()
                height: i32 > 10
        }
    );

    impl Example {
        fn exists(&self) {
            println!("I exist! My height is: {}", self.height);
        }
    }

    let e = Example::new();
    e.exists();
}
