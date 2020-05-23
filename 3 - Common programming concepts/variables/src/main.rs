const fn const_add_one(x: u32) -> u32 {
    x + 1
}

fn add_one(x: u32) -> u32 {
    x + 1
}

fn main() {
    let mut x = 1;
    println!("Number \t\t= {}", x);
    x = 2;
    println!("Number \t\t= {}", x);

    // const SPEED_OF_LIGHT: u32 = add_one(3); // raises error 
    // const SPEED_OF_LIGHT: u32 = const_add_one(x); // raises error "Non constant value: x"
    const SPEED_OF_LIGHT: u32 = const_add_one(2);
    println!("Speed of light \t= {} * 10^8", SPEED_OF_LIGHT);

    let x = add_one(3); // immutable
    println!("Number \t\t= {}", x);

    // x = x + 1; // throws an error
    let x = x + 1;
    println!("Number \t\t= {}", x);

    let spaces = "      ";
    let spaces: u32 = spaces.len() as u32;
    println!("No. of spaces \t= {}", spaces);

    // let mut spaces = "     ";
    // spaces = spaces.len(); // cannot mutate a variable's type, only value
}