use std::error::Error;
use std::fs; // to handle files

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    // error values will always be string literals with 'static lifetimes
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

// dyn removes the need to specify what particular type the error will be
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}