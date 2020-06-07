pub fn sum(a: i32, b: i32) -> i32 {
    if a == b {
        panic!("a and b have to be different");
    }
    a + b
}