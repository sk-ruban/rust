use std::env;
use std::process;

use minigrep::Config;

// main.rs handles running the program
// lib.rs handles all task logic
fn main() {
    // read any CL arguments and collect into a vector
    let args: Vec<String> = env::args().collect();

    // dbg!(args); // print vector

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1); // will stop the function immediately
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}