use std::env;

fn double_except(list: &Vec<u16>, v: u16) -> Vec<u16> {
    list.iter().map(|x| if *x == v { *x } else { x * 2 }).collect()
}

struct IteratorX{
    count: u16,
}

impl IteratorX {
    fn new() -> IteratorX {
        IteratorX {
            count: 0
        }
    }
}

impl Iterator for IteratorX {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 { 
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn analyse_args(mut args: std::env::Args) {
    args.next();

    match args.next() {
        Some(arg) => println!("First argument: {}", arg),
        None => println!("First argument: null"),
    }

    match args.next() {
        Some(arg) => println!("Second argument: {}", arg),
        None => println!("Second argument: null"),
    }
}

fn main() {
    analyse_args(env::args());

    /* iterators */

    let list = vec![5,6,3];
    let iter = list.iter();

    /* consumes the iter */
    for item in iter {
        print!("{} ", item);
    }
    println!();

    /* consumes the iter */
    println!("Sum: {}", list.iter().sum::<u16>());

    let iter = list.iter();
    /* consumes the iter */
    for item in iter.map(|x| x*2) {
        print!("{} ", item);
    }
    println!();

    /* Collect iterators to collections */
    let double_list: Vec<_> = list.iter().map(|x| x * 2).collect();
    println!("Double: {:?}", double_list);

    println!("Double: {:?}", double_except(&list, list[1])); /* doesn't take ownership of list */
    println!("Original: {:?}", list);

    /* Custom iterator */
    let iter = IteratorX::new();

    for item in iter {
        print!("{} ", item);
    }
    println!();

    /* zip */
    let zipped: Vec<u16> = IteratorX::new().zip(IteratorX::new().skip(1)).map(|(x, y)| x + y).collect();
    println!("{:?}", zipped);
}
