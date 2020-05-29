use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // panic!("crash and burn"); // fatal

    #[allow(unused_variables)]
    let filename = "hello.txt";
    let _f = File::open(filename);

    let _f = match _f {
        Ok(file) => {
            println!("Exising file found: {}", filename);
            file
        },
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(filename) {
                Ok(fc) => {
                    println!("New file created: {}", filename);
                    fc
                },
                Err(e) => panic!("Could not create file: {:?}", e),
            },
            other_error => {
                panic!("Problem reading file: {:?}", other_error)
            }
        }
    };
}
