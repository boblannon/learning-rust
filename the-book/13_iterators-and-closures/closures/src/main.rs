use std::thread;
use std::time::Duration;

//// closures can look similar to functions, but don't have to:
// fn  add_one_v1   (x: u32) -> u32 { x + 1 }
// let add_one_v2 = |x: u32| -> u32 { x + 1 };
// let add_one_v3 = |x|             { x + 1 };
// let add_one_v4 = |x|               x + 1  ;

//// closure type inference is done once for the first time called.
// let example_closure = |x| x;
//
// let s = example_closure(String::from("hello"));
// let n = example_closure(5); // error: mismatched types

//// Memoization (or Lazy Evaluation)
// every closure instance has its own unique anonymous type, so we need to use a generic T. All
// closures implement one of the Fn traits, though, so we can set a trait bound.
struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );

    // let simulated_user_specified_value = 26;

    // generate_workout(
    //     simulated_user_specified_value,
    //     simulated_random_number
    // );

    // let simulated_random_number = 3;

    // generate_workout(
    //     simulated_user_specified_value,
    //     simulated_random_number
    // );

    // closures can also capture the local environment
    let x = 4;

    let equal_to_x = |z| z == x;

    // won't work, x is not in scope
    // fn equal_to_x(z: i32) -> bool { z == x }

    let y = 4;

    assert!(equal_to_x(y));

    //// force closure to take ownership of values with `move`
    // let x = vec![1, 2, 3];
    // let equal_to_x = move |z| z == x;
    // println!("can't use x here: {:?}", x); // use of moved value: `x`
    // let y = vec![1, 2, 3];
    // assert!(equal_to_x(y));

}

#[test]
fn call_with_different_values() {
    // fails because Cacher assumes it will always be called with the same arg.
    // [TODO]: modify Cacher so that it has a hashmap to look up values
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}

#[test]
fn call_with_other_param_types() {
    // fails because Cacher can only accept closures that take/return u32
    // [TODO]: modify with generic params to make it more flexible
    let mut c = Cacher::new(|a| a);

    let v = c.value(15.0f64);
}
