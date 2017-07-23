use std::io;

pub fn fahrenheit_to_celsius() {
    println!("Enter the degress in Fahrenheit: ");

    let mut f = String::new();
    io::stdin().read_line(&mut f).expect("Failed to read line");
    let f = f.trim().parse::<f32>().unwrap();
    let c = ((f - 32.0) * 5.0) / 9.0;
    println!("Degress in Celsius is : {}", c);

}