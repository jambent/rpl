use std::io;
use std::collections::HashMap;


fn main() {
    let mut employee_department_hashmap: HashMap<String,Vec<String>> = HashMap::new();
    
    loop {
        println!("Enter number for desired operation:");
        println!("1. Add employee to department");
        println!("2. Get all employees in a department");
        println!("3. Get all employees in company");
        println!("4. Quit");
        let input_numbers:Vec<u32> = vec![1,2,3,4]; 

        let mut input_operation_number = String::new();
        io::stdin()
        .read_line(&mut input_operation_number)
        .expect("Input not recognised");

        let input_operation_integer: Result<u32,_> = input_operation_number.trim().parse();
        match input_operation_integer {
            Ok(num) => if input_numbers.contains(&num) {
                if num == 1 {
                    println!("Enter string in format\n'Add Paul to Finance':");
                        let mut new_employee_vector:Vec<String> = Vec::new();
                        let mut new_employee_str = String::new();
                        io::stdin()
                        .read_line(&mut new_employee_str)
                        .expect("Could not parse input for new employee");
                        
                        for word in new_employee_str.split_whitespace(){
                            new_employee_vector.push(word.to_string());
                            }  
                        if new_employee_vector.len() == 4 {
                            if &new_employee_vector[0].to_lowercase() == "add" {
                                if &new_employee_vector[2].to_lowercase() == "to"{
                                    let employee = new_employee_vector[1].clone();
                                    let department = new_employee_vector[3].clone();
                                    employee_department_hashmap
                                        .entry(department)
                                        .or_insert_with(Vec::new)
                                        .push(employee);
                                    //println!("{:?}",employee_department_hashmap);
                                }
                            }
                        }
                        else {
                            println!("Input for new employee does not match required format.");
                        }
                    }
                else if num == 2 {
                    println!("{}",&num);
                }
                else if num == 3 {
                    println!("{}",&num);
                }
                else if num == 4 {
                    break;
                }
            }
            else {
                println!("Input number not recognised");
            }
            Err(_e) => println!("Did not recognise input as a number")
        };
    }
}
