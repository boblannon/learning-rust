use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f{
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn shortcut() -> Result<String, io::Error> {
    // ? means you return the value inside Ok, or, if there's an error, return
    // the Err as the result of shortcut()
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    //read_username_from_file();
    let output = shortcut();
    let output = match output {
        Ok(file) => file,
        Err(e) => panic!("Oops! {:?}", e),
    };
}
