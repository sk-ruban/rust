// statements are instructions that perform some action and do not return a value.
// expressions evaluate to a resultant value.
// semicolons turn expressions into a statement

fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

// position of functions does not matter
// snake case for function and variable names
// type of each parameter MUST be declared
fn plus_one(x: i32) -> i32 {
    x + 1
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn five() -> i32 {
    5
}