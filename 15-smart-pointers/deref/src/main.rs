fn test_reference() {
    let x = 5;
    let y = MyBox::new(x); // reference pointing to a copied value of x

    assert_eq!(5, x);
    assert_eq!(5, *y);  // *y = *(y.deref())
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> { // Deref trait borrows self and returns a reference to inner data
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Deref coercion converts a reference to a type into a reference to another type
fn hello(name: &str) { // Deref converts &String to &str
    println!("Hello, {name}!");
}

fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}