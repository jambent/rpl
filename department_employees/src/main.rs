use std::io;
use std::collections::HashMap;

fn main() {
    println!("Enter number for desired operation:");
    println!("1. Add employee to department");
    println!("2. Get all employees in a department");
    println!("3. Get all employees in company");
    println!("4. Quit");

    let mut input_operation_number = String::new();
    io::stdin()
    .read_line(&mut input_operation_number)
    .expect("Input not recognised");
    
}
