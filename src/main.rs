#[macro_use]
extern crate struct_gen;

fn main() {
    struct_gen!(
        Example {
                data: String | "Something".to_string()
                height: i32 | 10
        }
    );

    impl Example {
        fn exists(&self) {
            println!("I exist! My height is: {} and data is: {}", self.height, self.data);
        }
    }

    let e = Example::new();
    e.exists();
}
