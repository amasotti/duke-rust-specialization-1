fn main() {
    variable_assignment();

    print_separator();

    variable_reassignment();
}

fn print_separator() {
    println!("-----------------");
}

fn variable_assignment() {
    let message = "Name: Antonio Masotti";
    let age = 30;
    let weight_lbs = 190.0;

    let kilos = weight_lbs / 2.2046;

    println!("{}", message);
    println!("Age: {}", age);
    println!("Weight: {:.2} kg", kilos);
}

/// Variables can be reassigned if they are declared as mutable
fn variable_reassignment() {
    /// Note the `mut` keyword to make the variable mutable
    let mut height = 72.0;

    println!("Height: {}", height);

    height = 72.5;
    println!("New Height: {}", height);
}
