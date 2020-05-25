fn iterate_over_vec() {
    let mut v = vec![1, 2, 3, 4];

    /* immutable borrows */
    for i in v.iter() {
        print!("{}, ", i);
    }
    println!();

    for i in &v {
        print!("{}, ", i);
    }
    println!();

    /* mutable borrows */
    for i in v.iter_mut() {
        *i += 50;
    }

    for i in &mut v {
        *i += 50;
    }

    println!("{:?}", v);
}

fn vec_indexing() {
    let mut v = Vec::new();
    let v2 = vec![4, 5, 6];

    for i in 1..6 {
        v.push(i);
    }

    println!("{:?}", v);
    println!("{:?}", v2);

    let value = v[2]; // makes a copy, doesn't affect the array
    // let value = &v[2] // borrows it, can change the array's values
    v[2] = 5; // doesn't change `value` as it is a copy
    println!("Third element = {}", value);

    /* `get` returns an option */
    match (vec![1,2]).get(2) {
        Some(third_elem) => println!("Third element \t={}", third_elem),
        None => println!("Third element doesn't exist in given array"),
    }
}

fn multi_type_vec() {
    enum Value {
        Int(i32),
        Text(String),
    };

    impl std::fmt::Debug for Value {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match &self {
                Value::Int(num) => write!(f, "{}", num),
                Value::Text(s) => write!(f, "{}", s),
            }
        }
    }

    let values = vec![Value::Int(3), Value::Text(String::from("Water"))];
    println!("{:?}", values);
}

fn print_vec_full() {
    let mut v = vec![0; 5];

    v.push(5);
    v.push(6);

    println!("{:?} {} {}", v, v.len(), v[1]);
    println!("{}", v.pop().unwrap());

    let slice: &[usize] = &v;
    println!("{:?}", slice);
}

fn main() {
    vec_indexing();
    iterate_over_vec();
    multi_type_vec();
    print_vec_full();
}
