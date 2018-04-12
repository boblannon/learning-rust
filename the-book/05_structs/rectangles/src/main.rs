// you can get debug output, but you have to opt in
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        // don't need to specify type, which is inferred from struct itself
        // do need to use a reference &
        //   - methods can take ownership, just like anywhere else
        //   - usu want to take ownership if method changes the struct and you
        //     want to avoid using the original instance after that change
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    // requires std::fmt::Display trait on Rectangle
    //println!("rect1 is {}", rect1);

    // basic debug printout, but must opt in with #[derive(Debug)]
    println!("rect1 is {:?}", rect1);

    // pretty-printed debug output, with newlines and indentation
    println!("rect1 is {:#?}", rect1);

    println!(
        "The area of the rect is {} square pixels",
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
