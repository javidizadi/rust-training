use std::io;
const NINE_DIVIDE_FIVE: f32 = 9.0 / 5.0;
fn main() {
    println!("Which one do you want to use?");
    println!("\t1. Celsius to Farehnheit");
    println!("\t2. Farehnheit to Celsius");
    let mut input_base_unit = String::new();
    io::stdin()
        .read_line(&mut input_base_unit)
        .expect("Failed to read input.");
    let input_base_unit: u8 = input_base_unit
        .trim()
        .parse()
        .expect("Failed to select converter.");

    println!("Say temputure: ");
    let mut input_temputure = String::new();
    io::stdin()
        .read_line(&mut input_temputure)
        .expect("Failed to read input.");
    let input_temp: f32 = input_temputure
        .trim()
        .parse()
        .expect("Failed to parse temputure.");

    match input_base_unit {
        1 => println!("{}F", celsius_to_farhenheit(input_temp)),
        2 => println!("{}C", farehnheit_to_celsius(input_temp)),
        _ => panic!("Invalid converter selected"),
    }
}
fn celsius_to_farhenheit(temp: f32) -> f32 {
    (NINE_DIVIDE_FIVE * temp) + 32.0
}
fn farehnheit_to_celsius(temp: f32) -> f32 {
    (temp - 32.0) / NINE_DIVIDE_FIVE
}
