struct Guess {
    value: i32,
}

impl Guess {
    fn new(value: i32) -> Guess {
        if value < 0 || value > 20 {
            panic!("0 <= value <= 20");
        }

        Guess { value }
    }

    fn get(&self) -> i32 {
        self.value
    }
}

fn main() {
    let guess = Guess::new(12).get();

    println!("{}", guess);
}
