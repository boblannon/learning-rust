

fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");

    // must specify type, because compiler can't infer
    let absent_number: Option<i32> = None;

    // T and Option<T> are different types
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // compiler won't let us use Option<T> as if it was definitely a valid
    // value. this code won't compile bc compiler doesn't know how to add
    // i8 and Option<i8> types.
    let sum = x + y;

    // this is Good(tm) because you need to convert Option<T> to T before
    // doing anything with it. that means you avoid a common issue: assuming
    // something isn't null when it turns out to be.
}
