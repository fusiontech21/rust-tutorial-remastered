// Chapter 7: Enums & Pattern Matching - Example Code

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
}

fn main() {
    // Enum variants
    let m1 = Message::Quit;
    let m2 = Message::Move { x: 10, y: 20 };
    let m3 = Message::Write(String::from("hello"));
    let m4 = Message::ChangeColor(255, 0, 0);
    
    // Pattern matching
    match m2 {
        Message::Quit => println!("Quitting"),
        Message::Move { x, y } => println!("Moving to ({}, {})", x, y),
        Message::Write(text) => println!("Message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Color: ({}, {}, {})", r, g, b),
    }
    
    // Option
    let some_value = Option::Some(5);
    let none_value: Option<i32> = Option::None;
    
    match some_value {
        Option::Some(n) => println!("Got: {}", n),
        Option::None => println!("Got None"),
    }
    
    // if let with Option
    if let Option::Some(n) = some_value {
        println!("Value: {}", n);
    }
    
    // Real Option usage
    let numbers = vec![1, 2, 3];
    let first = numbers.first();
    match first {
        Some(n) => println!("First number: {}", n),
        None => println!("No numbers"),
    }
}
