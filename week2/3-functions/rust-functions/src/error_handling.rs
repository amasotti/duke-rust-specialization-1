

use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind};


pub fn demo_error_handling(file_path: &str) {
    // f will be of type Result<File, Error> and that's the idiomatic way to handle errors in Rust
    let f = File::open(file_path);
    
    let f: Option<File> = match f {
        Ok(file) => Some(file),
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => { 
                    println!("FAILURE: \nFile not found, did you spell it correctly?\n");
                    None
                }
                _ => { 
                    println!("Some other error occurred: {:?}", error);
                    None 
                }
            }
        }
    };
    
    if f.is_none() {
        return;
    }
    
    let buff_reader = BufReader::new(f.unwrap());
    for line in buff_reader.lines() {
        println!("{}", line.unwrap());
    }
    
}

/*
In this snippet there are multiple error handling techniques used:

- The idiomatic way to handle errors in Rust is to use the Result type. 
  The Result type is an enum with two variants: Ok and Err. 
  In the case of File::open, the return type is Result<File, Error>.
  
- The match statement is used to match the Result type and handle the Ok and Err variants.

- The buffer at the end uses the unwrap method to get the value from the Option type. 
  The unwrap method is used when you are sure that the value is not None. 
  If the value is None, the unwrap method will panic and the program will crash.
 */

