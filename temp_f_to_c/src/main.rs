use std::io;

fn main() {
    println!("Convert temperature Fahrenheit to Celsius!");

    let mut temperature = String::new();
    println!("Enter temperature in Fahrenheit:");

    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");
    let temperature: f64 = temperature.trim().parse().expect("Please enter a number!");
    let result = (temperature-32.0) *5.0 / 9.0;
    println!("{temperature} F = {result} C")
}
