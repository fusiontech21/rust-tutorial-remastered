# Chapter 2: Basic Syntax & Variables

## 2.1 Rust Source File Structure

A typical Rust source file follows this structure:

```rust
// 1. Module declarations
mod utils;
mod helpers;

// 2. External crate imports (pre-2018 edition)
// extern crate some_crate;

// 3. Use statements (imports)
use std::collections::HashMap;
use std::io::{self, Read, Write};

// 4. Constants
const MAX_SIZE: usize = 100;
const PI: f64 = 3.14159265359;

// 5. Static variables (rarely used)
static mut COUNTER: u32 = 0;

// 6. Structs, Enums, Traits
struct Person {
    name: String,
    age: u32,
}

enum Status {
    Active,
    Inactive,
}

trait Greet {
    fn greet(&self) -> String;
}

// 7. Function implementations
fn main() {
    // Program entry point
}

// 8. Helper functions
fn helper_function() {
    // Implementation
}

// 9. Trait implementations
impl Greet for Person {
    fn greet(&self) -> String {
        format!("Hello, I'm {}", self.name)
    }
}

// 10. Tests (at the bottom)
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_something() {
        assert_eq!(2 + 2, 4);
    }
}
```

---

## 2.2 Comments

### Line Comments

```rust
// This is a single-line comment
let x = 5; // Inline comment

// Multi-line comments use multiple //
// This is line 1
// This is line 2
// This is line 3
```

### Block Comments

```rust
/* This is a block comment
   It can span multiple lines
   /* Nested block comments work too! */
*/
```

### Documentation Comments

```rust
/// Documentation comment for a function
/// These generate HTML documentation with `cargo doc`
/// 
/// # Examples
/// 
/// ```
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
/// 
/// # Panics
/// 
/// Panics if overflow occurs
fn add(a: i32, b: i32) -> i32 {
    a + b
}

//! Module-level documentation comment
//! This describes the entire module

// Doc comments support Markdown formatting:
/// - **Bold text**
/// - *Italic text*
/// - `inline code`
/// - [Links](https://rust-lang.org)
/// 
/// ## Code blocks
/// 
/// ```rust
/// let x = 42;
/// ```
```

---

## 2.3 Variables and Mutability

### Immutable Variables (Default)

```rust
fn main() {
    let x = 5;  // x is immutable
    println!("x = {}", x);
    
    // x = 6;  // ERROR: cannot assign twice to immutable variable
}
```

### Mutable Variables

```rust
fn main() {
    let mut y = 10;  // y is mutable
    println!("y = {}", y);
    
    y = 15;  // OK: y is mutable
    println!("y = {}", y);
    
    y = y + 5;  // OK
    println!("y = {}", y);  // Output: 20
}
```

### Variable Shadowing

```rust
fn main() {
    let x = 5;      // x is i32
    let x = x + 1;  // x is still i32, value is 6
    let x = x * 2;  // x is still i32, value is 12
    
    println!("x = {}", x);  // Output: 12
    
    // Shadowing can change types!
    let spaces = "   ";      // spaces is &str
    let spaces = spaces.len(); // spaces is now usize
    let spaces = spaces as f64; // spaces is now f64
    
    println!("spaces = {}", spaces);  // Output: 3
}
```

**Shadowing vs Mutability:**

| Feature | Mutable (`mut`) | Shadowing |
|---------|-----------------|-----------|
| Change value | ✅ | ✅ |
| Change type | ❌ | ✅ |
| Must initialize | ❌ | ✅ |
| Creates new variable | ❌ | ✅ |

---

## 2.4 Constants

Constants are like immutable variables but with key differences:

```rust
// Must have type annotation
const MAX_POINTS: u32 = 100_000;

// Can be declared in any scope (including global)
const PI: f64 = 3.14159265359;

// Constants must be compile-time constants
const SECONDS_PER_MINUTE: u32 = 60;
const MINUTES_PER_HOUR: u32 = 60;
const SECONDS_PER_HOUR: u32 = SECONDS_PER_MINUTE * MINUTES_PER_HOUR;

// Cannot use const for runtime values
// const WRONG: u32 = some_function();  // ERROR!

fn main() {
    println!("Max points: {}", MAX_POINTS);
    println!("Pi: {}", PI);
    println!("Seconds per hour: {}", SECONDS_PER_HOUR);
}
```

**Constants vs Immutable Variables:**

| Feature | `const` | `let` (immutable) |
|---------|---------|-------------------|
| Type annotation | Required | Optional (inferred) |
| Must be constant expression | ✅ | ❌ |
| Can be mutated | ❌ | ❌ (but can use `mut`) |
| Lives for entire program | ✅ | ❌ (scoped) |
| Inlined at compile time | ✅ | ❌ |
| Naming convention | SCREAMING_SNAKE_CASE | snake_case |

---

## 2.5 Static Variables

```rust
// Static variables live for the entire program duration
// They have a fixed memory address
static HELLO_WORLD: &str = "Hello, world!";

// Mutable statics are unsafe (require unsafe block)
static mut COUNTER: u32 = 0;

fn main() {
    println!("{}", HELLO_WORLD);
    
    // Accessing mutable static requires unsafe
    unsafe {
        COUNTER += 1;
        println!("Counter: {}", COUNTER);
    }
}
```

⚠️ **Warning**: Mutable statics are rarely needed and can cause data races. Prefer other approaches like `std::sync::Mutex` or `std::sync::atomic`.

---

## 2.6 Data Types Overview

Rust is statically typed, meaning every variable has a known type at compile time.

### Type Annotation Syntax

```rust
let variable_name: Type = value;

let x: i32 = 42;
let name: &str = "Alice";
let numbers: Vec<i32> = vec![1, 2, 3];
```

### Type Inference

Rust can often infer types:

```rust
let x = 42;           // Inferred as i32
let y = 3.14;         // Inferred as f64
let name = "Bob";     // Inferred as &str
let is_rust = true;   // Inferred as bool
```

When you need to specify the type explicitly:

```rust
// Using type annotation
let x: i32 = 42;

// Using turbofish syntax for generics
let numbers = vec![1, 2, 3];           // Vec<i32> inferred
let numbers: Vec<i32> = vec![];        // Explicit type
let numbers = Vec::<i32>::new();       // Turbofish
```

---

## 2.7 Functions

### Basic Function Syntax

```rust
// Function with no parameters and no return value
fn greet() {
    println!("Hello!");
}

// Function with parameters
fn add(a: i32, b: i32) -> i32 {
    a + b  // No semicolon = return value
}

// Function with multiple parameters of same type
fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

// Function with no return value (returns unit type ())
fn print_sum(a: i32, b: i32) {
    println!("Sum: {}", a + b);
}

fn main() {
    greet();
    
    let result = add(5, 3);
    println!("5 + 3 = {}", result);
    
    print_sum(10, 20);
}
```

### Expression vs Statement

```rust
fn main() {
    // Statement: performs action, returns nothing (unit type)
    let x = 5;  // This is a statement
    
    // Expression: evaluates to a value
    let y = {   // This block is an expression
        let a = 10;
        let b = 20;
        a + b   // No semicolon = this is the return value
    };
    
    println!("y = {}", y);  // Output: 30
    
    // Function calls are expressions
    let z = add(3, 4);  // add() returns a value
}
```

### Early Return

```rust
fn find_first_even(numbers: &[i32]) -> Option<i32> {
    for &n in numbers {
        if n % 2 == 0 {
            return Some(n);  // Early return
        }
    }
    None  // Implicit return (no semicolon)
}

fn main() {
    let nums = [1, 3, 5, 8, 9];
    match find_first_even(&nums) {
        Some(n) => println!("First even: {}", n),
        None => println!("No even numbers"),
    }
}
```

---

## 2.8 Printing and Formatting

### Basic Printing

```rust
fn main() {
    // println! prints with newline
    println!("Hello, Rust!");
    
    // print! prints without newline
    print!("No newline");
    print!(" continues here\n");
    
    // eprintln! prints to stderr
    eprintln!("This is an error message");
    
    // eprint! prints to stderr without newline
    eprint!("Error: ");
    eprintln!("Something went wrong");
}
```

### Format Specifiers

```rust
fn main() {
    let x = 42;
    let y = 3.14159;
    let name = "Alice";
    
    // {} - Default formatting
    println!("Number: {}", x);
    println!("Float: {}", y);
    println!("Name: {}", name);
    
    // {:?} - Debug formatting
    let point = (3, 4);
    println!("Point: {:?}", point);
    
    // {:#?} - Pretty debug formatting
    println!("Point (pretty): {:#?}", point);
    
    // {:.N} - Precision for floats
    println!("Pi to 2 decimals: {:.2}", y);
    println!("Pi to 4 decimals: {:.4}", y);
    
    // {:0N} - Zero padding
    println!("Padded: {:05}", x);  // Output: 00042
    
    // {:N} - Width
    println!("Width: {:10}", x);   // Output: "        42"
    
    // {:<N}, {:>N}, {:^N} - Alignment
    println!("Left:  {:<10}", x);  // Output: "42        "
    println!("Right: {:>10}", x);  // Output: "        42"
    println!("Center:{:^10}", x);  // Output: "    42    "
    
    // {:b}, {:o}, {:x}, {:X} - Number bases
    println!("Binary: {:b}", x);   // Output: 101010
    println!("Octal: {:o}", x);    // Output: 52
    println!("Hex: {:x}", x);      // Output: 2a
    println!("Hex: {:X}", x);      // Output: 2A
    
    // {:p} - Pointer address
    let ptr = &x;
    println!("Address: {:p}", ptr);
    
    // {} multiple values
    println!("{} + {} = {}", 2, 3, 2 + 3);
    
    // Positional arguments
    println!("{0}, {1}, {0}", "Alice", "Bob");
    
    // Named arguments
    println!("{name} is {age} years old", name = "Carol", age = 30);
}
```

### Capturing Output

```rust
use std::fmt::Write;

fn main() {
    // format! returns a String instead of printing
    let message = format!("Hello, {}!", "World");
    println!("{}", message);
    
    // Useful for creating strings
    let x = 5;
    let y = 10;
    let equation = format!("{} + {} = {}", x, y, x + y);
    println!("{}", equation);
}
```

---

## 2.9 Scope and Blocks

```rust
fn main() {
    // Outer scope
    let outer = "I'm outside";
    
    {
        // Inner scope
        let inner = "I'm inside";
        println!("{}", outer);  // Can access outer
        println!("{}", inner);  // Can access inner
    }
    
    println!("{}", outer);  // Can still access outer
    // println!("{}", inner);  // ERROR: inner is out of scope
    
    // Blocks are expressions
    let result = {
        let a = 10;
        let b = 20;
        a + b  // No semicolon = block returns this value
    };
    
    println!("Result: {}", result);  // Output: 30
}
```

---

## 2.10 Naming Conventions

| Item | Convention | Example |
|------|------------|---------|
| Variables | snake_case | `user_name`, `total_count` |
| Functions | snake_case | `calculate_total`, `get_user` |
| Structs | PascalCase | `UserProfile`, `HttpRequest` |
| Enums | PascalCase | `Option`, `Result`, `Status` |
| Traits | PascalCase | `Iterator`, `Clone`, `Display` |
| Constants | SCREAMING_SNAKE_CASE | `MAX_SIZE`, `PI` |
| Statics | SCREAMING_SNAKE_CASE | `DATABASE_URL` |
| Modules | snake_case | `user_handler`, `db_utils` |
| Types (type alias) | PascalCase | `NodeId`, `UserId` |

---

## Chapter 2 Exercises

### Exercise 2.1: Variables and Mutability
```rust
// Create a program that:
// 1. Declares an immutable variable with value 10
// 2. Declares a mutable variable with value 5
// 3. Adds them together and stores in a new variable
// 4. Prints the result
// 5. Try to modify the immutable variable and observe the error
```

### Exercise 2.2: Shadowing Practice
```rust
// Create a program that demonstrates shadowing:
// 1. Start with a String: "   Hello   "
// 2. Shadow to get the length (usize)
// 3. Shadow to multiply by 2 (still usize)
// 4. Shadow to convert to f64
// 5. Print the final value
```

### Exercise 2.3: Function Writing
```rust
// Write functions for:
// 1. Convert Celsius to Fahrenheit: F = C * 9/5 + 32
// 2. Calculate factorial of a number
// 3. Check if a number is prime
// 4. Find the maximum of three numbers
// Test each function with multiple inputs
```

### Exercise 2.4: Formatting Challenge
```rust
// Create a formatted table output:
// Product     Price    Quantity
// Apple       $1.50    10
// Banana      $0.75    25
// Orange      $2.00    15
// Use proper alignment and formatting specifiers
```

### Exercise 2.5: Scope Exploration
```rust
// Create nested scopes demonstrating:
// 1. Variable shadowing across scopes
// 2. Variables going out of scope
// 3. Block expressions returning values
// 4. Access variables from outer scopes in inner scopes
```

---

## Common Pitfalls

### Forgetting `mut`

```rust
// ❌ WRONG
let x = 5;
x = 10;  // ERROR: x is not mutable

// ✅ CORRECT
let mut x = 5;
x = 10;  // OK
```

### Semicolon Confusion

```rust
// ❌ WRONG - returns () instead of i32
fn add(a: i32, b: i32) -> i32 {
    a + b;  // Semicolon makes this a statement, returns ()
}

// ✅ CORRECT
fn add(a: i32, b: i32) -> i32 {
    a + b  // No semicolon = return value
}
```

### Type Inference Limitations

```rust
// ❌ WRONG - Rust can't infer the type
let x = 5.0;
let y = x.sqrt();  // ERROR: ambiguous type

// ✅ CORRECT
let x: f64 = 5.0;
let y = x.sqrt();  // OK
```

---

## Summary

In this chapter, you learned:

✅ Rust source file structure and organization
✅ Comments (line, block, documentation)
✅ Variables and mutability (`let` vs `let mut`)
✅ Variable shadowing and when to use it
✅ Constants and static variables
✅ Type annotations and type inference
✅ Function syntax and return values
✅ Expression vs statement distinction
✅ Printing and formatting with macros
✅ Scope rules and blocks
✅ Rust naming conventions

---

## What's Next?

Now that you understand Rust's basic syntax, let's dive deep into Rust's type system! In Chapter 3, we'll explore all of Rust's built-in data types in detail.

**Continue to [Chapter 3: Data Types](./03_data_types.md)**
