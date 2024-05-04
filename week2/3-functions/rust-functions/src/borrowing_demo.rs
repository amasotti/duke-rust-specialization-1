pub fn demo_borrwing() {
    let s = String::from("A wonderful test string");
    let mut v = vec![1, 2, 3, 4];
    
    //own_string(&s);
    borrow_string(&s);
    println!("The string is available here again: {}", s);
    
    //own_vec(v);
    borrow_vec(&mut v);
    println!("The vector is available here again: {:?}", v);
    
}

/// This will move the ownership of the string to the function and then drop it
fn own_string(s: String) {
    println!("I own this string: {}", s);
}

/// This will borrow the string and print it
fn borrow_string(s: &String) {
    println!("I borrowed this string: {}", s);
}

fn own_vec(mut v: Vec<i32>) {
    v.push(5);
    println!("I own this vector: {:?}", v);
}

fn borrow_vec(v: &mut Vec<i32>) {
    v.push(6); // This will not work because the vector is borrowed (immutable borrow
    println!("I borrowed this vector: {:?}", v);
}