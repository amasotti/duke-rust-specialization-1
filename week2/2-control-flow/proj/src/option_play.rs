pub fn conditional_check() {
    let maybe_number: Option<()> = Option::None;
    //let maybe_number = Some(32);

    if let Some(n) = maybe_number {
        println!("Number exists: {:?}", n);
    } else {
        println!("No number");
    }
}

pub fn conditional_nested_check() {
    let maybe_number = Some(Some(32));

    if let Some(n) = maybe_number {
        println!("Number exists: {:?}", n);
    } else if let Some(Some(n)) = maybe_number {
        println!("Number nested exists: {:?}", n);
    } else {
        println!("No number");
    }
}
