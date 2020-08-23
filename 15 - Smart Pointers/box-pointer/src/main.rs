enum UsualPointerList<'a> {
    PCons(i32, &'a UsualPointerList<'a>),
    PNil,
}

enum BoxList {
    Cons(i32, Box<BoxList>),
    Nil,
}

use UsualPointerList::{PCons, PNil};
use BoxList::{Cons, Nil};


fn main() {
    /* allocates on heap instead of stack */
    let word = Box::new("water");
    println!("word: {}", word);

    /* Recursive types */
    let list = PCons(1, &PCons(2, &PCons(3, &PNil)));
    if let PCons(v, _) = list {
        println!("{}", v);
    }
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
