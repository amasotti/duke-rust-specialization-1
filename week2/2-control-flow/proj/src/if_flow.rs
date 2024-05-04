/// Dummy function to demonstrate if-else control flow
pub fn if_age() {
    let age = 20;
    if age > 18 {
        println!("You are an adult");
    } else {
        println!("You are a minor");
    }
}

/// Inline if-else
pub fn check_threshold() {
    const TRESHOLD: u32 = 10;
    let val = 5;
    let health = if val > TRESHOLD {
        "Healthy"
    } else {
        "Unhealthy"
    };
    println!("The health status is: {}", health);
}

/// Dummy function to demonstrate if-else control flow passing a reference
pub fn shall_proceed() {
    print!("Passing true to make_choice: ");
    let result = make_choice(&true);
    println!("{}", result);

    print!("Passing false to make_choice: ");
    let result = make_choice(&false);
    println!("{}", result);
}

pub fn make_choice(choice: &bool) -> &str {
    if *choice {
        "You chose true"
    } else {
        "You chose false"
    }
}

/// Dummy function to demonstrate shadowing a variable
pub fn shadowing_var() {
    let x = 5;

    // Shadowing the variable x by re-declaring it. Notice that this is different from mutability
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    // This wouldn't work, since we're trying to change the type of the variable
    // let mut spaces = "   ";
    // spaces = spaces.len();
    //
    // println!("The number of spaces is: {}", spaces);

    // This would work, since we're shadowing the variable
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The number of spaces is: {}", spaces);
}
