# Chapter 10: Generics & Traits

## 10.1 Generics

### Generic Functions

```rust
// Generic function with single type parameter
fn identity<T>(x: T) -> T {
    x
}

fn main() {
    // Type inference
    let a = identity(5);           // i32
    let b = identity("hello");     // &str
    let c = identity(3.14);        // f64
    
    // Explicit type
    let d = identity::<i32>(10);
    let e = identity::<String>(String::from("hello"));
    
    println!("{}, {}, {}, {}, {}", a, b, c, d, e);
}
```

### Generic Structs

```rust
// Generic struct
struct Point<T> {
    x: T,
    y: T,
}

// Multiple type parameters
struct Pair<T, U> {
    first: T,
    second: U,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    
    let pair = Pair {
        first: String::from("hello"),
        second: 42,
    };
    
    // Access fields
    println!("Point: ({}, {})", integer.x, integer.y);
    println!("Pair: ({}, {})", pair.first, pair.second);
}
```

### Generic Enums

```rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

enum Message<T> {
    Quit,
    Move { x: T, y: T },
    Write(T),
}

fn main() {
    let some: Option<i32> = Option::Some(5);
    let ok: Result<i32, &str> = Result::Ok(42);
    let msg: Message<String> = Message::Write(String::from("hello"));
}
```

### Generic Methods

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
    
    fn x(&self) -> &T {
        &self.x
    }
    
    fn y(&self) -> &T {
        &self.y
    }
}

// Implement for specific type
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Mix generic and specific
impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point::new(5, 10);
    let p2 = Point::new("Hello", "World");
    
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```

### Generic Performance

```rust
// Generics are zero-cost!
// Rust monomorphizes generics at compile time

fn sum_slice(slice: &[i32]) -> i32 {
    slice.iter().sum()
}

// This compiles to the same code as a hand-written loop
// No runtime overhead!
```

---

## 10.2 Traits

### Defining Traits

```rust
trait Summary {
    fn summarize(&self) -> String;
    
    // Default implementation
    fn summary_type(&self) -> &str {
        "Default summary"
    }
    
    // Method with default using other methods
    fn summarize_author(&self) -> String {
        format!("by {}", self.summarize())
    }
}
```

### Implementing Traits

```rust
trait Summary {
    fn summarize(&self) -> String;
    
    fn summary_type(&self) -> &str {
        "Summary"
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    
    // Override default
    fn summary_type(&self) -> &str {
        "News Article"
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Rust 1.75 Released"),
        location: String::from("Internet"),
        author: String::from("Rust Team"),
    };
    
    let tweet = Tweet {
        username: String::from("@rustlang"),
        content: String::from("Hello, world!"),
    };
    
    println!("Article: {}", article.summarize());
    println!("Tweet: {}", tweet.summarize());
    println!("Type: {}", article.summary_type());
}
```

### Trait Bounds

```rust
use std::fmt::Display;

// Single trait bound
fn print_item<T: Display>(item: T) {
    println!("{}", item);
}

// Multiple trait bounds (AND)
fn print_and_debug<T: Display + std::fmt::Debug>(item: T) {
    println!("Display: {}", item);
    println!("Debug: {:?}", item);
}

// where clause (cleaner for complex bounds)
fn complex_function<T, U>(t: T, u: U) -> String
where
    T: Display + Clone,
    U: std::fmt::Debug + Default,
{
    format!("{} {:?}", t, u)
}

fn main() {
    print_item(5);
    print_item("hello");
    
    print_and_debug(42);
    
    let result = complex_function("hello", 42);
    println!("{}", result);
}
```

### impl Trait Syntax

```rust
use std::fmt::Display;

// Return type implementing trait
fn create_greeting(name: &str) -> impl Display {
    format!("Hello, {}!", name)
}

// Multiple types with same trait
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let greeting = create_greeting("Alice");
    println!("{}", greeting);
    
    let numbers = vec![34, 50, 25, 100, 65];
    println!("Largest: {}", largest(&numbers));
    
    let chars = vec!['y', 'm', 'a', 'q'];
    println!("Largest: {}", largest(&chars));
}
```

### Conditional Trait Implementation

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Only implement for T that implements Display + PartialOrd
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("Largest is x = {}", self.x);
        } else {
            println!("Largest is y = {}", self.y);
        }
    }
}

fn main() {
    let pair = Pair::new(5, 10);
    pair.cmp_display();
    
    let pair = Pair::new("hello", "world");
    pair.cmp_display();
}
```

---

## 10.3 Common Standard Library Traits

### Display and Debug

```rust
use std::fmt;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// Custom Display
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// Custom Debug
impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Point")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

fn main() {
    let p = Point { x: 3, y: 4 };
    println!("Display: {}", p);
    println!("Debug: {:?}", p);
    println!("Pretty Debug: {:#?}", p);
}
```

### Clone and Copy

```rust
#[derive(Clone, Debug)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let p1 = Person {
        name: String::from("Alice"),
        age: 30,
    };
    
    let p2 = p1.clone();  // Deep copy
    println!("p1: {:?}", p1);
    println!("p2: {:?}", p2);
}

// Copy trait (implicit clone)
#[derive(Copy, Clone, Debug)]
struct Coordinate {
    x: i32,
    y: i32,
}

fn main() {
    let c1 = Coordinate { x: 5, y: 10 };
    let c2 = c1;  // Copy, not move
    println!("c1: {:?}", c1);  // Still valid
    println!("c2: {:?}", c2);
}
```

### PartialEq and Eq

```rust
#[derive(PartialEq, Eq, Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

fn main() {
    let red = Color { r: 255, g: 0, b: 0 };
    let red2 = Color { r: 255, g: 0, b: 0 };
    let blue = Color { r: 0, g: 0, b: 255 };
    
    println!("red == red2: {}", red == red2);
    println!("red == blue: {}", red == blue);
    println!("red != blue: {}", red != blue);
}
```

### PartialOrd and Ord

```rust
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Score(u32);

fn main() {
    let s1 = Score(85);
    let s2 = Score(90);
    let s3 = Score(85);
    
    println!("s1 < s2: {}", s1 < s2);
    println!("s1 > s2: {}", s1 > s2);
    println!("s1 == s3: {}", s1 == s3);
    println!("s1 <= s3: {}", s1 <= s3);
}
```

### Default

```rust
#[derive(Default, Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
}

fn main() {
    let user = User::default();
    println!("{:?}", user);
    
    // Update specific fields
    let user = User {
        username: String::from("alice"),
        ..Default::default()
    };
    println!("{:?}", user);
}

// Custom Default
#[derive(Debug)]
struct Config {
    host: String,
    port: u16,
    timeout: u32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            host: String::from("localhost"),
            port: 8080,
            timeout: 30,
        }
    }
}
```

### From and Into

```rust
struct Wrapper(String);

// Implement From<String>
impl From<String> for Wrapper {
    fn from(s: String) -> Self {
        Wrapper(s)
    }
}

// Implement From<&str>
impl From<&str> for Wrapper {
    fn from(s: &str) -> Self {
        Wrapper(s.to_string())
    }
}

fn main() {
    // Using From
    let w1 = Wrapper::from(String::from("hello"));
    let w2 = Wrapper::from("hello");
    
    // Using Into (automatically provided)
    let w3: Wrapper = String::from("hello").into();
    let w4: Wrapper = "hello".into();
}
```

---

## 10.4 Trait Objects

### Dynamic Dispatch

```rust
use std::fmt::Display;

struct NewsArticle {
    headline: String,
}

struct Tweet {
    username: String,
}

trait Summary {
    fn summarize(&self) -> String;
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("Headline: {}", self.headline)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("Tweet: {}", self.username)
    }
}

fn main() {
    // Vec of trait objects
    let items: Vec<Box<dyn Summary>> = vec![
        Box::new(NewsArticle { headline: String::from("Rust Released") }),
        Box::new(Tweet { username: String::from("@rustlang") }),
    ];
    
    for item in items {
        println!("{}", item.summarize());
    }
    
    // Reference to trait object
    let article = NewsArticle { headline: String::from("News") };
    let tweet = Tweet { username: String::from("@user") };
    
    let summaries: Vec<&dyn Summary> = vec![&article, &tweet];
    
    for summary in summaries {
        println!("{}", summary.summarize());
    }
}
```

### Static vs Dynamic Dispatch

```rust
// Static dispatch (monomorphization)
fn print_summary_static<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}

// Dynamic dispatch (trait object)
fn print_summary_dynamic(item: &dyn Summary) {
    println!("{}", item.summarize());
}

// Static: faster, larger binary
// Dynamic: flexible, runtime polymorphism
```

---

## 10.5 Advanced Trait Patterns

### Marker Traits

```rust
// Marker trait (no methods)
trait Sendable {}

struct Data(Vec<u8>);
impl Sendable for Data {}

fn send_data<T: Sendable>(data: T) {
    // Send over network
}

// Auto traits (automatically implemented)
// Send: can be transferred across threads
// Sync: can be referenced from multiple threads
```

### Associated Types

```rust
trait Iterator {
    type Item;
    
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    count: u32,
}

impl Iterator for Counter {
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let mut counter = Counter { count: 0 };
    
    while let Some(value) = counter.next() {
        println!("{}", value);
    }
}
```

### Supertraits

```rust
trait OutlinePrint: std::fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("* {} *", output);
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

fn main() {
    let p = Point { x: 1, y: 2 };
    p.outline_print();
}
```

### Newtype Pattern for Traits

```rust
use std::fmt;

// Can't implement trait for type you don't own
// Use newtype pattern

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![
        String::from("hello"),
        String::from("world"),
    ]);
    println!("{}", w);
}
```

---

## Chapter 10 Exercises

### Exercise 10.1: Generic Data Structures
```rust
// Create a generic Stack<T> with:
// - push, pop, peek methods
// - is_empty, len methods
// Test with different types
```

### Exercise 10.2: Custom Traits
```rust
// Define a Drawable trait with draw() method
// Implement for Circle, Rectangle, Triangle
// Store in Vec<Box<dyn Drawable>> and draw all
```

### Exercise 10.3: Trait Bounds
```rust
// Write functions with different trait bounds:
// - Display + Clone
// - PartialOrd + Copy
// - Using where clause for complex bounds
```

### Exercise 10.4: Standard Traits
```rust
// Create a struct and derive/implement:
// - Debug, Display
// - PartialEq, Eq
// - PartialOrd, Ord
// - Clone, Copy
// - Default
```

### Exercise 10.5: Iterator Trait
```rust
// Implement Iterator for a custom type
// Create a Fibonacci iterator
// Practice with associated types
```

---

## Summary

In this chapter, you learned:

✅ Generic functions, structs, and enums
✅ Trait definition and implementation
✅ Trait bounds and where clauses
✅ impl Trait syntax
✅ Common standard library traits
✅ Trait objects and dynamic dispatch
✅ Associated types and supertraits
✅ Newtype pattern for traits
✅ Static vs dynamic dispatch

---

## What's Next?

Traits let you define shared behavior, but how does Rust ensure references are valid? In Chapter 11, we'll explore **Lifetimes** - Rust's way of guaranteeing reference validity.

**Continue to [Chapter 11: Lifetimes](./11_lifetimes.md)**
