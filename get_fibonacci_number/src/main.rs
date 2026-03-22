use std::io;

fn main() {
    println!("Find the nth Fibonacci number");
    println!("Enter n:");

    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to process input n value");

    let n: i64 = n.trim().parse().expect("Input must be an integer");
    let fibonacci_value = generate_fibonacci_value(n);
    println!("The nth Fibonacci number is {fibonacci_value}");
}

fn generate_fibonacci_value(target_num: i64) -> i64 {
    if target_num == 0 {
        0
    }
    else if target_num == 1 {
        1
    }
    else {
        let mut previous_value = 0;
        let mut current_value = 1;
        let mut num = 1;
        let mut fibonacci_value = 1;
        while num != target_num {
            fibonacci_value = previous_value + current_value;
            previous_value = current_value;
            current_value = fibonacci_value;
            num += 1;
        }
        return fibonacci_value;
    }
}