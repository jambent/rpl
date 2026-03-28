use std::io;
use std::collections::HashMap;


fn main() {
    println!("Enter a list of integers, pressing enter after each.");
    println!("Type 'q' and Enter to finish, and get the median and mode of the inputs.");

    let mut int_vector: Vec<i32> = Vec::new();
    

    loop {
        let mut input_str = String::new();
        io::stdin()
            .read_line(&mut input_str)
            .expect("Could not parse input");
        if input_str.trim() == "q"{
            break;
        }
        match input_str.trim().parse::<i32>() {
            Ok(n) => int_vector.push(n),
            Err(_e) => println!("Integer not recognised")
        }
    }
    int_vector.sort();
    let median_result:f32 = median(&int_vector);
    let mode_result:i32 = mode(&int_vector);
    println!("The median of the input integers is {:?}"
                ,median_result);
    println!("The mode is {:?}", mode_result);
}

fn median(int_vector: &Vec<i32>) -> f32{
    let vector_length:usize = int_vector.len();
    if vector_length %2 != 0 {
        let middle_index:usize = vector_length/2; 
        return int_vector[middle_index] as f32;
    }
    else {
        let upper_middle_index:usize = vector_length/2;
        let lower_middle_index:usize = upper_middle_index - 1;
        return ((int_vector[lower_middle_index] +
            int_vector[upper_middle_index]) / 2)  as f32;
    }
}

fn mode(int_vector: &Vec<i32>) -> i32{
    let mut int_counts = HashMap::new();
    for value in int_vector {
        let count = int_counts.entry(value).or_insert(0);
        *count += 1;
    }
    let mut mode_value:&i32 = &int_vector[0]; 
    let mut count_for_mode_value:i32 = int_counts.get(&mode_value).copied().unwrap_or(0);
    for value in int_vector {
        let count_for_value:i32 = int_counts.get(value).copied().unwrap_or(0);
        if count_for_value > count_for_mode_value {
            mode_value = value;
            count_for_mode_value = count_for_value;
        } 
    }
    return *mode_value;
}
