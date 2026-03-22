use std::io;

fn main() {
    let mut fahrenheit_input = String::new();
    println!("Enter temperature in Fahrenheit, for conversion to Celsius");
    io::stdin() 
        .read_line(&mut fahrenheit_input)
        .expect("Temperature input failed");

    let fahrenheit_input: f64 = fahrenheit_input
                                .trim().parse()
                                .expect("Temperature input was not a number"); 

    println!("The equivalent Celsius temperature is {}",
                    (fahrenheit_input - 32.0) * 5.0 / 9.0);
}
