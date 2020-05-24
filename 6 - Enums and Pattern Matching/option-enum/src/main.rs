const MIN_DRINKING_AGE: u8 = 21;

fn can_drink(age: Option<u32>) -> bool {
    age.unwrap_or(0) >= MIN_DRINKING_AGE as u32
}

fn print_can_drink(age: Option<u32>) {
    match age {
        Some(age) => {
            if age >= MIN_DRINKING_AGE as u32 {
                println!("Age: {} .. you can drink!", age);
            } else {
                println!("Age: {} .. you cannot drink!", age);
            }
        },
        None => {
            println!("You provided no age, you cannot drink!");
        }
    }
}

fn main() {
    let age = Some(22);
    let age_ = Some(18);
    let no_age: Option<u32> = None;

    println!("Age: {} Drinking allowed ? {}", age.unwrap(), can_drink(age));
    println!("Age: {} Drinking allowed ? {}", age_.unwrap(), can_drink(age_));
    println!("Age: None Drinking allowed ? {}", can_drink(no_age));

    print_can_drink(age);
    print_can_drink(age_);
    print_can_drink(no_age);
}
