mod str_manipulation;
mod borrowing_demo;
mod test_panic;
mod error_handling;

fn main() {
   str_manipulation::demo_str_manipulation();
    demo_sum_of_numbers();
    borrowing_demo::demo_borrwing();
    test_panic::demo_panic();
    error_handling::demo_error_handling("non_existent_file.txt");
    error_handling::demo_error_handling("Cargo.toml")
}

fn demo_sum_of_numbers() {
    let n = [1, 2, 3, 4, 5];
    println!("Sum of numbers: {}", _sum_numbers(&n));
    println!("Average of numbers: {}", average(&n));
}

fn _sum_numbers(numbers: &[i32]) -> i32 {
    let mut sum = 0;
    for n in numbers {
        sum += n;
    }
    sum
}

fn average(n: &[i32]) -> i32 {
    let sum = _sum_numbers(n);
    let length = n.len() as i32;
    sum / length
}
