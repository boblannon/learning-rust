use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);

    // can also construct using collect
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // _ for type tells compiler to infer
    let final_scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", final_scores);

    // hash map becomes owner of owned values like String
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name & field_value are invalid now, so "use of moved value err" for:
    //println!("{}: {}", field_name, field_value);

    // not true for types like i32, which implement the Copy trait
    let field_name = String::from("Favorite number");
    let field_value = 42i32;
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("{}", field_value);

    // access value with get
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("{:?}", score);

    // iterating
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // updating
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // only insert if key has no value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    // update value based on old value (like defaultdict)
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // or_insert returns a murable referens to the value
        let count = map.entry(word).or_insert(0);
        // dereference the value and change
        *count +=1;
    }

    println!("{:?}", map);

    // hashing function is cryptographically secure by default, but can be
    // changed for speed. need to specify a different hasher. A hasher is a
    // type that implements the BuildHasher trait

}
