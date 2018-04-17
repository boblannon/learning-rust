use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // main function should be limited to:
    //  (1) Calling the command line parsing logic with the argument values
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    //  (2) Setting up any other configuration
    //  (3) calling a `run` function in lib.rs
    //  (4) handling errors if `run` returns them

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // file io
    let mut f = File::open(config.filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    // clone is a perf hit, but here it's not too big, and it simiplifies our code to have the
    // Config struct own the values, so that we don't need to manage lifetimes, scope, etc.
    let query = args[1].clone();
    let filename = args[2].clone();

    // variables and fields have the same name, so shortcut to assignment works here
    Config { query, filename }
}
