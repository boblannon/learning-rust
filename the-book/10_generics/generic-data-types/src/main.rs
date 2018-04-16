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

// must declare T just after impl. this tells compiler that the T in Point<T> is a generic type
// rather than a concrete type.
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// this would only be implemented on Points where the type is f32
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// allow type of fields to be different
struct PointTwo<T, U> {
    x: T,
    y: U,
}

// might want to use different generic types in method sigs
impl<T, U> PointTwo<T, U> {
    fn mixup<V, W>(self, other: PointTwo<V, W>) -> PointTwo<T, W> {
        PointTwo {
            x: self.x,
            y: other.y,
        }
    }
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

    // mixing up types
    let p1 = PointTwo { x: 5, y: 10.4}; // T is i32, U is f32
    let p2 = PointTwo { x: "Hello", y: 'c' }; // V is str, W is char
    let p3 = p1.mixup(p2); // output will be <T, W> (i32 and char)

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y );
}

// Important: performance isn't affected by using generics. compiler will do _monomorphization_ of
// code using generics at compile time. It looks at how functions like mixup actually get called by
// the source and compiles separate versions of it for those cases.
