use std::io::stdin;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 20);

    loop {
        println!("Guess the number (1 to 20):");

        // let mut input = String::new();
        let mut input = String::from("00");

        stdin().read_line(&mut input).expect("Invalid input");

        input = String::from(input.trim()); // to remove \n
        println!("Printing input: {}", input);

        // let input:u16 = input.trim().parse() // parse returns a Result Enum, not the number
        //    .expect("Please type a number!"); // expect returns the number

        let input:u32 = match input.parse() {
            Ok(n) => {
                n
            },
            Err(e) => {
                println!("{}. Please enter a number!", e);
                continue;
            },
        };

        println!("You guessed: {}", input);
        // println!("Secret Number: {}", secret_number);

        match input.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("The guess {} is correct. Winner Winner, Chicken Dinner!", input);
                break;
            },
        }
    }
}
