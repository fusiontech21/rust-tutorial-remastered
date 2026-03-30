// Chapter 9: Error Handling - Example Code

use std::fs::File;
use std::io::{self, Read};

fn main() {
    // Result with match
    let file_result = File::open("hello.txt");
    match file_result {
        Ok(file) => println!("File opened successfully"),
        Err(_) => println!("Failed to open file"),
    }
    
    // unwrap
    // let file = File::open("hello.txt").unwrap();
    
    // expect
    // let file = File::open("hello.txt").expect("Failed to open hello.txt");
    
    // ? operator
    match read_file() {
        Ok(content) => println!("File content: {}", content),
        Err(e) => println!("Error: {}", e),
    }
    
    // Option
    let opt = Some(5);
    match opt {
        Some(n) => println!("Got: {}", n),
        None => println!("Got None"),
    }
    
    // unwrap_or
    let value = opt.unwrap_or(0);
    println!("Value: {}", value);
}

fn read_file() -> Result<String, io::Error> {
    let mut file = File::open("hello.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
