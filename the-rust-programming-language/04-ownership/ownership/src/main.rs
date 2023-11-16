// data on the heap can live indefinitely
// stack only holds data associated with a specific fn and are deallocated when the fn returns
// rust does not allow for the manual deallocation of memory
// heap data can only be accessed through its current owner
// Vec, String and HashMap are boxes
// rust deallocates heap data once its owner goes out of scope.

// Box Deallocation Principle: If a variable owns a box, when Rust deallocates the variable's frame, then Rust deallocates the box's heap memory
// Moved Heap Data Principle: if a variable x moves ownership of heap data to another variable y, then x cannot be used after the move

fn main() {

    let mut num = 8;
    let num_ref = &mut num;
    *num_ref += 10;

    println!("{}", *num_ref);
}

fn heap() {
    // use Box::new to put data in the heap
    let a = Box::new([0; 1_000_000]);
    let b = a;
}

fn clone() {
    let first = String::from("Ferris");
    // avoid moving data by using clone()
    let first_clone = first.clone();
    let full = add_suffix(first_clone);
    println!("{full}, originally {first}");
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}