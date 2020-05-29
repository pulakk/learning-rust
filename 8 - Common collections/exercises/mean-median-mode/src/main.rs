use std::io::{Write, stdout, stdin};
use std::collections::HashMap;

fn main() {
    let mut input = String::new();

    print!("Enter a list of numbers: ");
    // example input: 22 120 23 100 24 22 23 24
    // example output: 
    //  Mean is 44
    //  Median is 23.5
    //  Frequency of occurence: {23: 2, 24: 2, 22: 2, 120: 1, 100: 1}
    //  Mode: 22

    stdout().flush().unwrap();
    stdin().read_line(&mut input).expect("invalid input!");

    let mut numbers: Vec<i32> = input
        .trim()
        .split(" ")
        .map(|s| s.parse().unwrap())
        .collect::<Vec<i32>>();
    let size = numbers.len() as i32;
    
    numbers.sort();
    println!("Sorted: {:?}, size: {}", numbers, size);

    println!("Mean is {}", numbers.iter().fold(0i32, |a, b| a + b)/size);

    println!("Median is {}", if size % 2 == 0 {
        (numbers[(size/2) as usize] + numbers[(size-1) as usize/2]) as f64 / 2f64
    } else {
        numbers[(size/2) as usize] as f64
    });

    let mut frequency = HashMap::new();
    for number in numbers { *(frequency.entry(number).or_insert(0)) += 1 };
    println!("Frequency of occurence: {:?}", frequency);
    println!(
        "Mode: {:?}",
        frequency.iter()
            .fold(
                (&0i32, &0i32),
                |a, b| if a.1 > b.1 { a } else { b })
            .0
    );
}
