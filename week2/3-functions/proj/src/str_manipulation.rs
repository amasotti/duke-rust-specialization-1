/// This function demonstrates how to manipulate a string in Rust.
pub fn demo_str_manipulation() -> () {
    // Notice here the empty tuple () is used to indicate that the function returns nothing.
    // It's basically Unit of Kotlin
    let s = String::from("hello world");
    let res = split_string(s, ' ', 1);
    println!("Returning fragment #0: {}", res);
}

/// This function splits a string by a delimiter and returns a part of the string.
fn split_string(s: String, del: char, part: usize) -> String {
    let split_s: Vec<&str> = s.split(del).collect();
    let res = split_s.get(part);

    res.unwrap().to_string()
}
