fn main() {
    let mut s = String::new();

    let data = "initial contents";

    // any type that implements the Display trait has to_string
    let s = data.to_string();

    // could also work on literal directly
    let s = "initial contents".to_string();

    // could also use String::from
    let s = String::from("initial contents");

    println!("{}", s); // initial contents

    // utf-8 encoded
    let hello = String::from("नमस्ते");
    println!("{}", hello); // नमस्ते

    // update by pushing more data in
    let mut s = String::from("foo");
    s.push_str("bar");

    println!("{}", s); // foobar

    // push_str takes a slice because we don't necessarily want to take
    // ownership, eg:
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(&s2);
    println!("s2 is {}", s2); // s2 is bar

    // add character with push
    let mut s = String::from("lo");
    s.push('l');
    println!("{}", s); // lol

    // concat with +
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 is moved and can't be used
    println!("{}", s3);

    // concat with format
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // this is unwieldy and takes ownership of s1:
    //     let s = s1 + "-" + &s2 + "-" + &s3;
    // but this is easier to read and doesn't take ownership of anything:
    let s = format!("{}-{}-{}", s1, s2, s3);

    // internal rep is Vec<u8>
    let len = String::from("Hola").len(); // 4
    println!("Hola has length {}", len);

    let hello = String::from("Здравствуйте");
    println!("{} has length {}", hello, hello.len()); // 24, not 12

    // नमस्ते in Devanagari is stored as Vecu8>, and looks like:
    let namaste_vec = [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165,
                       141, 224, 164, 164, 224, 165, 135];
    // as unicode scalar values (which is what `char` is)
    let namaste_char = ['न', 'म', 'स', '्', 'त', 'े'];

    // grapheme clusters
    let namaste_grapheme = ["न", "म", "स्", "ते"];

    // slicing is okay ...if you slice at char boundaries
    let s = &hello[0..4];
    println!("The first four bytes of {} are \"{}\"", hello, s);

    // these slices will cause panic

    //let s = &hello[0..1];
    // byte index 1 is not a char boundary; it is inside 'З'
    //let s = &hello[0..3];
    // byte index 3 is not a char boundary; it is inside 'д'

    // iterating: chars
    let namaste = "नमस्ते";
    for c in namaste.chars() {
        println!("{}", c);
    }

    for b in namaste.bytes() {
        println!("{}", b);
    }

}
