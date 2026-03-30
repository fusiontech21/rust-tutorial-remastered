# Chapter 4: Control Flow

## 4.1 Conditional Expressions

### if Expressions

In Rust, `if` is an **expression**, not a statement. This means it returns a value.

```rust
fn main() {
    let number = 10;
    
    // Basic if statement
    if number > 5 {
        println!("{} is greater than 5", number);
    }
    
    // if-else
    if number % 2 == 0 {
        println!("{} is even", number);
    } else {
        println!("{} is odd", number);
    }
    
    // if-else if-else
    let temperature = 25;
    
    if temperature < 0 {
        println!("Freezing!");
    } else if temperature < 15 {
        println!("Cold");
    } else if temperature < 25 {
        println!("Mild");
    } else if temperature < 35 {
        println!("Warm");
    } else {
        println!("Hot!");
    }
    
    // if as an expression (returns a value)
    let message = if number > 10 {
        "greater than 10"
    } else {
        "10 or less"
    };
    println!("Number is {}", message);
    
    // All branches must return the same type
    let result = if number > 0 {
        "positive"
    } else if number < 0 {
        "negative"
    } else {
        "zero"
    };
    
    // Can use blocks in if expressions
    let value = if number > 5 {
        let x = number * 2;
        x + 1  // No semicolon = return value
    } else {
        number
    };
    println!("Value: {}", value);
}
```

### Common Mistake: No Ternary Operator

Rust doesn't have a ternary operator (`condition ? true : false`), but `if-else` expressions serve the same purpose:

```rust
fn main() {
    let number = 10;
    
    // ❌ This doesn't work in Rust
    // let result = number > 5 ? "big" : "small";
    
    // ✅ This is the Rust way
    let result = if number > 5 { "big" } else { "small" };
    println!("Result: {}", result);
}
```

---

## 4.2 Pattern Matching with match

The `match` expression is one of Rust's most powerful features.

### Basic match

```rust
fn main() {
    let number = 5;
    
    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Something else"),  // _ is a wildcard
    }
    
    // match as an expression
    let description = match number {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "other",
    };
    println!("Number is {}", description);
    
    // All possible values must be covered
    // This won't compile without the _ wildcard:
    // match number {
    //     1 => println!("One"),
    //     2 => println!("Two"),
    // }  // ERROR: not exhaustive
}
```

### match with Ranges

```rust
fn main() {
    let dice_roll = 4;
    
    match dice_roll {
        1 => println!("Got a one!"),
        2..=5 => println!("Got {} to 5", dice_roll),
        6 => println!("Got a six!"),
        _ => unreachable!(),
    }
    
    // Grade calculator
    let score = 85;
    let grade = match score {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        0..=59 => 'F',
        _ => panic!("Invalid score: {}", score),
    };
    println!("Grade: {}", grade);
}
```

### match Guards

```rust
fn main() {
    let number = Some(4);
    
    match number {
        Some(n) if n % 2 == 0 => println!("Even number: {}", n),
        Some(n) if n % 2 == 1 => println!("Odd number: {}", n),
        Some(n) => println!("Number: {}", n),
        None => println!("No number"),
    }
    
    // Multiple conditions in guard
    let point = (3, 4);
    match point {
        (x, y) if x > 0 && y > 0 => println!("First quadrant"),
        (x, y) if x < 0 && y > 0 => println!("Second quadrant"),
        (x, y) if x < 0 && y < 0 => println!("Third quadrant"),
        (x, y) if x > 0 && y < 0 => println!("Fourth quadrant"),
        _ => println!("On an axis"),
    }
}
```

### match with Multiple Patterns

```rust
fn main() {
    let coin = "H";
    
    match coin {
        "H" | "h" => println!("Heads!"),
        "T" | "t" => println!("Tails!"),
        _ => println!("Invalid coin"),
    }
    
    // Multiple patterns with guards
    let x = 5;
    match x {
        1 | 3 | 5 | 7 | 9 => println!("Odd single digit"),
        2 | 4 | 6 | 8 => println!("Even single digit"),
        _ => println!("Other"),
    }
}
```

### Destructuring in match

```rust
fn main() {
    // Tuple destructuring
    let point = (3, 4);
    match point {
        (0, 0) => println!("Origin"),
        (0, y) => println!("On Y axis at {}", y),
        (x, 0) => println!("On X axis at {}", x),
        (x, y) => println!("At ({}, {})", x, y),
    }
    
    // Struct destructuring
    struct Point {
        x: i32,
        y: i32,
    }
    
    let p = Point { x: 5, y: 10 };
    match p {
        Point { x: 0, y: 0 } => println!("Origin"),
        Point { x: 0, y } => println!("On Y axis at {}", y),
        Point { x, y: 0 } => println!("On X axis at {}", x),
        Point { x, y } => println!("At ({}, {})", x, y),
    }
    
    // Shorthand when field names match
    match p {
        Point { x: 0, y: 0 } => println!("Origin"),
        Point { x, y } => println!("At ({}, {})", x, y),
    }
    
    // Enum destructuring (covered in Chapter 7)
    enum Option<T> {
        Some(T),
        None,
    }
    
    let opt: Option<i32> = Option::Some(5);
    match opt {
        Option::Some(value) => println!("Value: {}", value),
        Option::None => println!("None"),
    }
}
```

### @ Bindings

```rust
fn main() {
    // Bind matched value to a variable
    let number = Some(5);
    
    match number {
        Some(n @ 1..=10) => println!("Got {} which is between 1 and 10", n),
        Some(n) => println!("Got {}", n),
        None => println!("Got None"),
    }
    
    // Complex example
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    
    let msg = Message::Move { x: 10, y: 20 };
    
    match msg {
        Message::Move { x, y } if x > 5 => println!("Moving far to ({}, {})", x, y),
        Message::Move { x, y } => println!("Moving to ({}, {})", x, y),
        _ => println!("Other message"),
    }
}
```

---

## 4.3 if let

`if let` is a concise way to handle a single pattern.

```rust
fn main() {
    let config_max = Some(3u8);
    
    // Verbose way with match
    match config_max {
        Some(max) => println!("Maximum is {}", max),
        _ => (),
    }
    
    // Concise way with if let
    if let Some(max) = config_max {
        println!("Maximum is {}", max);
    }
    
    // if let with else
    let coin = Some("H");
    
    if let Some(side) = coin {
        println!("Got {}", side);
    } else {
        println!("Got nothing");
    }
    
    // if let with multiple conditions
    let number = Some(5);
    
    if let Some(n) = number {
        if n > 0 {
            println!("Positive number: {}", n);
        }
    }
    
    // Combining with regular if
    let x = 10;
    let y = Some(5);
    
    if x > 5 && let Some(n) = y {
        println!("x is {} and y contains {}", x, n);
    }
    
    // if let with pattern matching
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }
    
    let color = Color::Rgb(255, 0, 0);
    
    if let Color::Rgb(r, g, b) = color {
        println!("RGB color: ({}, {}, {})", r, g, b);
    }
}
```

---

## 4.4 while Loops

```rust
fn main() {
    // Basic while loop
    let mut count = 0;
    
    while count < 5 {
        println!("Count: {}", count);
        count += 1;
    }
    
    // while as an expression
    let mut x = 0;
    let result = while x < 5 {
        x += 1;
    };  // result is ()
    
    // while with break value
    let mut counter = 0;
    let result = while counter < 10 {
        counter += 1;
        if counter == 5 {
            break counter * 2;  // Break with value
        }
    };
    println!("Result: {}", result);  // 10
    
    // Infinite loop with while true (prefer loop for this)
    while true {
        println!("Infinite...");
        break;
    }
}
```

---

## 4.5 loop

### Basic loop

```rust
fn main() {
    // Infinite loop
    let mut count = 0;
    
    loop {
        count += 1;
        println!("Count: {}", count);
        
        if count >= 5 {
            break;
        }
    }
    
    // loop with return value
    let result = loop {
        count += 1;
        if count == 10 {
            break count * 2;  // Return value from loop
        }
    };
    println!("Result: {}", result);
}
```

### Loop Labels

```rust
fn main() {
    // Breaking out of nested loops
    let mut count = 0;
    
    'outer: loop {
        println!("Outer loop iteration {}", count);
        
        'inner: loop {
            if count >= 3 {
                break 'outer;  // Break out of outer loop
            }
            
            if count == 2 {
                println!("Breaking inner at count 2");
                break 'inner;  // Break only inner loop
            }
            
            count += 1;
        }
        
        count += 1;
    }
    
    // Continuing outer loop from inner
    let mut x = 0;
    let mut y = 0;
    
    'count_x: loop {
        if x >= 3 {
            break 'count_x;
        }
        
        'count_y: loop {
            if y >= 3 {
                y = 0;
                x += 1;
                continue 'count_x;  // Continue outer loop
            }
            
            println!("({}, {})", x, y);
            y += 1;
        }
    }
}
```

---

## 4.6 for Loops

### Iterating over Ranges

```rust
fn main() {
    // Inclusive range (0, 1, 2, 3, 4)
    for i in 0..5 {
        println!("i = {}", i);
    }
    
    // Inclusive range (0, 1, 2, 3, 4, 5)
    for i in 0..=5 {
        println!("i = {}", i);
    }
    
    // Reverse iteration
    for i in (0..5).rev() {
        println!("Reverse: {}", i);
    }
    
    // With step (requires step_by)
    for i in (0..10).step_by(2) {
        println!("Even: {}", i);
    }
}
```

### Iterating over Collections

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    // Iterate by reference (doesn't consume the vector)
    for num in &numbers {
        println!("Number: {}", num);
    }
    
    // Iterate by mutable reference
    let mut mutable_numbers = vec![1, 2, 3, 4, 5];
    for num in &mut mutable_numbers {
        *num *= 2;
    }
    println!("Doubled: {:?}", mutable_numbers);
    
    // Iterate by value (consumes the vector)
    for num in numbers {
        println!("Owned: {}", num);
    }
    // numbers is moved here, can't use it anymore
    
    // Iterate with index using enumerate
    let fruits = ["apple", "banana", "cherry"];
    for (index, fruit) in fruits.iter().enumerate() {
        println!("{}: {}", index, fruit);
    }
    
    // Iterate over string characters
    for c in "Hello, Rust!".chars() {
        println!("Char: {}", c);
    }
    
    // Iterate over HashMap
    use std::collections::HashMap;
    
    let mut scores = HashMap::new();
    scores.insert("Alice", 90);
    scores.insert("Bob", 85);
    scores.insert("Charlie", 95);
    
    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }
}
```

### for with Pattern Matching

```rust
fn main() {
    let pairs = vec![(1, 2), (3, 4), (5, 6)];
    
    // Destructure in for loop
    for (a, b) in pairs {
        println!("a = {}, b = {}", a, b);
    }
    
    // Match in for loop
    let options = vec![Some(1), None, Some(3)];
    
    for opt in options {
        if let Some(value) = opt {
            println!("Value: {}", value);
        }
    }
}
```

---

## 4.7 continue and break

```rust
fn main() {
    // break exits the loop
    for i in 0..10 {
        if i == 5 {
            break;  // Exit loop when i is 5
        }
        println!("i = {}", i);
    }
    
    // continue skips to next iteration
    for i in 0..10 {
        if i % 2 == 0 {
            continue;  // Skip even numbers
        }
        println!("Odd: {}", i);
    }
    
    // break and continue with values in loops
    let mut sum = 0;
    for i in 0.. {
        if i > 100 {
            break sum;
        }
        if i % 2 == 0 {
            continue;
        }
        sum += i;
    }
}
```

---

## 4.8 Advanced Patterns

### Nested Control Flow

```rust
fn main() {
    // Complex nested control flow
    for i in 1..=10 {
        if i % 3 == 0 {
            continue;
        }
        
        if i > 7 {
            break;
        }
        
        match i {
            1 => println!("One"),
            2 | 4 | 5 => println!("Even or five"),
            n if n < 5 => println!("Small: {}", n),
            _ => println!("Other: {}", i),
        }
    }
}
```

### Early Returns

```rust
fn find_first_positive(numbers: &[i32]) -> Option<i32> {
    for &n in numbers {
        if n > 0 {
            return Some(n);  // Early return
        }
    }
    None
}

fn validate_input(x: i32, y: i32) -> Result<(), &'static str> {
    if x < 0 {
        return Err("x must be positive");
    }
    if y < 0 {
        return Err("y must be positive");
    }
    if x > 100 {
        return Err("x too large");
    }
    Ok(())
}

fn main() {
    let nums = [-5, -3, 0, 2, 4, 6];
    match find_first_positive(&nums) {
        Some(n) => println!("First positive: {}", n),
        None => println!("No positive numbers"),
    }
    
    match validate_input(50, 30) {
        Ok(()) => println!("Input valid"),
        Err(e) => println!("Error: {}", e),
    }
}
```

### Guard Clauses

```rust
fn process_data(data: Option<Vec<i32>>) -> i32 {
    // Guard clause pattern
    let data = match data {
        Some(d) => d,
        None => return 0,
    };
    
    if data.is_empty() {
        return 0;
    }
    
    data.iter().sum()
}

// Alternative using if let
fn process_data_v2(data: Option<Vec<i32>>) -> i32 {
    if let Some(d) = data {
        if !d.is_empty() {
            return d.iter().sum();
        }
    }
    0
}
```

---

## 4.9 Performance Considerations

### Loop Optimization

```rust
fn main() {
    // Prefer for loops over while when possible
    let numbers = vec![1, 2, 3, 4, 5];
    
    // ✅ Idiomatic and optimized
    for n in &numbers {
        println!("{}", n);
    }
    
    // ❌ More verbose, same performance
    let mut i = 0;
    while i < numbers.len() {
        println!("{}", numbers[i]);
        i += 1;
    }
    
    // Iterators are zero-cost abstractions
    let sum: i32 = numbers.iter().sum();
    
    // This compiles to the same code as a manual for loop
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
}
```

### Range Performance

```rust
fn main() {
    // Ranges are lazy and don't allocate memory
    for i in 0..1_000_000 {
        // This doesn't create a million-element array
        if i == 10 {
            break;
        }
    }
    
    // Collecting a range creates a Vec
    let numbers: Vec<i32> = (0..10).collect();
}
```

---

## Chapter 4 Exercises

### Exercise 4.1: FizzBuzz
```rust
// Classic FizzBuzz:
// Print numbers 1 to 100
// For multiples of 3, print "Fizz"
// For multiples of 5, print "Buzz"
// For multiples of both 3 and 5, print "FizzBuzz"
// Otherwise print the number
```

### Exercise 4.2: Prime Number Checker
```rust
// Write a function that checks if a number is prime
// Use a for loop with early return
// Test with various numbers
```

### Exercise 4.3: Pattern Matching Calculator
```rust
// Create a simple calculator using match
// Support: add, subtract, multiply, divide
// Handle division by zero
// Use enum for operations
```

### Exercise 4.4: Number Guessing Game
```rust
// Generate a random number (use rand crate)
// Let user guess in a loop
// Provide hints (higher/lower)
// Count attempts and display when correct
```

### Exercise 4.5: Grade Statistics
```rust
// Given a list of scores:
// - Calculate average
// - Find highest and lowest
// - Count grades in each range (A, B, C, D, F)
// - Use match for grade classification
```

---

## Common Pitfalls

### Off-by-One Errors

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    // ❌ WRONG - goes out of bounds
    // for i in 0..=numbers.len() {
    //     println!("{}", numbers[i]);
    // }
    
    // ✅ CORRECT
    for i in 0..numbers.len() {
        println!("{}", numbers[i]);
    }
    
    // ✅ BETTER - iterate directly
    for num in &numbers {
        println!("{}", num);
    }
}
```

### Infinite Loops

```rust
fn main() {
    // ❌ Missing increment
    // let mut i = 0;
    // while i < 10 {
    //     println!("{}", i);
    //     // Forgot: i += 1;
    // }
    
    // ✅ CORRECT
    let mut i = 0;
    while i < 10 {
        println!("{}", i);
        i += 1;
    }
}
```

### Match Exhaustiveness

```rust
fn main() {
    let x = Some(5);
    
    // ❌ This won't compile - not exhaustive
    // match x {
    //     Some(n) => println!("{}", n),
    // }
    
    // ✅ CORRECT
    match x {
        Some(n) => println!("{}", n),
        None => println!("None"),
    }
    
    // ✅ Or use if let for single pattern
    if let Some(n) = x {
        println!("{}", n);
    }
}
```

---

## Summary

In this chapter, you learned:

✅ `if` expressions (not statements!) and their usage
✅ Pattern matching with `match` expressions
✅ Match guards and multiple patterns
✅ `if let` for single-pattern matching
✅ `while` loops for conditional iteration
✅ `loop` for infinite loops with break values
✅ Loop labels for nested loop control
✅ `for` loops with ranges and collections
✅ `break` and `continue` statements
✅ Early returns and guard clauses
✅ Performance considerations for loops

---

## What's Next?

Now that you've mastered control flow, you're ready for Rust's most important concept! In Chapter 5, we'll dive deep into **Ownership and Borrowing** - the feature that makes Rust unique.

**Continue to [Chapter 5: Ownership & Borrowing](./05_ownership_borrowing.md)**
