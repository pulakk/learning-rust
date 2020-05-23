use std::io::{self,Write};

fn fibonacci_naive(n: u8) -> u32 {
    if n == 0 { 0 }
    else if n == 1 { 1 }
    else { fibonacci(n-1) + fibonacci(n-2) }
}

fn fibonacci(n: u8) -> u32 {
    if n == 0 { return 0 };

    let (mut p, mut q) = (0, 1);

    for _ in 1..n {
        let tmp = q;
        q = p + q;
        p = tmp;
    }

    return q;
}

fn main() {
    const MAX_N: u8 = 40;
    let mut n = String::new();

    print!("N = ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut n).unwrap();

    let n: u8 = match n.trim().parse() {
        Ok(num) => {
            if num > MAX_N  {
                println!("ERROR: \"Number too large\" Allowed: 0 <= N <= {}", MAX_N);
                return;
            }
            num
        },
        Err(e) => {
            println!("ERROR: \"{}\" Allowed: 0 <= N <= {}", e, MAX_N);
            return;
        }
    };

    for i in 1..n {
        println!("The {}th fibonacci number is {} ({})", i, fibonacci_naive(i), fibonacci(i));
    }
}
