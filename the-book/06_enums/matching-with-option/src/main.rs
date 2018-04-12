fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        // must include arms for None and Some, or compiler error.
        None => None,
        Some(i) => Some(i + 1),

    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // _ is a placeholder and matches anything not specified
    let some_u8_value = 3u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}
