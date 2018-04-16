
// this function (from generic-types) errored at first because not all types can by used with the >
// operation. that's because not all types have an implementation of the trait
// std::cmp::PartialOrd, but we can use trait bounds to avoid passing those types in.
//
// We also need to specify that the type needs Copy.
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// [TODO]: wouldn't need to require Copy if the return type was &T and the body of the function
// was changed to return a reference.  Not sure how to do this tho.

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

}
