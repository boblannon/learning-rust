extern crate adder;

// putting shared util functions into common/mod.rs keeps them from being treated as a test
mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
