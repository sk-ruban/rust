// result enum is defined as follows:
// T is the type of value to be returned, E the type of error
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

use std::fs;
use std::io;
use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn result_match() {
    let greeting_file_result = File::open("hello.txt"); // File::open is a Result<T, E>

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}

// unwrap helps to return the value or call the panic! macro
fn result_unwrap() {
    let greeting_file = File::open("hello.txt").unwrap();
}

// expect lets us choose the panic! error message
fn result_expect() {
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}

// propagating errors up to the calling code
// return type is Result<String, io::Error> where T is a String and E is io::Error
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e), // return keyword to return early
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e), // no return keyword as it is the last expression
    }
}

// ? Operator
// ? can only be used if the return type of the function is compatible with the value ? is used on
// cannot be used in main() since the return type is ()
fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// even more succinct
fn read_username_from_file_3() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// changing main's return type
fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}