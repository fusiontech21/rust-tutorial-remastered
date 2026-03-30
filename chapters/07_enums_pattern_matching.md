# Chapter 7: Enums & Pattern Matching

## 7.1 Defining Enums

### Basic Enum Syntax

```rust
enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    let dir = Direction::North;
    
    match dir {
        Direction::North => println!("Going north!"),
        Direction::South => println!("Going south!"),
        Direction::East => println!("Going east!"),
        Direction::West => println!("Going west!"),
    }
}
```

### Enums with Data

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let m1 = Message::Quit;
    let m2 = Message::Move { x: 10, y: 20 };
    let m3 = Message::Write(String::from("hello"));
    let m4 = Message::ChangeColor(255, 0, 0);
    
    match m2 {
        Message::Quit => println!("Quitting"),
        Message::Move { x, y } => println!("Moving to ({}, {})", x, y),
        Message::Write(text) => println!("Message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Color: ({}, {}, {})", r, g, b),
    }
}
```

### Enum with Methods

```rust
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn process(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to ({}, {})", x, y),
            Message::Write(text) => println!("Write: {}", text),
            Message::ChangeColor(r, g, b) => println!("Color: ({}, {}, {})", r, g, b),
        }
    }
    
    fn move_to(x: i32, y: i32) -> Self {
        Message::Move { x, y }
    }
}

fn main() {
    let msg = Message::move_to(10, 20);
    msg.process();
}
```

---

## 7.2 Option Enum

### Understanding Option

```rust
// Standard library definition
enum Option<T> {
    Some(T),
    None,
}

fn main() {
    // Using Option
    let some_number: Option<i32> = Some(5);
    let some_string: Option<&str> = Some("hello");
    let absent: Option<i32> = None;
    
    // Type inference
    let x = Some(5);  // Option<i32>
    let y: Option<_> = Some(5);  // Explicit Option, inferred inner type
    
    // Pattern matching
    match some_number {
        Some(n) => println!("Number: {}", n),
        None => println!("No number"),
    }
    
    // if let
    if let Some(n) = some_number {
        println!("Number: {}", n);
    }
    
    // unwrap_or
    let value = absent.unwrap_or(42);
    println!("Value: {}", value);  // 42
    
    // unwrap_or_else
    let value = absent.unwrap_or_else(|| {
        println!("Computing default...");
        42
    });
    
    // map
    let doubled = some_number.map(|n| n * 2);
    println!("Doubled: {:?}", doubled);  // Some(10)
    
    // and_then (flat_map)
    let result = some_number.and_then(|n| {
        if n > 0 {
            Some(n * 2)
        } else {
            None
        }
    });
    
    // is_some, is_none
    println!("Is some: {}", some_number.is_some());
    println!("Is none: {}", absent.is_none());
}
```

### Common Option Patterns

```rust
fn find_first_even(numbers: &[i32]) -> Option<i32> {
    for &n in numbers {
        if n % 2 == 0 {
            return Some(n);
        }
    }
    None
}

fn get_username(user_id: u32) -> Option<String> {
    // Simulating database lookup
    if user_id == 1 {
        Some("alice".to_string())
    } else {
        None
    }
}

fn main() {
    let numbers = vec![1, 3, 5, 8, 9];
    match find_first_even(&numbers) {
        Some(n) => println!("First even: {}", n),
        None => println!("No even numbers"),
    }
    
    // Chaining Option operations
    let username = get_username(1)
        .map(|name| name.to_uppercase())
        .unwrap_or_else(|| "UNKNOWN".to_string());
    
    println!("Username: {}", username);
}
```

---

## 7.3 Result Enum

### Understanding Result

```rust
// Standard library definition
enum Result<T, E> {
    Ok(T),
    Err(E),
}

use std::fs::File;
use std::io::{self, Read};

fn main() {
    // File operations return Result
    let file_result = File::open("hello.txt");
    
    match file_result {
        Ok(file) => {
            println!("File opened successfully");
            // Use file
        }
        Err(error) => {
            println!("Error opening file: {:?}", error);
        }
    }
    
    // unwrap - panics on error
    let file = File::open("hello.txt").unwrap();
    
    // expect - panics with custom message
    let file = File::open("hello.txt")
        .expect("Failed to open hello.txt");
    
    // unwrap_or default
    let content = read_file().unwrap_or_else(|_| String::from("default"));
    
    // map and map_err
    let result: Result<i32, &str> = Ok(5);
    let doubled = result.map(|x| x * 2);  // Ok(10)
    
    let result: Result<i32, &str> = Err("error");
    let logged = result.map_err(|e| {
        println!("Error: {}", e);
        e
    });
}

fn read_file() -> Result<String, io::Error> {
    let mut file = File::open("hello.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
```

### Result Propagation

```rust
use std::fs::File;
use std::io::{self, Read};

// Using match
fn read_username_v1() -> Result<String, io::Error> {
    let file_result = File::open("username.txt");
    
    let mut file = match file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    
    let mut username = String::new();
    
    match file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// Using ?
fn read_username_v2() -> Result<String, io::Error> {
    let mut file = File::open("username.txt")?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}

// Chaining ?
fn read_username_v3() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("username.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// Using ? in main
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("file.txt")?;
    println!("{}", content);
    Ok(())
}
```

---

## 7.4 Advanced Pattern Matching

### Exhaustiveness Checking

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// With associated data
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ... other states
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quarter from {:?}", state);
            25
        }
    }
}
```

### Catch-all Patterns

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

fn main() {
    let msg = Message::Write(String::from("hello"));
    
    // _ wildcard
    match msg {
        Message::Write(text) => println!("Text: {}", text),
        _ => (),  // Ignore other variants
    }
    
    // _ as placeholder
    let coin = Coin::Penny;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}", state),
        _ => println!("Not a quarter"),
    }
    
    // Multiple patterns with _
    let x = Some(5);
    match x {
        Some(1) | Some(2) | Some(3) => println!("Small number"),
        Some(_) => println!("Other number"),
        None => println!("None"),
    }
}
```

### Pattern Guards

```rust
fn main() {
    let pair = (2, -2);
    
    match pair {
        (x, y) if x == y => println!("Equal"),
        (x, y) if x + y == 0 => println!("Sum to zero"),
        (x, y) if x > 0 && y > 0 => println!("Both positive"),
        _ => println!("Other"),
    }
    
    // Multiple conditions
    let num = Some(4);
    match num {
        Some(x) if x < 0 => println!("Negative number"),
        Some(x) if x % 2 == 0 => println!("Even number: {}", x),
        Some(x) if x % 2 == 1 => println!("Odd number: {}", x),
        Some(x) => println!("Number: {}", x),
        None => println!("None"),
    }
}
```

### @ Bindings

```rust
enum Message {
    Hello { id: i32 },
    Quit,
}

fn main() {
    let msg = Message::Hello { id: 5 };
    
    match msg {
        Message::Hello { id: id_variable @ 3..=7 } => {
            println!("Found id {} in range 3-7", id_variable);
        }
        Message::Hello { id: 10..=12 } => {
            println!("Found id in range 10-12");
        }
        Message::Hello { id } => {
            println!("Found other id: {}", id);
        }
        Message::Quit => {
            println!("Quit");
        }
    }
}
```

---

## 7.5 if let and while let

### if let

```rust
fn main() {
    let config_max = Some(3u8);
    
    // Verbose match
    match config_max {
        Some(max) => println!("Maximum: {}", max),
        _ => (),
    }
    
    // Concise if let
    if let Some(max) = config_max {
        println!("Maximum: {}", max);
    }
    
    // if let with else
    let coin = Some("H");
    if let Some(side) = coin {
        println!("Got {}", side);
    } else {
        println!("Got nothing");
    }
    
    // Multiple conditions
    let x = 5;
    let y = 10;
    
    if x == 5 && let Ok(val) = Result::Ok::<_, ()>(5) {
        println!("Both conditions met: {}", val);
    }
}
```

### while let

```rust
fn main() {
    // Stack simulation
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    while let Some(top) = stack.pop() {
        println!("Popped: {}", top);
    }
    
    // Iterator
    let mut iter = vec![1, 2, 3].into_iter();
    while let Some(value) = iter.next() {
        println!("Value: {}", value);
    }
}
```

---

## 7.6 Patterns in Different Contexts

### Patterns in let

```rust
fn main() {
    // Destructure tuple
    let (x, y, z) = (1, 2, 3);
    println!("x={}, y={}, z={}", x, y, z);
    
    // Destructure struct
    struct Point {
        x: i32,
        y: i32,
    }
    
    let p = Point { x: 10, y: 20 };
    let Point { x, y } = p;
    println!("x={}, y={}", x, y);
    
    // Destructure with rename
    let Point { x: a, y: b } = p;
    println!("a={}, b={}", a, b);
    
    // Destructure enum
    let opt = Some(5);
    let Some(value) = opt else {
        panic!("Expected Some");
    };
    println!("Value: {}", value);
}
```

### Patterns in Function Parameters

```rust
fn print_coordinates(point: (i32, i32)) {
    println!("Current location: ({}, {})", point.0, point.1);
}

fn print_coordinates_destructured((x, y): (i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

struct Point {
    x: i32,
    y: i32,
}

fn print_point(Point { x, y }: Point) {
    println!("Point at ({}, {})", x, y);
}

fn main() {
    print_coordinates((5, 10));
    print_coordinates_destructured((5, 10));
    
    let p = Point { x: 15, y: 25 };
    print_point(p);
}
```

### Patterns in for Loops

```rust
fn main() {
    let points = vec![(1, 2), (3, 4), (5, 6)];
    
    // Destructure in for loop
    for (x, y) in points {
        println!("Point: ({}, {})", x, y);
    }
    
    // With enumerate
    let items = vec!["a", "b", "c"];
    for (index, item) in items.iter().enumerate() {
        println!("{}: {}", index, item);
    }
    
    // Pattern matching in for
    let options = vec![Some(1), None, Some(3)];
    for opt in options {
        if let Some(value) = opt {
            println!("Value: {}", value);
        }
    }
}
```

---

## 7.7 Refutability

### Refutable vs Irrefutable Patterns

```rust
fn main() {
    // Irrefutable patterns (always match)
    let (x, y) = (1, 2);  // Always succeeds
    let [a, b, c] = [1, 2, 3];  // Always succeeds
    
    // Refutable patterns (might not match)
    let opt = Some(5);
    
    // Must use match or if let for refutable patterns
    match opt {
        Some(x) => println!("Got {}", x),
        None => (),
    }
    
    if let Some(x) = opt {
        println!("Got {}", x);
    }
    
    // This won't compile - refutable pattern in let
    // let Some(x) = opt;  // ERROR
}
```

### else with let

```rust
fn main() {
    // let-else pattern (Rust 1.65+)
    let opt = Some(5);
    
    let Some(value) = opt else {
        panic!("Expected Some, got None");
    };
    
    println!("Value: {}", value);
    
    // Useful for early returns
    fn process(data: Option<i32>) -> i32 {
        let Some(value) = data else {
            return 0;
        };
        value * 2
    }
}
```

---

## 7.8 Practical Examples

### State Machine

```rust
#[derive(Debug, PartialEq)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn next(&self) -> Self {
        match self {
            TrafficLight::Red => TrafficLight::Green,
            TrafficLight::Green => TrafficLight::Yellow,
            TrafficLight::Yellow => TrafficLight::Red,
        }
    }
    
    fn action(&self) -> &'static str {
        match self {
            TrafficLight::Red => "Stop",
            TrafficLight::Yellow => "Caution",
            TrafficLight::Green => "Go",
        }
    }
}

fn main() {
    let mut light = TrafficLight::Red;
    
    for _ in 0..6 {
        println!("Light: {:?}, Action: {}", light, light.action());
        light = light.next();
    }
}
```

### Command Pattern

```rust
enum Command {
    Create { name: String },
    Update { id: u32, data: String },
    Delete { id: u32 },
    List,
    Quit,
}

impl Command {
    fn parse(input: &str) -> Option<Self> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        
        match parts.as_slice() {
            ["create", name] => Some(Command::Create { name: name.to_string() }),
            ["update", id, data @ ..] => {
                Some(Command::Update { 
                    id: id.parse().ok()?, 
                    data: data.join(" ") 
                })
            }
            ["delete", id] => Some(Command::Delete { id: id.parse().ok()? }),
            ["list"] => Some(Command::List),
            ["quit"] => Some(Command::Quit),
            _ => None,
        }
    }
    
    fn execute(&self) {
        match self {
            Command::Create { name } => println!("Creating: {}", name),
            Command::Update { id, data } => println!("Updating {} with {}", id, data),
            Command::Delete { id } => println!("Deleting: {}", id),
            Command::List => println!("Listing all items"),
            Command::Quit => println!("Quitting"),
        }
    }
}

fn main() {
    let commands = vec![
        "create item1",
        "update 1 some data",
        "list",
        "delete 1",
        "quit",
    ];
    
    for cmd_str in commands {
        if let Some(cmd) = Command::parse(cmd_str) {
            cmd.execute();
        }
    }
}
```

---

## Chapter 7 Exercises

### Exercise 7.1: Custom Enum
```rust
// Create an enum for playing cards:
// - Suit: Hearts, Diamonds, Clubs, Spades
// - Rank: Number(2-10), Jack, Queen, King, Ace
// Implement methods to get card value
```

### Exercise 7.2: Option Practice
```rust
// Write functions that return Option:
// - Find maximum in empty-safe way
// - Get first element of slice
// - Parse string to integer (handle errors)
// Chain Option operations
```

### Exercise 7.3: Result Error Handling
```rust
// Create a custom error type
// Write functions that return Result
// Practice ? operator and error propagation
// Implement Display for your error type
```

### Exercise 7.4: Pattern Matching
```rust
// Create a complex enum with nested data
// Write match expressions with:
// - Guards
// - @ bindings
// - Multiple patterns
// - Nested patterns
```

### Exercise 7.5: State Machine
```rust
// Implement a vending machine state machine:
// - States: Idle, HasMoney, Dispensing
// - Events: InsertCoin, SelectItem, Dispense
// - Handle invalid transitions
```

---

## Summary

In this chapter, you learned:

✅ Defining enums with and without data
✅ The Option<T> enum for nullable values
✅ The Result<T, E> enum for error handling
✅ Pattern matching with match expressions
✅ Pattern guards and @ bindings
✅ if let and while let for single patterns
✅ Patterns in let, function parameters, and for loops
✅ Refutable vs irrefutable patterns
✅ let-else pattern for early returns
✅ Practical patterns (state machines, commands)

---

## What's Next?

Now that you can define complex data types, let's explore Rust's built-in collections! In Chapter 8, we'll dive into **Vec, String, and HashMap**.

**Continue to [Chapter 8: Collections](./08_collections.md)**
