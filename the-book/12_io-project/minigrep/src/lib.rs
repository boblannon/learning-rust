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

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    // iterate through each line of the contents
    for line in contents.lines() { // lines method returns iterator
        // check whether the line contains query
        if line.contains(query) {
            // if it does, add it to the list of values we're returning
            results.push(line);
        }
        // if not, do nothing
    }

    // return list of results that match
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
}
