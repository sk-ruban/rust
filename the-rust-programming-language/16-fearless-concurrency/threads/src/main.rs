use std::thread;
use std::time::Duration;

fn handle() {
    let handle = thread::spawn(|| {
        for i in 1..10 { // spawned threads are shut down after main thread completes
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap(); // main thread will wait for spawned thread to finish

    // main thread
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1)); // stop execution so different thread can run
    }
}

// with handle
// hi number 1 from the spawned thread!
// hi number 2 from the spawned thread!
// hi number 3 from the spawned thread!
// hi number 4 from the spawned thread!
// hi number 5 from the spawned thread!
// hi number 6 from the spawned thread!
// hi number 7 from the spawned thread!
// hi number 8 from the spawned thread!
// hi number 9 from the spawned thread!
// hi number 1 from the main thread!
// hi number 2 from the main thread!
// hi number 3 from the main thread!
// hi number 4 from the main thread!

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || { // with move, closure takes ownership of the values
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}