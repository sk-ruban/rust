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
        process::exit(1); // will stop the function immediately
    });

    if let Err(e) = minigrep::run(config) {
        process::exit(1);
    }
}