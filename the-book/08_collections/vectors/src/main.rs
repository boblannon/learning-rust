fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("third is {}", third);

    let third: Option<&i32> = v.get(2);
    println!("third is {:?}", third);

    // get() handles index errors. eg, this causes panic!:
    //let hundo: &i32 = &v[100];

    // but this doesn't
    let hundo: Option<&i32> = v.get(100);
    println!("hundo is {:?}", hundo);

    // can't have mut and immut refs in same scope
    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6);

    // you can access immutable references to elements in a loop
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    // to write to mutable references you need to deref first
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    println!("{:?}", v);

    // use enum to store multiple types
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:?}", row);
}
