use std::io;
use std::cmp::Ordering;

mod fahrenheit;
mod fib;
mod carol;

fn help() {

    println!("Enter which program to run:");
    println!("1. Convert Fahrenheit to Celsius");
    println!("2. Generate the n'th fibonacci number");
    println!("3. Print the Christmas carol \"The Twelve Days of Christmas\"");
    
}

fn main() {

    help();
    const MAX_CHOICE:u32  = 3;
    let mut iterations = 0;
    loop {
    iterations+=1;
    if iterations > 5 {
        iterations = 0;
        help();
    }

    println!("Enter choice: ");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");

    let choice: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    match MAX_CHOICE.cmp(&choice) {
        Ordering::Less => continue,
        _ => {},
    };

    match choice {
        1 => { fahrenheit::fahrenheit_to_celsius(); break; },
        2 => { fib::fibonacci(); break; },
        3 => {  carol::twelve_days(); break; },
        _ => continue,
    }

    }
}
