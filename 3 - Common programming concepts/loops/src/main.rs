fn returnable_loop() {
    let mut i = 0;
    let sum = loop { i += 1; if i == 5 { break i; } };

    println!("{}", sum);
}

fn while_loop() {
    println!("WHILE LOOP");
    let mut i = 10;
    while i > 0 {
        println!("i = {}", i);
        i -= 1;

        if i == 5 { break; }
    }
}

fn for_loop() {
    println!("FOR LOOP");
    let mut a = [0, 2, 3, 4, 5];
    a[0] = 1;

    for i in a.iter() { println!("{}", i); }

    /* both iteration and borrowing changes the array */
    println!("Borrow the array");
    for i in &mut a {
        *i = 2;
        println!("{}", i);
    }

    println!("Iterate over it and mutate");
    for i in a.iter_mut() {
        *i = 3;
        println!("{}", i);
    }
}

fn for_range() {
    println!("Ranges");
    for number in (1..4).rev() {
        println!("{}", number);
    }
}

fn main() {
    returnable_loop();
    while_loop();
    for_loop();
    for_range();
}
