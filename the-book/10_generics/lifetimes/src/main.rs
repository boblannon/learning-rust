use std::fmt::Display;

//// can't use uninitialized
// fn print_r() {
//     let r;
//     println!("r: {}", r);
//     {
//         let x = 5;
//         r = &x;
//     }

// }

//// can't use out-of-scope
// fn print_r() {
//     let r;                // -------+-- 'a (lifetime of r)
//                           //        |
//     {                     //        |
//         let x = 5;        // -+-----+-- 'b (lifetime of x)
//         r = &x;           //  |     |
//     }                     // -+     |
//                           //        |
//     println!("r: {}", r); //        |
// }                         // -------+


fn print_r() {
    let x = 5;            // -----+-- 'b (lifetime of x)
                          //      |
    let r = &x;           // --+--+-- 'a (lifetime of r)
                          //   |  |
    println!("r: {}", r); //   |  |
                          // --+  |
}                         // -----+

//// this won't compile: missing lifetime specifier on return value. we don't know what the lifetimes
//// of the arguments will be, and we also don't know which argument we'll return.
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

//// lifetime annotations: all must have same lifetime
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

//// if we never return y, it doesn't need lifetime annotation
//fn longest<'a>(x: &'a str, y: &str) -> &'a str {
//    x
//}

//// no good, result's lifetime has no relationship to either param's lifetime. it will go out of
//// scope at the end of the fct block.
//fn longest<'a>(x: &str, y: &str) -> &'a str {
//    let result = String::from("really long string");
//    result.as_str()
//}


// lifetime annotation in struct defs
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {

    // no lifetime issues here because return is not a ref
    fn level(&self) -> i32 {
        3
    }

    // rule three: first param is &self, so output lifetime is same as self ('a)
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// combining everything
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
	print_r();


    //// this is fine
    // let string1 = String::from("abcd");
    // let string2 = String::from("xyz");
    // let result = longest(string1.as_str(), string2.as_str());
    // println!("The longest string is {}", result);


    //// also fine
    let string1 = String::from("abcd");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    //// not fine: string2 doesn't live long enough
    // let string1 = String::from("abcd");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result); // string2 needs to live as long as result does.

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    // instance of ImportantExcerpt that holds a reference to the first sentence of the String
    // owned by novel
    let i = ImportantExcerpt { part: first_sentence };
    println!("{:?}", i);

    // 'static lifetime: the entire duration of the program. all string literals have 'static
    let s = "I have a static lifetime.";
    // more verbosely:
    let s: &'static str = "I have a static lifetime.";

    let string1 = String::from("abcd");
    let string2 = String::from("xyz");
    let result = longest_with_an_announcement(string1.as_str(), string2.as_str(),
                    "ladies and gentlemen and friends beyond the binary...");
    println!("The longest string is {}", result);
}
