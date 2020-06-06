fn static_lifetime() -> &'static i32 {
    &1
}

fn get_ref(x: &str) -> &str {
    &x
}

#[allow(unused_variables)]
fn first<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// ERROR: returns a value referencing data owned by the current function
// fn inside<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("infernape");
//     result.as_str()
// }

#[derive(Debug)]
struct Pokemon<'a> {
    name: &'a str,
}
 
fn main() {
    let r;

    r = static_lifetime();

    println!("r: {}", r);
    // {
    //     let x = 2;
    //     r = &x;
    // }

    // println!("r: {}", r); // ERROR: x doesn't live long enough

    let string = String::from("bulbasaur");
    let r = get_ref(&string);
    println!("r: {}", r);

    let blaziken = String::from("Blaziken");
    let longest_word;

    if true {
        let charizard = String::from("Charizard");

        longest_word = longest(&blaziken, &charizard);
        println!("first word: {}", first(&blaziken, &charizard));
        println!("longest word: {}", longest_word);
    }

    // println!("longest word: {}", longest_word); // ERROR: borrowed value '&charizard' does not live long enough

    let pikachu;
    {
        // pikachu = Pokemon{name: &String::from("pikachu")}; // ERROR: creates a temporary which is freed while still in use
        let name = String::from("pikachu");
        pikachu = Pokemon { name: &name }; // Struct's lifetime is equal to the lifetime of it's string reference in `name`
        println!("{:?}", pikachu);
    }

    // println!("{:?}", pikachu); // ERROR: borrowed value `name` does not live long enough
}
