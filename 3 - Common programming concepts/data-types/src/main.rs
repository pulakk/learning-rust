fn main() {
    // tuples
    let tup = (1, 6.4, 'a');

    let (x, y, _z) = tup;
    println!("{}, {}, {}", x, y, tup.2);

    // arrays
    let a: [i32; 10] = [1; 10];
    println!("{}", a[9]);
}
