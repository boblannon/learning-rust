use std::env;
use std::process;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // main function should be limited to:
    //  (1) Calling the command line parsing logic with the argument values
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    //  (2) Setting up any other configuration
    //  (3) calling a `run` function in lib.rs
    //  (4) handling errors if `run` returns them

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    run(config);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
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

fn run(config: Config) {
    let mut f = File::open(config.filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("With text:\n{}", contents);
}
