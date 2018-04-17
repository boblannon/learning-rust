extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // main function should be limited to:
    //  (1) Calling the command line parsing logic with the argument values
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    //  (2) Setting up any other configuration
    //  (3) calling a `run` function in lib.rs
    //  (4) handling errors if `run` returns them
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    };

}

