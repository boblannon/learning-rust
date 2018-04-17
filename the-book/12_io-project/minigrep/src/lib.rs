use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;


pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
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
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        // variables and fields have the same name, so shortcut to assignment works here
        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
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

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // to_lowercase creates new data, which is a String (not a slice), so we need to add an
    // ampersand when passing query to contains below (which is defined to take &str).
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
