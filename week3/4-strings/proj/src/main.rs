fn main() {
    string_example();

    // Concatenation
    modify_string();

    // Query capacity
    query_capacity();

    // Format
    format_string();

    // Accessing strings & slices
    let s = access_string();

    // Iterating over chars in a string
    // Notice we're passing a reference to the string as &str, not String
    iterate_over_chars(&s);

    // Iterating over bytes in a string
    iterate_over_bytes(&s);

    // Splitting a string and collecting the results into a vector
    split_and_collect();
}

fn string_example() {
    // Working with strings
    let s = String::from("hello");
    let s2 = s.clone();
    println!("s = {}, s2 = {}", s, s2);
}

fn modify_string() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    let s1 = String::from("hello");
    let s2 = String::from("world");
    let s3 = s1 + &s2;
    println!("{}", s3);
}

fn query_capacity() {
    // Initialize a new empty String
    let mut s = String::new();

    // Check initial length and capacity
    println!("Initial length: {}", s.len()); // Output: 0
    println!("Initial capacity: {}", s.capacity()); // Output: 0

    // Add some content to the string
    s.push_str("hello");

    // Check length and capacity after adding content
    println!("Length after adding 'hello': {}", s.len()); // Output: 5
    println!("Capacity after adding 'hello': {}", s.capacity()); // Likely output: something >= 5, depends on allocation strategy

    // Ensure the capacity to handle more characters without reallocating
    s.reserve(10);

    // Check length and capacity after reserving additional space
    println!("Length after reserve: {}", s.len()); // Output: still 5
    println!("Capacity after reserve: {}", s.capacity()); // Output: likely at least 15

    // Adding more characters
    s.push_str(" world");

    // Final length and capacity
    println!("Final length: {}", s.len()); // Output: 11
    println!("Final capacity: {}", s.capacity()); // Output: likely unchanged if it was already sufficient
}

fn format_string() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);
}

/// Notice the type of the indexed slice is &str
/// which is the second most popular way of working with strings in Rust
/// They're immutable and are a reference to the original string
fn access_string() -> String {
    let s = String::from("hello");
    let h = &s[0..1];
    let e = &s[1..2];
    let ell = &s[1..4];
    let lo = &s[3..];
    println!("{} {} {} {}", h, e, ell, lo);
    s
}

fn iterate_over_chars(s: &str) {
    for c in s.chars() {
        println!("{}", c);
    }
}

fn iterate_over_bytes(s: &str) {
    for b in s.bytes() {
        println!("{}", b);
    }
}

fn split_and_collect() {
    let s = String::from("lorem   ipsum          dolor sit amet");
    let words: Vec<&str> = s.split_whitespace().collect();
    println!("{:?}", words);
}
