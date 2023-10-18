use std::env;

/*
    A program that accepts arguments <temperature> <unit> then gives an output of converted temperature
*/
fn main() {
    let args: Vec<String> = env::args().collect();
    
    let temp: &f32 = &args[1].trim().parse().expect("Must be a number");
    let unit: &str = &args[2];

    let (input_unit, result) = match unit {
        "fahrenheit" => (symbol("celsius"), fahrenheit(*temp)),
        "celsius" => (symbol("fahrenheit"), celsius(*temp)),
        &_ => ('0', 0.0)
    };

    println!("{temp}{input_unit} converted to {result}{}", symbol(unit));
}

fn fahrenheit(celsius: f32) -> f32 { (celsius * 2.0) + 30.0 }
fn celsius(fahrenheit: f32) -> f32 { (fahrenheit - 30.0) / 2.0 }
fn symbol(str: &str) -> char { str.chars().nth(0).expect("Alphabetic Characters Only"). to_ascii_uppercase() }
