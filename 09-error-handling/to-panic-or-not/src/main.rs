// returning Result is a good default choice
// if it is logically impossible for Err, you van use unwrap

use std::net::IpAddr;

fn ip_address() {
    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
}

// define a Guess type for the guessing game so that values are in between 1 and 100
pub struct Guess {
    value: i32, // value is private so code isn't able to set the value directly
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    // getter since value of Guess struct is private
    pub fn value(&self) -> i32 {
        self.value
    }
}