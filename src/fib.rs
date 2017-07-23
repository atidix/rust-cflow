use std::io;

pub fn fibonacci() {
    println!("Enter which fibonacci number to generate: ");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n = n.trim().parse::<u32>().unwrap();

    println!("The {}'th fibonacci number is {}", n, fib(n));
}

fn fib(i: u32) -> u32 {
    return match i {
        0 => i,
        1 => i,
        _ => fib(i-1) + fib(i-2),
    }
}