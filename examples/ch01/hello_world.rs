// Chapter 1: Introduction & Setup - Example Code

fn main() {
    // Hello World
    println!("Hello, Rust!");
    
    // Variables and formatting
    let name = "Developer";
    let age = 25;
    println!("Name: {}, Age: {}", name, age);
    
    // Debug formatting
    let numbers = vec![1, 2, 3, 4, 5];
    println!("Numbers: {:?}", numbers);
    
    // Pretty debug
    println!("Numbers (pretty): {:#?}", numbers);
}
