fn if_let_value(value: Option<u32>) {
    if let Some(3) = value {
        println!("Value is 3");
    } else if let Some(4) = value {
        println!("Value is 4");
    } else if let None = value {
        println!("Value is null");
    } else {
        println!("Value is neither 3 nor 4");
    }
}

fn match_value(value: Option<u32>) {
    match value {
        Some(3) => println!("Value is 3"),
        Some(4) => println!("Value is 4"),
        None => println!("Value is null"),
        _ => println!("Value is neither 3 nor 4"),
    }
}

fn main() {
    for i in 1..6 {
        match i {
            1 => println!("Started"),
            5 => println!("Ended"),
            _ => {},
        }
    }

    let values = [
        Some(3),
        Some(4),
        Some(5),
        None
    ];

    for value in values.iter() {
        if_let_value(*value);
        match_value(*value);
    }
}
