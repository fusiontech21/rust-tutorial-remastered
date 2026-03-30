// Chapter 2: Basic Syntax & Variables - Example Code

fn main() {
    // Immutable variables
    let x = 5;
    println!("x = {}", x);
    // x = 6;  // ERROR: x is immutable
    
    // Mutable variables
    let mut y = 10;
    println!("y = {}", y);
    y = 15;
    println!("y after mutation = {}", y);
    
    // Shadowing
    let z = 5;
    let z = z + 1;
    let z = z * 2;
    println!("z after shadowing = {}", z);
    
    // Shadowing with type change
    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces length = {}", spaces);
    
    // Constants
    const MAX_POINTS: u32 = 100_000;
    println!("Max points: {}", MAX_POINTS);
    
    // Functions
    println!("add(3, 4) = {}", add(3, 4));
    
    // Block expressions
    let result = {
        let a = 10;
        let b = 20;
        a + b  // No semicolon = return value
    };
    println!("Block result = {}", result);
}

fn add(a: i32, b: i32) -> i32 {
    a + b  // No semicolon = return value
}
