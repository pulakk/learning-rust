use std::io::stdin;

fn convert(f: i32) -> i32 {
    (f - 32) * 5 / 9
}

fn main() {
    let mut fahrenheit = String::new();

    println!("Please enter Temperature in fahrenheit ..");
    stdin().read_line(&mut fahrenheit).unwrap();

    let fahrenheit: i32 = fahrenheit.trim().parse().expect("Could not parse input to integer");

    println!("Temperature is {}°F or {}°C", fahrenheit, convert(fahrenheit));
}
