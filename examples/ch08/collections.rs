// Chapter 8: Collections - Example Code

use std::collections::HashMap;

fn main() {
    // Vec
    let mut vec = vec![1, 2, 3, 4, 5];
    println!("Vec: {:?}", vec);
    
    vec.push(6);
    println!("After push: {:?}", vec);
    
    let last = vec.pop();
    println!("Popped: {:?}, Vec: {:?}", last, vec);
    
    // Vec access
    println!("First: {}", vec[0]);
    println!("Get safe: {:?}", vec.get(10));
    
    // Vec iteration
    for (index, value) in vec.iter().enumerate() {
        println!("vec[{}] = {}", index, value);
    }
    
    // String
    let mut s = String::from("hello");
    s.push_str(" world");
    println!("String: {}", s);
    
    let s2 = String::from("!");
    let s3 = format!("{}{}", s, s2);
    println!("Formatted: {}", s3);
    
    // String methods
    let text = "  Hello, World!  ";
    println!("Trimmed: '{}'", text.trim());
    println!("Uppercase: '{}'", text.to_uppercase());
    println!("Contains 'Hello': {}", text.contains("Hello"));
    
    // HashMap
    let mut scores = HashMap::new();
    scores.insert("Alice", 90);
    scores.insert("Bob", 85);
    scores.insert("Charlie", 95);
    
    println!("Scores: {:?}", scores);
    
    // HashMap access
    println!("Alice's score: {:?}", scores.get("Alice"));
    println!("Contains Bob: {}", scores.contains_key("Bob"));
    
    // HashMap iteration
    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }
    
    // Entry API
    *scores.entry("David").or_insert(0) += 10;
    println!("After entry: {:?}", scores);
}
