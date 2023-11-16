// switch to unsafe rust using the unsafe keyword

fn main() {
    // raw pointers
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // calling an unsafe function or method
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

// interact with foreign languages
extern "C" { // C standard library
fn abs(input: i32) -> i32;
}

// static is equivalent to global variables
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}