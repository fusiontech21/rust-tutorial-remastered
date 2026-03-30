// Chapter 4: Control Flow - Example Code

fn main() {
    // if expression
    let number = 10;
    
    if number > 5 {
        println!("{} is greater than 5", number);
    } else {
        println!("{} is 5 or less", number);
    }
    
    // if as expression
    let result = if number > 5 { "big" } else { "small" };
    println!("Number is {}", result);
    
    // match expression
    match number {
        1 => println!("One"),
        2 | 3 | 4 => println!("Small number"),
        5..=10 => println!("Medium number"),
        _ => println!("Large number"),
    }
    
    // if let
    let option = Some(5);
    if let Some(value) = option {
        println!("Got value: {}", value);
    }
    
    // while loop
    let mut count = 0;
    while count < 5 {
        println!("Count: {}", count);
        count += 1;
    }
    
    // for loop with range
    for i in 0..5 {
        println!("Range i: {}", i);
    }
    
    // for loop with collection
    let numbers = vec![1, 2, 3, 4, 5];
    for (index, value) in numbers.iter().enumerate() {
        println!("numbers[{}] = {}", index, value);
    }
    
    // loop with break
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Loop result: {}", result);
}
