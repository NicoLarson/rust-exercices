use std::io;

fn main() {
    println!("De Celsius à Fahrenheit");
    println!("Veuillez enter une température");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Entrée invalide!");

    let celsius: f32 = input
        .trim()
        .parse()
        .ok()
        .expect("Température invalide!");

    let celsius_to_fahrenheit = celsius_to_fahrenheit(celsius);

    println!("...");
    println!("{}C = {}F", celsius, celsius_to_fahrenheit);
}

fn celsius_to_fahrenheit(celsius: f32) -> f32{
    let fahrenheit: f32 = celsius * (9.0/5.0) + 32.0;
    fahrenheit
}
