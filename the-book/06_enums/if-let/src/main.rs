#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ...etc
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // alternatively, using if-let:
    if let Some(3) = some_u8_value {
        println!("three");
    }

    // i think this is ugly and not good
    // note: we lose exhaustive checking

    // can use else
    let coin = Coin::Quarter(UsState::Alaska);

    let mut count = 0;

    //// match version:
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    //     _ => count += 1,
    // }

    // if-let-else version
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
