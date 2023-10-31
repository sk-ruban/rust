// Boxes allows for data storage in the heap rather than the stack
// Box<T> values are treated like references
// pointers take up 8 bytes on a 64-bit architecture

fn box_example() {
    let b = Box::new(5);
    println!("b = {}", b);
} // box gets deallocated

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}