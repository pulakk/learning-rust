use super::rectangle::*;

#[test]
fn larger_can_hold_smaller() {
    let larger = Rectangle { width: 2, height: 3 };
    let smaller = Rectangle { width: 1, height: 3 };
    assert!(larger.can_hold(&smaller));
}

#[test]
fn smaller_can_hold_larger() {
    let larger = Rectangle { width: 2, height: 3 };
    let smaller = Rectangle { width: 1, height: 3 };
    assert!(!smaller.can_hold(&larger));
}