use std::num::ParseIntError;

/// Learning lab: Main entry point for the lab assignment
/// 
/// What it does:
/// 
/// 1. Ask the user to enter numbers until they press enter without entering a number
/// 
/// 2. Print the numbers entered by the user
/// 
/// 3. Print the sum of the numbers
/// 
/// 4. Print the average of the numbers
pub fn learning_lab_arguments_in_functions() {
    let numbers = build_vec_from_user_input();
    
    println!("You entered: {:?}", numbers);
    println!("Sum of numbers: {}", sum(&numbers));
    println!("Average of numbers: {}", average(&numbers));
}

/// Sums the numbers in the given slice
fn sum(numbers: &[i32]) -> i32 {
    let mut result = 0;
    for number in numbers {
        result += number;
    }
    result
}


/// Calculates the average of the numbers in the given slice
fn average(numbers: &[i32]) -> f64 {
    let sum = sum(numbers) as f64;
    let length = numbers.len() as f64;
    sum / length
}

/// Builds a vector of numbers from user input
fn build_vec_from_user_input() -> Vec<i32> {
    let mut numbers = Vec::new();

    // Named loop, just because I love this feature
    'ask_user: loop {
        let mut input = String::new();
        println!("Enter a number (or simply enter to finish): ");
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim().is_empty() {
            break 'ask_user;
        }

        // parse() returns a Result type, which we can match on
        let number: Result<i32, ParseIntError> = input.trim().parse();
        match number {
            Ok(n) => numbers.push(n),
            Err(_) => {
                println!("Invalid number. Please try again.");
                // No need to panic here, just ignore the invalid input
                continue 'ask_user;
            }
        }
    }

    numbers
}
