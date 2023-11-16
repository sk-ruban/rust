fn main() {
    type Kilometers = i32; // Kilometers is a synonym for i32, not a new type

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);
    // x + y = 10

    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
    // The answer is: 12
}

// ! is a never type, used when a function will never return
// fn bar() -> ! {
//     // --snip--
// }

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
    // needs Box as rust does not know how much space is needed to store the closure
}