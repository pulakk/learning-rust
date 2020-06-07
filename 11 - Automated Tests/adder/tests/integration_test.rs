use adder;

mod common;

#[test]
fn add_two() {
    common::setup();
    assert_eq!(5, adder::add::sum(2, 3));
}