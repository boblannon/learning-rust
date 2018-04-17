use std::error::Error;
use std::fs::File;
use std::io::prelude::*;


pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // clone is a perf hit, but here it's not too big, and it simiplifies our code to have the
        // Config struct own the values, so that we don't need to manage lifetimes, scope, etc.
        let query = args[1].clone();
        let filename = args[2].clone();

        // variables and fields have the same name, so shortcut to assignment works here
        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    println!("With text:\n{}", contents);

    Ok(())
}
