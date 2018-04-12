#[derive(Debug)]
enum Message {
    Quit, // no data
    Move { x: i32, y: i32 }, // anonymous struct
    Write(String), // includes string
    ChangeColor(i32, i32, i32), // includes three i32 values
}

// similar, we could do this with structs
struct QuitMessage; // unit struct
struct MoveMessage{
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

// but it would be harder to make a function that could take any of these kinds
// of structs, since they're all of different types. with the Message enum, we
// get one type, and can define functions that take it as an arg.

impl Message {
    fn call(&self) {
        // method body here
        println!("message: {:#?}", self);
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
}
