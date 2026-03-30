// Chapter 5: Ownership & Borrowing - Example Code

fn main() {
    // Move semantics
    let s1 = String::from("hello");
    let s2 = s1;  // s1 is moved to s2
    // println!("{}", s1);  // ERROR: s1 is no longer valid
    println!("s2 = {}", s2);
    
    // Clone for deep copy
    let s3 = s2.clone();
    println!("s2 = {}, s3 = {}", s2, s3);
    
    // Copy types
    let x = 5;
    let y = x;  // Copy, not move
    println!("x = {}, y = {}", x, y);
    
    // References
    let s = String::from("hello");
    let len = calculate_length(&s);
    println!("Length of '{}' is {}", s, len);  // s still valid
    
    // Mutable reference
    let mut s = String::from("hello");
    change(&mut s);
    println!("After change: {}", s);
    
    // Multiple immutable references
    let r1 = &s;
    let r2 = &s;
    println!("r1 = {}, r2 = {}", r1, r2);
    
    // One mutable reference
    let r3 = &mut s;
    r3.push_str(", world!");
    println!("r3 = {}", r3);
    
    // Slices
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
