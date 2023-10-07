// mut makes the variable mutable
// const are always immutable + can be used in any scope
// let can only be used in function scope
// we are able to change type with shadowing unlike mut
// signed integers (i) are able to be negative

use std::io;

fn shadow() {
    let x = 5;
    // shadowing
    let x = x + 1;

    {
        // shadowing within inner scope
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

fn tup() {
    // elements in tuples can have different type
    let x: (i32, f64, u8) = (500, 6.4, 1);

    // we can also access a tuple element directly by using a period (.)
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
}

fn main() {
    // elements in arrays have to be the same type
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}