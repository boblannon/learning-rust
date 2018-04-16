fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// this function only differs in param and return type. internal code is identical
fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// generic T
// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         // this errors at first because not all types can by used with the > operation. that's
//         // because not all types have an implementation of the trait std::cmp::PartialOrd
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// require that two fields in a struct be of the same type
struct Point<T> {
    x: T,
    y: T,
}

// allow type of fields to be different
struct PointTwo<T, U> {
    x: T,
    y: U,
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    let both_integer = Point{ x: 5, y: 10 };
    let both_float = Point{ x: 1.0, y: 4.0 };
    // This won't work, since int and float
    // let integer_and_float = Point { x: 5, y: 4.0 }; // err: mismatched types

    // these will all work (okay if T and U happen to be the same type
    let both_integer = PointTwo { x: 5, y: 10 };
    let both_float = PointTwo { x: 1.0, y: 4.0 };
    let integer_and_float = PointTwo { x: 5, y: 4.0 };

}

