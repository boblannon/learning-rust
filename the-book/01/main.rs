// define a function, no params, no return
fn main() {
    // println is a macro
    println!("Hello, world!");
    // why?
    // - string arg can have formatting specifiers
    //      - checked at compile-time
    //      - can have named args (rust fcts can't)
    // - can take variable number of args
    // - implicitly takes args by ref (even when passed by value)
}
