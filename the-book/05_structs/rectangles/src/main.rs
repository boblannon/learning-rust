// you can get debug output, but you have to opt in
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    // requires std::fmt::Display trait on Rectangle
    //println!("rect1 is {}", rect1);

    // basic debug printout, but must opt in with #[derive(Debug)]
    println!("rect1 is {:?}", rect1);

    // pretty-printed debug output, with newlines and indentation
    println!("rect1 is {:#?}", rect1);

    println!(
        "The area of the rect is {} square pixels",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
