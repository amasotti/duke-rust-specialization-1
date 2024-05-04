use std::io;

pub fn showcase_match() {
    let mut something = String::new();

    io::stdin()
        .read_line(&mut something)
        .expect("Failed to read line");

    match something.to_lowercase().trim() {
        "hello" => println!("Found Hello"),
        "world" => println!("Found World"),
        "hello world" => println!("Found Hello World"),
        _ => println!("Found Nothing"),
    }
}
