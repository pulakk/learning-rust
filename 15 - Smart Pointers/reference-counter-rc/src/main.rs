use std::rc::Rc;

enum List<T: Copy> {
    Cons(T, Rc<List<T>>),
    Nil,
}

struct ConsListIterator<'l, T: Copy> {
    current: &'l List<T>,
}

impl<'l, T: Copy> ConsListIterator<'l, T> {
    fn new(list: &'l List<T>) -> ConsListIterator<'l, T> {
        ConsListIterator { current: list }
    }

    fn print_list(list: &List<T>) -> Vec<T> {
        ConsListIterator::new(list).collect::<Vec<T>>()
    }
}

impl<'l, T: Copy> Iterator for ConsListIterator<'l, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match self.current {
            List::Cons(v, next) => {
                self.current = &next;
                Some(*v)
            },
            List::Nil => {
                None
            }
        }
    }
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Rc::new(Cons(12, Rc::new(Cons(13, Rc::new(Nil)))));

    println!("Reference count: {}", Rc::strong_count(&list)); // Reference count: 1

    let second_list = Cons(1, Rc::clone(&list));
    {
        println!("Reference count: {}", Rc::strong_count(&list)); // Reference count: 2
        let third_list = Cons(2, Rc::clone(&list));

        println!("{:?}", ConsListIterator::print_list(&second_list)); // [1, 12, 13]
        println!("{:?}", ConsListIterator::print_list(&third_list)); // [2, 12, 13]
        println!("Reference count: {}", Rc::strong_count(&list)); // Reference count: 3
    }

    println!("Reference count: {}", Rc::strong_count(&list)); // Reference count: 2

}
