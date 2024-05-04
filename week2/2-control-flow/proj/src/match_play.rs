use std::io;
use std::io::Read;

pub fn showcase_match() {
    let mut something = String::new();
    
    io::stdin().read_line(&mut something).expect("Failed to read line");
    
    match something.trim() {
        "Hello" => println!("Found Hello"),
        "World" => println!("Found World"),
        "Hello World" => println!("Found Hello World"),
        _ => println!("Found Nothing"),
    }
}