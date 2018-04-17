extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // main function should be limited to:
    //  (1) Calling the command line parsing logic with the argument values
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    //  (2) Setting up any other configuration
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    //  (3) calling a `run` function in lib.rs
    //  (4) handling errors if `run` returns them
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    };

}

