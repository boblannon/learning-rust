use std::io;
// io from standard lib, not included in prelude

fn main() {
    println!("Guess the number!");

    println!("Please input your guess");

    let mut guess = String::new();
    // variables immutable by default. mut makes mutable
    // String is in std. it's growable, UTF-8 text
    // ::new means new is an associated fct of String
    //   - associated fct is implemented on a type
    //   - like static methods in other langs

    io::stdin().read_line(&mut guess)
        .expect("failed to read line");
    // could have been std::io::stdin if we hadn't included io
    // stdin returns instance of io::Stdin, type that is a handle for
    //    stdin of terminal
    // read_line
    //   - method on Stdin to get input from user
    //   - job: get input and place it into a string
    //   - guess is an arg because that's where input will be placed
    //   - returns: an io::Result
    // &mut guess
    //   - & means the arg is a reference (see chap 4 for more)
    //   - references are immutable by defualt, mut makes this mutable
    // io::Result
    //   - there are lots of Result types for different submodules
    //   - Result types are enumerations (enums).
    //     - enums can have a fixed set of values (its _variants_)
    //     - (see chap 6 for more)
    //   - variants of `Result`:
    //     - `Ok` successfully generated value
    //     - `Err` operation failed. contains information about how or why
    //   - purpose is to encode error handling
    // `io::Result` has `expect`
    //   - if instance is an `Err` value: crash program and display message
    //     passed to `expect`
    //   - if instance is an `Ok` value: return value that `Ok` is holding
    //   - in this case, the number of bytes that the user entered into stdin
    //   - if we didn't call this, we'd get a warning about possibly not catching
    //     an error. the Right Way is to use proper error handling, but this is
    //     the basic way (program crashes when there's a problem)

    println!("You guessed: {}", guess);
    // prints out the string that user input
    // `{}` is for interpolation
    //   - you can use multiple sets of brackets
    //   - second set uses third arg, and so on
}
