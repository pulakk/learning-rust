use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(v: T) -> MyBox<T> {
        MyBox(v)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn do_something(word: &str) {
    println!("I did something with word: {}", word);
}

fn main() {
    let b = MyBox::new(12);

    assert_eq!(12, *b); /* no errors */

    let x = MyBox::new(String::from("Water Bottle"));
    do_something(&x);
}
