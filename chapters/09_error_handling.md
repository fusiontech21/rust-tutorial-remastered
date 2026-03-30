# Chapter 9: Error Handling

## 9.1 Unrecoverable Errors with panic!

### When to Use panic!

```rust
fn main() {
    // Explicit panic
    panic!("Something went wrong!");
    
    // Out of bounds access (causes panic)
    let v = vec![1, 2, 3];
    // let x = v[99];  // Panics!
    
    // Division by zero (panics in debug, undefined in release)
    // let x = 1 / 0;
}
```

### Backtrace

```rust
// Set RUST_BACKTRACE=1 to see full backtrace
// export RUST_BACKTRACE=1

fn main() {
    panic!("Error with backtrace");
}
```

Output:
```
thread 'main' panicked at 'Error with backtrace', src/main.rs:4:5
stack backtrace:
   0: rust_begin_unwind
   1: core::panicking::panic_fmt
   2: main
   ...
```

### Using panic! in Production

```rust
// Generally avoid panic! in library code
// Use Result instead

// Acceptable uses of panic!:
// - Examples/prototypes
// - Unrecoverable bugs
// - Testing
// - When contract is violated (e.g., index out of bounds)

fn get_first(v: &[i32]) -> i32 {
    // Better to return Option
    v.first().copied().unwrap_or(0)
}

fn get_first_panic(v: &[i32]) -> i32 {
    // Only if empty slice is a bug
    v.first().copied().expect("Vector must not be empty")
}
```

---

## 9.2 Recoverable Errors with Result

### Result Enum

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### Basic Result Usage

```rust
use std::fs::File;
use std::io::Error;

fn main() {
    let file_result = File::open("hello.txt");
    
    match file_result {
        Ok(file) => {
            println!("File opened successfully");
            // Use file
        }
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}
```

### Handling Different Errors

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let file_result = File::open("hello.txt");
    
    let file = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                // Create file if not found
                File::create("hello.txt").expect("Failed to create file")
            }
            other_error => {
                panic!("Error opening file: {:?}", other_error);
            }
        },
    };
}
```

---

## 9.3 Shortcuts: unwrap, expect, and ?

### unwrap

```rust
use std::fs::File;

fn main() {
    // unwrap: returns value or panics
    let file = File::open("hello.txt").unwrap();
    
    // Equivalent to:
    let file = match File::open("hello.txt") {
        Ok(f) => f,
        Err(e) => panic!("Error: {:?}", e),
    };
}
```

### expect

```rust
use std::fs::File;

fn main() {
    // expect: like unwrap but with custom message
    let file = File::open("hello.txt")
        .expect("Failed to open hello.txt");
}
```

### The ? Operator

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_file() -> Result<String, io::Error> {
    let mut file = File::open("hello.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// Chaining ?
fn read_file_short() -> Result<String, io::Error> {
    let mut contents = String::new();
    File::open("hello.txt")?.read_to_string(&mut contents)?;
    Ok(contents)
}

// With ? in main
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string("hello.txt")?;
    println!("{}", contents);
    Ok(())
}
```

### When to Use ?

```rust
// Use ? when:
// - You want to propagate errors up
// - The error type matches or can be converted

// Don't use ? when:
// - You need to handle the error locally
// - You're in a function that doesn't return Result

fn process_file() -> Result<(), Box<dyn std::error::Error>> {
    // Good use of ?
    let contents = std::fs::read_to_string("config.txt")?;
    
    // Handle error locally
    let value = match contents.parse::<i32>() {
        Ok(v) => v,
        Err(_) => 0,  // Default value
    };
    
    Ok(())
}
```

---

## 9.4 Custom Error Types

### Basic Custom Error

```rust
use std::fmt;
use std::error::Error;

#[derive(Debug)]
enum AppError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
    NotFound(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "IO error: {}", e),
            AppError::Parse(e) => write!(f, "Parse error: {}", e),
            AppError::NotFound(item) => write!(f, "Not found: {}", item),
        }
    }
}

impl Error for AppError {}

// Implement From for automatic conversion
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> AppError {
        AppError::Io(err)
    }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(err: std::num::ParseIntError) -> AppError {
        AppError::Parse(err)
    }
}

fn read_config() -> Result<i32, AppError> {
    let contents = std::fs::read_to_string("config.txt")?;
    let value: i32 = contents.trim().parse()?;
    Ok(value)
}

fn main() {
    match read_config() {
        Ok(value) => println!("Config value: {}", value),
        Err(e) => println!("Error: {}", e),
    }
}
```

### Using thiserror Crate

```rust
// Add to Cargo.toml: thiserror = "1.0"

use thiserror::Error;

#[derive(Error, Debug)]
enum DataError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Parse error: {0}")]
    Parse(#[from] std::num::ParseIntError),
    
    #[error("Item not found: {0}")]
    NotFound(String),
    
    #[error("Invalid input: {field} - {message}")]
    InvalidInput { field: String, message: String },
}

fn process_data() -> Result<(), DataError> {
    let contents = std::fs::read_to_string("data.txt")?;
    let value: i32 = contents.trim().parse()?;
    
    if value < 0 {
        return Err(DataError::InvalidInput {
            field: "value".to_string(),
            message: "Must be positive".to_string(),
        });
    }
    
    Ok(())
}
```

### Using anyhow Crate

```rust
// Add to Cargo.toml: anyhow = "1.0"

use anyhow::{Result, Context, anyhow, bail};

fn read_config() -> Result<i32> {
    let contents = std::fs::read_to_string("config.txt")
        .context("Failed to read config file")?;
    
    let value: i32 = contents.trim().parse()
        .context("Failed to parse config value")?;
    
    if value < 0 {
        bail!("Config value must be positive, got {}", value);
    }
    
    Ok(value)
}

fn main() -> Result<()> {
    let value = read_config()
        .context("Failed to load configuration")?;
    
    println!("Config: {}", value);
    Ok(())
}
```

---

## 9.5 Error Handling Patterns

### Option to Result

```rust
fn main() {
    let opt = Some(5);
    
    // Option to Result
    let result: Result<i32, &str> = opt.ok_or("No value");
    
    // With custom error
    let result: Result<i32, String> = opt.ok_or_else(|| "No value".to_string());
    
    // unwrap_or default
    let value = opt.unwrap_or(0);
    
    // unwrap_or_else
    let value = opt.unwrap_or_else(|| {
        println!("Using default");
        0
    });
    
    // unwrap_or_default
    let value: Option<String> = None;
    let s = value.unwrap_or_default();  // ""
}
```

### Result Combinators

```rust
fn main() {
    let result: Result<i32, &str> = Ok(5);
    
    // map
    let doubled = result.map(|x| x * 2);  // Ok(10)
    
    // map_err
    let result: Result<i32, String> = Err("error".to_string());
    let logged = result.map_err(|e| {
        println!("Error: {}", e);
        e
    });
    
    // and_then (flat_map)
    let result: Result<i32, &str> = Ok(5);
    let chained = result.and_then(|x| {
        if x > 0 {
            Ok(x * 2)
        } else {
            Err("Must be positive")
        }
    });
    
    // or_else
    let result: Result<i32, &str> = Err("first error");
    let recovered = result.or_else(|e| {
        println!("Recovering from: {}", e);
        Ok(0)
    });
    
    // map_or, map_or_else
    let result: Result<i32, &str> = Ok(5);
    let value = result.map_or(0, |x| x * 2);  // 10
    
    let value = result.map_or_else(
        |e| 0,  // Default if error
        |x| x * 2,  // Transform if Ok
    );
}
```

### Multiple Errors with ?

```rust
use std::fs;
use std::io;

fn process_files() -> Result<(), Box<dyn std::error::Error>> {
    // All errors convert to Box<dyn Error>
    let content1 = fs::read_to_string("file1.txt")?;
    let content2 = fs::read_to_string("file2.txt")?;
    
    println!("{} + {}", content1, content2);
    Ok(())
}

// Collect multiple Results
fn parse_all_numbers() -> Result<Vec<i32>, std::num::ParseIntError> {
    let strings = vec!["1", "2", "3", "not_a_number"];
    
    // This returns first error encountered
    let numbers: Result<Vec<i32>, _> = strings
        .iter()
        .map(|s| s.parse::<i32>())
        .collect();
    
    numbers
}
```

---

## 9.6 When to Use panic! vs Result

```rust
// Use Result when:
// - Error is expected and recoverable
// - Writing library code
// - Caller should decide how to handle error

fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse()
}

// Use panic! when:
// - Bug in the program (invariant violated)
// - Example/prototype code
// - Error is truly unrecoverable

fn get_element(v: &[i32], index: usize) -> i32 {
    // This is a bug if index is out of bounds
    v[index]  // Will panic with clear message
}

// Use expect when:
// - Operation should never fail
// - Default handling is appropriate

fn get_config() -> String {
    std::env::var("CONFIG_PATH")
        .expect("CONFIG_PATH must be set")
}
```

---

## 9.7 Advanced Error Handling

### Error Context

```rust
use std::fs;
use std::io;

#[derive(Debug)]
struct Config {
    value: i32,
}

fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let path = std::env::var("CONFIG_PATH")
        .unwrap_or_else(|_| "config.txt".to_string());
    
    let contents = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read {}: {}", path, e))?;
    
    let value: i32 = contents.trim().parse()
        .map_err(|e| format!("Invalid config value: {}", e))?;
    
    Ok(Config { value })
}
```

### Retry Pattern

```rust
use std::{thread, time::Duration};
use std::io::{self, Read};
use std::fs::File;

fn read_with_retry(path: &str, max_retries: u32) -> Result<String, io::Error> {
    let mut last_error = None;
    
    for attempt in 0..max_retries {
        match File::open(path) {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;
                return Ok(contents);
            }
            Err(e) => {
                last_error = Some(e);
                if attempt < max_retries - 1 {
                    thread::sleep(Duration::from_secs(1));
                }
            }
        }
    }
    
    Err(last_error.unwrap())
}
```

---

## Chapter 9 Exercises

### Exercise 9.1: Custom Error Type
```rust
// Create a custom error type for a banking application:
// - InsufficientFunds
// - InvalidAccount
// - TransactionLimit
// Implement Display and Error traits
```

### Exercise 9.2: Error Propagation
```rust
// Create a function chain that:
// - Reads a file
// - Parses JSON
// - Validates data
// - Returns custom error type
// Use ? operator throughout
```

### Exercise 9.3: Result Combinators
```rust
// Practice using:
// - map, map_err
// - and_then, or_else
// - unwrap_or, unwrap_or_else
// - ok_or, ok_or_else
```

### Exercise 9.4: anyhow and thiserror
```rust
// Rewrite error handling using:
// - thiserror for library code
// - anyhow for application code
// Compare the two approaches
```

---

## Summary

In this chapter, you learned:

✅ panic! for unrecoverable errors
✅ Result<T, E> for recoverable errors
✅ unwrap, expect, and ? operators
✅ Custom error types
✅ Implementing Display and Error traits
✅ From trait for error conversion
✅ Error handling patterns and combinators
✅ When to use panic! vs Result
✅ Using anyhow and thiserror crates

---

## What's Next?

Now that you can handle errors like a pro, let's explore Rust's powerful abstraction mechanisms! In Chapter 10, we'll dive into **Generics and Traits**.

**Continue to [Chapter 10: Generics & Traits](./10_generics_traits.md)**
