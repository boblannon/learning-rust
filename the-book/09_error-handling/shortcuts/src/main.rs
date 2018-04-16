use std::fs::File;

fn main() {
    // unwrap is a shortcut for "if err, panic! else return value inside ok"
    // let f = File::open("hello.txt").unwrap();

    // expect is similar, but also lets us choose the panic! error message
    let f = File::open("hello.txt").expect("Failed to open hello.txt")
}
