fn main() {
    let mut s = String::new();
    s.push_str("open");

    let mut s = String::new();
    s.push_str("close");
    s.push(' ');

    println!("{}{} {}", s, String::from("water"), "bottle".to_string()); // open water bottle
    println!("{}", String::new() + "please" + ", " + &String::from("do") + " " + &s); // please, do open
    println!("{}", s + &String::from("done")); // close done
    // println!("{}", s); // throws an error, s moved (not borrowed) in line 8

    let mut s = String::new();
    s.push_str("close");

    let output = format!("{}{} {}", s, String::new() + " " + &s, s); // close close close
    println!("{}", output);

    let word = String::from("नमस्ते"); // [न, म, स,  ्, त,  े] -> 6 characters
    // println!("{}", &word[0..2]); // Error: byte index 2 is not a char boundary; 
    //                              // it is inside 'न' (bytes 0..3) of `नमस्ते`'
    println!("{}, total {} bytes", &word[0..3], word.len()); // न, total 18 bytes

    for c in word.chars() {
        print!("{}, ", c);
    }
    println!();

    for b in word.bytes() {
        print!("{}, ", b);
    }
    println!();
}
