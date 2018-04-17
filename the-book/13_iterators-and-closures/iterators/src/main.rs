fn main() {
    let v1 = vec![1, 2, 3];

    // iterators are lazy. they're not evaluated until they're consumed
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    // can't use here because loop took ownership (and made it mutable)
    // println!("Iterator is {:?}", v1_iter);

    // this works because iter() returns immutable references
    println!("vec is {:?}", v1);
    // into_iter() - take ownership of v1 and return owned values
    // iter_mut() - iterate over mutable references

    // map is an iterator adaptor: changes iterators into different kinds of iterators
    // - chainable
    // - lazy: at some point we need to call a consuming adaptor to get results
    let v2: Vec<i32> = vec![1,2,3];
    // v2.iter().map(|x| x + 1); // warning: unused
    let v3: Vec<_> = v2.iter().map(|x| x + 1).collect(); // collect consumes
    assert_eq!(v3, vec![2, 3, 4]);
}
