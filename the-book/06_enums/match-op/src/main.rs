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

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        // each arm is pattern => execute
        // use brackets for multi-line
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn main() {
    println!("Your {:?} is worth {} cents", Coin::Penny, value_in_cents(Coin::Penny));
    println!("Your {:?} is worth {} cents", Coin::Quarter(UsState::Alaska), value_in_cents(Coin::Quarter(UsState::Alaska)));
}
