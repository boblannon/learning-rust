use std::net::IpAddr;

// when to use panic!
//  - examples
//  - prototype code
//  - tests (panic may be how a test is marked as failure)
//  - cases where you have more info than the compiler. eg, hardcoded input
//    that you can be sure will not raise errors
fn hardcoded_input() -> IpAddr {
    let home: IpAddr = "127.0.0.1".parse().unwrap();
    return home
}

// invalid arugment types are a good reason to use panic! but constantly having checks for types
// can be verbose and annoying. use rust's type system to do the work for you!
//  - functions should have typed params
//  - custom types

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }

    // this is a getter function. `value` is private, so that it's not possible to assign a value
    // to it after having called new(). If that was possible, then a user could bypass the test in
    // the new() function that guarantees value is within range
    pub fn value(&self) -> u32 {
        self.value
    }
}


fn main() {
    let home = hardcoded_input();
    println!("home is {}", home);

    let good_guess = Guess::new(50u32);
    // this will error because value is private and immutable:
    // good_guess.value = 75;
    println!("guess is {}", good_guess.value());

    // this will panic because it will fail test in new
    let bad_guess = Guess::new(350u32);
}
