// & operator creates a reference (non-owning pointer) to the variable rather than the heap directly
// Pointer Safety Principle: data should never be aliased and mutated at the same time
// creating a reference will transfer permissions from the borrowed path to the reference
// permissions are returned once the reference's lifetime has ended

fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    // v loses ALL permissions since num is a mutable reference
    let num: &mut i32 = &mut v[2];
    *num += 1;
    println!("Third element is {}", *num);
    println!("Vector is now {:?}", v);
}

fn greet(g1: &String, g2: &String) { // note the ampersands
    println!("{} {}!", g1, g2);
}

fn mutation() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &i32 = &v[2];
    // push creates a different memory location
    // num is now not pointing at anything
    v.push(4);
    println!("Third element is {}", *num);
}

fn borrow_checker() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &i32 = &v[2];
    // v has lent its write permissions to num so unable to push
    v.push(4);
    println!("Third element is {}", *num);
}