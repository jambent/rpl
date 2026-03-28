use std::io;


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
            Err(e) => println!("Integer not recognised")
        }
    }
    int_vector.sort();
    println!("{:?}",int_vector);
    let median_result:f32 = median(&int_vector);
    println!("{:?}",median_result);
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
