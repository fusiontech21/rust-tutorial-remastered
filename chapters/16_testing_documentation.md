# Chapter 16: Testing & Documentation

## 16.1 Writing Tests

### Test Function Basics

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_addition() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    
    #[test]
    fn test_string() {
        let s = String::from("hello");
        assert!(!s.is_empty());
        assert_eq!(s.len(), 5);
    }
}
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_addition

# Run tests matching pattern
cargo test test_

# Run tests in specific file
cargo test --test integration_test

# Show output
cargo test -- --nocapture

# Run ignored tests
cargo test -- --ignored

# Run tests in release mode
cargo test --release

# Show test time
cargo test -- --test-threads=1
```

---

## 16.2 Assertions

### Basic Assertions

```rust
#[test]
fn test_assertions() {
    // assert! - boolean condition
    assert!(true);
    assert!(2 + 2 == 4);
    
    // assert! with custom message
    assert!(2 + 2 == 4, "Math is broken!");
    
    // assert_eq! - equality
    assert_eq!(2 + 2, 4);
    assert_eq!("hello", "hello");
    
    // assert_eq! with message
    assert_eq!(2 + 2, 4, "Addition failed");
    
    // assert_ne! - inequality
    assert_ne!(2 + 2, 5);
    
    // assert!(Result is Ok)
    let result: Result<i32, &str> = Ok(5);
    assert!(result.is_ok());
    
    // assert!(Result is Err)
    let result: Result<i32, &str> = Err("error");
    assert!(result.is_err());
}
```

### Testing Panics

```rust
#[test]
#[should_panic]
fn test_panic() {
    panic!("This test should panic");
}

#[test]
#[should_panic(expected = "specific error")]
fn test_panic_with_message() {
    panic!("specific error occurred");
}

#[test]
fn test_result_no_panic() {
    let result = std::panic::catch_unwind(|| {
        panic!("Test panic");
    });
    assert!(result.is_err());
}
```

---

## 16.3 Test Organization

### Unit Tests

```rust
// In src/lib.rs or src/main.rs

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_add_positive() {
        assert_eq!(add(2, 3), 5);
    }
    
    #[test]
    fn test_add_negative() {
        assert_eq!(add(-2, -3), -5);
    }
    
    #[test]
    fn test_add_mixed() {
        assert_eq!(add(-2, 3), 1);
    }
}
```

### Integration Tests

```rust
// In tests/integration_test.rs

use my_crate::public_function;

#[test]
fn test_public_api() {
    let result = public_function();
    assert_eq!(result, expected_value);
}

// tests/common/mod.rs (shared test utilities)
pub fn setup() {
    // Setup code
}

pub fn teardown() {
    // Teardown code
}
```

### Test Modules Structure

```
my_project/
├── src/
│   ├── lib.rs
│   ├── main.rs
│   └── module.rs      # With #[cfg(test)] mod tests
├── tests/
│   ├── integration_test.rs
│   └── common/
│       └── mod.rs
└── Cargo.toml
```

---

## 16.4 Test Attributes

### Ignoring Tests

```rust
#[test]
#[ignore]
fn slow_test() {
    // This test is ignored by default
}

#[test]
#[ignore = "waiting for fix"]
fn broken_test() {
    // With reason
}

// Run with: cargo test -- --ignored
```

### Should Panic

```rust
#[test]
#[should_panic]
fn test_division_by_zero() {
    let _ = 1 / 0;
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_out_of_bounds() {
    let v = vec![1, 2, 3];
    let _ = v[10];
}
```

### Environment Variables

```rust
#[test]
fn test_with_env() {
    std::env::set_var("TEST_VAR", "value");
    let value = std::env::var("TEST_VAR").unwrap();
    assert_eq!(value, "value");
}
```

---

## 16.5 Parameterized Tests

```rust
#[test]
fn test_addition_cases() {
    let test_cases = vec![
        (0, 0, 0),
        (1, 2, 3),
        (-1, 1, 0),
        (100, 200, 300),
    ];
    
    for (a, b, expected) in test_cases {
        assert_eq!(add(a, b), expected, "add({}, {}) failed", a, b);
    }
}

// Using macro for cleaner syntax
macro_rules! test_case {
    ($name:ident, $a:expr, $b:expr, $expected:expr) => {
        #[test]
        fn $name() {
            assert_eq!(add($a, $b), $expected);
        }
    };
}

test_case!(test_zero, 0, 0, 0);
test_case!(test_positive, 1, 2, 3);
test_case!(test_negative, -1, -1, -2);
```

---

## 16.6 Mocking

### Basic Mocking

```rust
trait Database {
    fn get_user(&self, id: u32) -> Option<String>;
}

struct RealDatabase;
impl Database for RealDatabase {
    fn get_user(&self, id: u32) -> Option<String> {
        // Real implementation
        None
    }
}

struct MockDatabase;
impl Database for MockDatabase {
    fn get_user(&self, id: u32) -> Option<String> {
        Some(format!("User{}", id))
    }
}

fn get_username(db: &dyn Database, id: u32) -> String {
    db.get_user(id).unwrap_or_else(|| "Unknown".to_string())
}

#[test]
fn test_get_username() {
    let mock = MockDatabase;
    let name = get_username(&mock, 1);
    assert_eq!(name, "User1");
}
```

### Using mockall Crate

```rust
// Cargo.toml: mockall = "0.12"

use mockall::*;

#[automock]
trait Database {
    fn get_user(&self, id: u32) -> Option<String>;
}

#[test]
fn test_mock() {
    let mut mock = MockDatabase::new();
    mock.expect_get_user()
        .with(eq(1))
        .times(1)
        .returning(|_| Some("Alice".to_string()));
    
    assert_eq!(mock.get_user(1), Some("Alice".to_string()));
}
```

---

## 16.7 Documentation Comments

### Basic Documentation

```rust
/// Adds two numbers together.
/// 
/// # Arguments
/// 
/// * `a` - The first number
/// * `b` - The second number
/// 
/// # Returns
/// 
/// The sum of `a` and `b`
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
/// 
/// # Errors
/// 
/// Returns an error if inputs are invalid
/// 
/// # Safety
/// 
/// This function is safe to call
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### Documentation Sections

```rust
/// A user account.
/// 
/// # Fields
/// 
/// * `id` - Unique identifier
/// * `name` - User's display name
/// * `email` - User's email address
/// 
/// # Examples
/// 
/// ```
/// let user = User::new(1, "Alice", "alice@example.com");
/// ```
/// 
/// # See Also
/// 
/// * [`User::new()`] for creating users
/// * [`UserRepository`] for persistence
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
}
```

---

## 16.8 Doc Tests

### Testing Examples

```rust
/// Doubles the input value.
/// 
/// ```
/// assert_eq!(double(5), 10);
/// assert_eq!(double(0), 0);
/// assert_eq!(double(-3), -6);
/// ```
pub fn double(x: i32) -> i32 {
    x * 2
}

/// Divides two numbers.
/// 
/// ```should_panic
/// divide(1, 0);
/// ```
/// 
/// ```no_run
/// // This test compiles but doesn't run
/// let result = divide(10, 2);
/// ```
/// 
/// ```ignore
/// // This test is ignored
/// divide(1, 2);
/// ```
pub fn divide(a: i32, b: i32) -> i32 {
    a / b
}
```

### Hidden Setup

```rust
/// Processes a file.
/// 
/// ```
/// # fn setup() { /* hidden setup */ }
/// # fn teardown() { /* hidden teardown */ }
/// # setup();
/// let result = process_file("test.txt");
/// assert!(result.is_ok());
/// # teardown();
/// ```
pub fn process_file(path: &str) -> Result<(), String> {
    Ok(())
}
```

---

## 16.9 Generating Documentation

### cargo doc

```bash
# Generate documentation
cargo doc

# Open in browser
cargo doc --open

# Include private items
cargo doc --private

# No dependencies
cargo doc --no-deps

# With documentation tests
cargo test --doc
```

### Documentation Configuration

```toml
# Cargo.toml
[package]
name = "my_crate"
version = "0.1.0"
edition = "2021"

[dependencies]

[dev-dependencies]
mockall = "0.12"

[[bin]]
name = "my_binary"
path = "src/main.rs"

[lib]
name = "my_crate"
path = "src/lib.rs"
```

---

## 16.10 Test Best Practices

### Naming Conventions

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_function_name() {
        // Basic naming
    }
    
    #[test]
    fn test_specific_case() {
        // Test specific behavior
    }
    
    #[test]
    #[should_panic]
    fn test_error_condition() {
        // Test error handling
    }
}
```

### Arrange-Act-Assert Pattern

```rust
#[test]
fn test_user_creation() {
    // Arrange
    let name = String::from("Alice");
    let email = String::from("alice@example.com");
    
    // Act
    let user = User::new(name, email);
    
    // Assert
    assert_eq!(user.name(), "Alice");
    assert_eq!(user.email(), "alice@example.com");
    assert!(user.is_active());
}
```

### Test Independence

```rust
#[test]
fn test_one() {
    // Each test should be independent
    let mut counter = 0;
    counter += 1;
    assert_eq!(counter, 1);
}

#[test]
fn test_two() {
    // Should not depend on test_one
    let mut counter = 0;
    counter += 1;
    assert_eq!(counter, 1);
}
```

---

## 16.11 Property-Based Testing

### Using proptest

```rust
// Cargo.toml: proptest = "1.4"

use proptest::prelude::*;

fn parse_u32(s: &str) -> Option<u32> {
    s.parse().ok()
}

proptest! {
    #[test]
    fn test_parse_u32(s in "\\PC*") {
        if let Ok(n) = s.parse::<u32>() {
            prop_assert_eq!(parse_u32(&s), Some(n));
        } else {
            prop_assert_eq!(parse_u32(&s), None);
        }
    }
}
```

---

## Chapter 16 Exercises

### Exercise 16.1: Basic Tests
```rust
// Write tests for a calculator:
// - Addition, subtraction
// - Multiplication, division
// - Division by zero handling
```

### Exercise 16.2: Test Organization
```rust
// Create unit tests in source files
// Create integration tests
// Share test utilities
```

### Exercise 16.3: Documentation
```rust
// Add documentation to existing code
// Include examples in doc comments
// Run cargo doc --open
```

### Exercise 16.4: Mocking
```rust
// Create a trait with implementations
// Write tests with mocks
// Use mockall crate
```

### Exercise 16.5: Property Testing
```rust
// Install proptest
// Write property-based tests
// Test sorting, reversing functions
```

---

## Summary

In this chapter, you learned:

✅ Writing test functions with #[test]
✅ Assertions (assert!, assert_eq!, assert_ne!)
✅ Testing panics with should_panic
✅ Unit tests vs integration tests
✅ Test attributes (ignore, should_panic)
✅ Parameterized tests
✅ Mocking with traits and mockall
✅ Documentation comments
✅ Doc tests
✅ Generating documentation with cargo doc
✅ Test best practices
✅ Property-based testing

---

## What's Next?

You've mastered testing! In Chapter 17, we'll explore **Advanced Topics and the Rust Ecosystem** to complete your Rust journey.

**Continue to [Chapter 17: Advanced Topics & Ecosystem](./17_advanced_ecosystem.md)**
