use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        // variables in rust are immutable by default
        // to make them mutable, we add mut before the variable name
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) // & indicates the argument is a reference
            .expect("Failed to read line"); // if Result returns Err

        // trim() eliminates newline \n
        // parse() converts type to u32
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // continue to next iteration of the loop
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}