fn main() {
    let x = false;
    let y = if x { 1 } else { 2 };

    let z = if y == 1 {
        println!("\ty = 1");
        true
    } else {
        println!("\ty = 2");
        false
    };

    println!("=>\tx = {}", z);
}
