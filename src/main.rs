// Convert temperatures between Fahrenheit and Celsius

// Temperature in degrees Fahrenheit (°F) = (Temperature in degrees Celsius (°C) * 9/5) + 32
// Temperature in degrees Celsius (°C) = (Temperature in degrees Fahrenheit (°F) - 32) * 5/9

use std::io;

fn main() {
    println!("Hello Rustacean ;) ;)");

    let mut which_conversion: String = String::new();
    println!("In which conversion you want to do. ( F for Fahrenheit, C for Celsius ) ?");
    io::stdin()
        .read_line(&mut which_conversion)
        .expect("Provider the conversion");
    println!("You want to convert it into {}", which_conversion);
    println!("What is the temperature ?");
    let which_conversion = which_conversion.trim();
    let mut temp: String = String::new();

    io::stdin()
        .read_line(&mut temp)
        .expect("Provide the temperature");

    let temp: f64 = temp.trim().parse().expect("Provide number");
    let temp: f64 = _convertiser(temp, &which_conversion);

    println!(
        "The temperature you provide is {} {}",
        temp, which_conversion
    );
}

fn _convertiser(temp: f64, convert_into: &str) -> f64 {
    let number: f64;
    if convert_into == "F" {
        number = (temp * (9 as f64 / 5 as f64)) + 32 as f64;
    } else {
        number = (temp - 32 as f64) * (5 as f64 / 9 as f64);
    }
    return number;
}
