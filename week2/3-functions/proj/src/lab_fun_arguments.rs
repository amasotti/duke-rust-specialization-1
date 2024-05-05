use std::num::ParseIntError;

pub fn learning_lab_arguments_in_functions() {
    let numbers = build_vec_from_user_input();
    
    println!("You entered: {:?}", numbers);
    println!("Sum of numbers: {}", sum(&numbers));
    println!("Average of numbers: {}", average(&numbers));
}

fn sum(numbers: &[i32]) -> i32 {
    let mut result = 0;
    for number in numbers {
        result += number;
    }
    result
}

fn average(numbers: &[i32]) -> f64 {
    let sum = sum(numbers) as f64;
    let length = numbers.len() as f64;
    sum / length
}

fn build_vec_from_user_input() -> Vec<i32> {
    let mut numbers = Vec::new();

    'ask_user: loop {
        let mut input = String::new();
        println!("Enter a number (or simply enter to finish): ");
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim().is_empty() {
            break 'ask_user;
        }

        let number: Result<i32, ParseIntError> = input.trim().parse();
        match number {
            Ok(n) => numbers.push(n),
            Err(_) => {
                println!("Invalid number. Please try again.");
                continue 'ask_user;
            }
        }
    }

    numbers
}
