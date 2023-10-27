use std::env;
use std::fs; // to handle files

fn main() {
    // read any CL arguments and collect into a vector
    let args: Vec<String> = env::args().collect();

    // dbg!(args); // print vector

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}