#[derive(Debug)]
struct Adder {}

impl Adder {
    fn add_two() -> i32 {
        4
    }
}

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, Adder::add_two());
}
