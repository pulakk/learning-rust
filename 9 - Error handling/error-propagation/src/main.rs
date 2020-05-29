use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file(filename: &str) -> Result<String, io::Error> {
    let f = File::open(filename);

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e), // explicit return because it's not the last statement
    };

    let mut username = String::new();

    match f.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e), // no need to explicitly return
    }
}

fn read_username_from_another_file(filename: &str) -> Result<String, io::Error> {
    let mut username = String::new();

    File::open(filename)?.read_to_string(&mut username)?;
    Ok(username)
}

fn main() {
    println!("{}", read_username_from_file("username.txt").unwrap());
    println!("{}", read_username_from_another_file("username.txt").unwrap());
    println!("{}", std::fs::read_to_string("username.txt").unwrap());
}
