fn add_one(x: u32) -> u32 {
    x + 1
}

fn main() {
    let _x = 5;

    let y = {
        let _x = 3;
        _x + 1
    };

    println!("The value of y is: {}", y);

    println!("Add one: {}", add_one(2));
}
