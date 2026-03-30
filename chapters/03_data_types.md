# Chapter 3: Data Types

## 3.1 Type System Overview

Rust is a **statically typed** language with **type inference**. Every value in Rust has a specific type that is known at compile time.

### Type Categories

```
Rust Types
├── Scalar Types (single values)
│   ├── Integers (i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize)
│   ├── Floating-point (f32, f64)
│   ├── Boolean (bool)
│   └── Character (char)
└── Compound Types (multiple values)
    ├── Tuples (fixed-length collection)
    └── Arrays (fixed-length, same-type collection)
```

---

## 3.2 Integer Types

### Signed Integers

| Type | Size | Minimum Value | Maximum Value |
|------|------|---------------|---------------|
| i8 | 8-bit | -128 | 127 |
| i16 | 16-bit | -32,768 | 32,767 |
| i32 | 32-bit | -2,147,483,648 | 2,147,483,647 |
| i64 | 64-bit | -9,223,372,036,854,775,808 | 9,223,372,036,854,775,807 |
| i128 | 128-bit | -170,141,183,460,469,231,731,687,303,715,884,105,728 | 170,141,183,460,469,231,731,687,303,715,884,105,727 |
| isize | Platform-dependent | -2^(N-1) | 2^(N-1) - 1 |

### Unsigned Integers

| Type | Size | Minimum Value | Maximum Value |
|------|------|---------------|---------------|
| u8 | 8-bit | 0 | 255 |
| u16 | 16-bit | 0 | 65,535 |
| u32 | 32-bit | 0 | 4,294,967,295 |
| u64 | 64-bit | 0 | 18,446,744,073,709,551,615 |
| u128 | 128-bit | 0 | 340,282,366,920,938,463,463,374,607,431,768,211,455 |
| usize | Platform-dependent | 0 | 2^N - 1 |

### Usage Examples

```rust
fn main() {
    // Default integer type is i32
    let x = 42;           // i32
    
    // Explicit type annotations
    let a: i8 = 127;
    let b: i16 = 32_767;
    let c: i32 = 2_147_483_647;
    let d: i64 = 9_223_372_036_854_775_807;
    let e: i128 = 170_141_183_460_469_231_731_687_303_715_884_105_727;
    
    // Unsigned integers
    let f: u8 = 255;
    let g: u32 = 4_294_967_295;
    
    // Platform-dependent size (pointer-sized)
    let h: usize = 100;  // Size depends on architecture (32-bit vs 64-bit)
    
    // Underscores for readability
    let one_million: i32 = 1_000_000;
    let credit_card: u64 = 1234_5678_9012_3456;
    
    println!("x = {}, a = {}, one_million = {}", x, a, one_million);
}
```

### Integer Overflow

```rust
fn main() {
    // Debug mode: Panics on overflow
    let x: u8 = 255;
    // let y = x + 1;  // Panics in debug mode!
    
    // Release mode: Wraps around (2's complement)
    
    // Explicit wrapping behavior
    let a: u8 = 255;
    let b = a.wrapping_add(1);  // b = 0
    println!("Wrapping: 255 + 1 = {}", b);
    
    // Checked arithmetic (returns Option)
    let c: u8 = 255;
    match c.checked_add(1) {
        Some(result) => println!("Result: {}", result),
        None => println!("Overflow occurred!"),
    }
    
    // Saturating arithmetic (clamps to max/min)
    let d: u8 = 255;
    let e = d.saturating_add(1);  // e = 255 (saturates at max)
    println!("Saturating: 255 + 1 = {}", e);
    
    // Using wrapping in release mode explicitly
    let f: u8 = 200;
    let g = f.wrapping_mul(2);  // Wraps around
    println!("200 * 2 (wrapping) = {}", g);
}
```

---

## 3.3 Floating-Point Types

### f32 and f64

| Type | Size | Precision | Use Case |
|------|------|-----------|----------|
| f32 | 32-bit | ~7 decimal digits | Memory-constrained, GPU |
| f64 | 64-bit | ~15 decimal digits | Default, most calculations |

```rust
fn main() {
    // Default float type is f64
    let x = 3.14;         // f64
    
    // Explicit types
    let a: f32 = 3.14159;
    let b: f64 = 3.14159265358979;
    
    // Scientific notation
    let c: f64 = 1.0e6;   // 1,000,000
    let d: f64 = 1.0e-6;  // 0.000001
    
    // Special values
    let infinity = f64::INFINITY;
    let neg_infinity = f64::NEG_INFINITY;
    let nan = f64::NAN;
    
    println!("Pi: {}", b);
    println!("Infinity: {}", infinity);
    println!("One million: {}", c);
    
    // Float operations
    let e = 10.0 / 3.0;
    println!("10 / 3 = {}", e);
    
    // Math methods
    let f = 4.0;
    println!("sqrt(4) = {}", f.sqrt());
    println!("4^2 = {}", f.powi(2));
    println!("4^2.5 = {}", f.powf(2.5));
    println!("abs(-5) = {}", (-5.0).abs());
    println!("floor(3.7) = {}", 3.7_f64.floor());
    println!("ceil(3.2) = {}", 3.2_f64.ceil());
    println!("round(3.5) = {}", 3.5_f64.round());
}
```

### Floating-Point Comparison

```rust
fn main() {
    // ⚠️ Never compare floats for exact equality!
    let a = 0.1 + 0.2;
    let b = 0.3;
    
    println!("0.1 + 0.2 = {}", a);  // 0.30000000000000004
    println!("0.3 = {}", b);
    println!("Are they equal? {}", a == b);  // false!
    
    // ✅ Compare with epsilon
    let epsilon = f64::EPSILON;  // ~2.22e-16
    let are_equal = (a - b).abs() < epsilon;
    println!("Approximately equal? {}", are_equal);
    
    // For most cases, use a reasonable epsilon
    let epsilon = 1e-10;
    let are_equal = (a - b).abs() < epsilon;
    println!("Approximately equal (1e-10)? {}", are_equal);
}
```

---

## 3.4 Boolean Type

```rust
fn main() {
    // Boolean literals
    let t: bool = true;
    let f: bool = false;
    
    // Boolean operations
    let a = true && false;  // AND: false
    let b = true || false;  // OR: true
    let c = !true;          // NOT: false
    
    // Comparison operators return bool
    let x = 5;
    let y = 10;
    
    let eq = x == y;    // false
    let ne = x != y;    // true
    let lt = x < y;     // true
    let gt = x > y;     // false
    let le = x <= y;    // true
    let ge = x >= y;    // false
    
    println!("t={}, f={}, a={}, b={}, c={}", t, f, a, b, c);
    println!("Comparisons: eq={}, ne={}, lt={}, gt={}, le={}, ge={}", 
             eq, ne, lt, gt, le, ge);
    
    // Booleans in control flow
    let is_rust_great = true;
    if is_rust_great {
        println!("Rust is great!");
    }
}
```

---

## 3.5 Character Type

```rust
fn main() {
    // Characters are Unicode scalar values (4 bytes)
    let c: char = 'a';
    let emoji: char = '🦀';  // Rust crab emoji!
    let heart: char = '❤';
    let japanese: char = 'あ';
    
    // Single quotes for char, double quotes for strings
    // let wrong = "a";  // This is &str, not char
    
    // Escape sequences
    let newline: char = '\n';
    let tab: char = '\t';
    let backslash: char = '\\';
    let single_quote: char = '\'';
    let unicode: char = '\u{1F980}';  // Crab emoji using Unicode escape
    
    // Character operations
    let letter = 'A';
    println!("Character: {}", letter);
    println!("Is alphabetic: {}", letter.is_alphabetic());
    println!("Is numeric: {}", letter.is_numeric());
    println!("Is whitespace: {}", letter.is_whitespace());
    
    // Convert to uppercase/lowercase
    for c in letter.to_uppercase() {
        println!("Uppercase: {}", c);
    }
    
    // Get numeric value
    println!("Unicode code point: {}", 'A' as u32);  // 65
    
    // Iterate over characters in a string
    for c in "Hello".chars() {
        println!("Char: {}", c);
    }
    
    println!("Emoji: {}", emoji);
    println!("Unicode: {}", unicode);
}
```

**char vs u8:**

```rust
fn main() {
    // u8 is a single byte (0-255)
    let byte: u8 = b'A';  // Byte literal
    println!("Byte: {}", byte);  // 65
    
    // char is a Unicode scalar value (4 bytes)
    let ch: char = 'A';
    println!("Char: {}", ch);  // A
    
    // char can represent any Unicode character
    let emoji: char = '🦀';  // 4 bytes, but one character
    println!("Emoji size: {} bytes", std::mem::size_of::<char>());  // 4
    
    // Conversion
    let byte_to_char = byte as char;
    println!("Byte to char: {}", byte_to_char);
    
    let char_to_byte = 'A' as u8;
    println!("Char to byte: {}", char_to_byte);
}
```

---

## 3.6 Tuple Types

Tuples group a fixed number of values with potentially different types.

```rust
fn main() {
    // Creating tuples
    let tuple: (i32, f64, &str) = (50, 3.14, "hello");
    
    // Type inference works
    let t = (1, 2.0, "three", true);
    
    // Accessing elements
    let first = t.0;   // 1
    let second = t.1;  // 2.0
    let third = t.2;   // "three"
    let fourth = t.3;  // true
    
    // Destructuring tuples
    let (a, b, c, d) = t;
    println!("a={}, b={}, c={}, d={}", a, b, c, d);
    
    // Partial destructuring
    let (x, _, _, z) = t;  // Ignore middle values with _
    println!("x={}, z={}", x, z);
    
    // Nested tuples
    let nested = ((1, 2), (3, 4));
    let val = nested.0.1;  // 2
    
    // Tuple with single element (unit type)
    let single = (42,);  // Note the comma!
    let not_tuple = (42);  // Just 42 with parentheses
    
    // Empty tuple is the unit type
    let unit: () = ();
    
    // Tuples can be compared (if elements implement comparison)
    let t1 = (1, 2, 3);
    let t2 = (1, 2, 3);
    println!("t1 == t2: {}", t1 == t2);  // true
    
    // Tuples can be returned from functions
    let result = get_coordinates();
    println!("Coordinates: {:?}", result);
}

fn get_coordinates() -> (i32, i32) {
    (10, 20)
}
```

### Named Tuples (Tuple Structs)

```rust
// Tuple structs have named fields but use tuple syntax
struct Color(u8, u8, u8);
struct Point(f64, f64, f64);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0.0, 0.0, 0.0);
    
    // Access fields by index
    println!("Red component: {}", black.0);
    println!("X coordinate: {}", origin.0);
    
    // Destructure
    let Color(r, g, b) = black;
    println!("RGB: ({}, {}, {})", r, g, b);
    
    let Point(x, y, z) = origin;
    println!("Point: ({}, {}, {})", x, y, z);
}
```

---

## 3.7 Array Types

Arrays are fixed-size collections of elements of the same type.

```rust
fn main() {
    // Array with explicit type
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    
    // Type inference
    let fruits = ["apple", "banana", "orange"];
    
    // Initialize all elements to same value
    let zeros = [0; 10];  // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    let ones = [1; 5];    // [1, 1, 1, 1, 1]
    
    // Accessing elements
    let first = numbers[0];
    let third = numbers[2];
    
    // Last element
    let last = numbers[numbers.len() - 1];
    
    // Array length
    let len = numbers.len();
    println!("Length: {}", len);
    
    // Modify elements (must be mutable)
    let mut mutable = [1, 2, 3];
    mutable[1] = 10;
    println!("Modified: {:?}", mutable);
    
    // Out of bounds access causes panic
    // let out_of_bounds = numbers[10];  // Panics!
    
    // Safe access with get()
    match numbers.get(10) {
        Some(value) => println!("Value: {}", value),
        None => println!("Index out of bounds"),
    }
    
    // Array slicing
    let slice = &numbers[1..4];  // [2, 3, 4]
    println!("Slice: {:?}", slice);
    
    // Iterate over arrays
    for (index, value) in numbers.iter().enumerate() {
        println!("numbers[{}] = {}", index, value);
    }
    
    // Convert to Vec for dynamic sizing
    let vec: Vec<i32> = numbers.to_vec();
}
```

### Array vs Vec

| Feature | Array | Vec |
|---------|-------|-----|
| Size | Fixed at compile time | Dynamic |
| Type | `[T; N]` | `Vec<T>` |
| Memory | Stack (usually) | Heap |
| Performance | Faster | Slightly slower |
| Flexibility | Less | More |

---

## 3.8 Type Conversion (Casting)

### Using `as` Keyword

```rust
fn main() {
    // Integer to integer
    let a: u8 = 255;
    let b: u16 = a as u16;  // 255
    println!("u8 to u16: {}", b);
    
    // Larger to smaller (truncation)
    let c: u16 = 1000;
    let d: u8 = c as u8;  // Truncates to 232
    println!("u16 to u8 (truncated): {}", d);
    
    // Integer to float
    let e: i32 = 42;
    let f: f64 = e as f64;
    println!("i32 to f64: {}", f);
    
    // Float to integer (truncates toward zero)
    let g: f64 = 3.9;
    let h: i32 = g as i32;  // 3, not 4!
    println!("f64 to i32 (truncated): {}", h);
    
    // Boolean to integer
    let i: bool = true;
    let j: u8 = i as u8;  // 1
    println!("bool to u8: {}", j);
    
    let k: bool = false;
    let l: u8 = k as u8;  // 0
    println!("bool to u8: {}", l);
    
    // Character to integer
    let m: char = 'A';
    let n: u32 = m as u32;  // 65
    println!("char to u32: {}", n);
    
    // Pointer to usize (for FFI)
    let ptr = &e;
    let addr = ptr as *const i32 as usize;
    println!("Address: {}", addr);
}
```

### Safe Conversion with Traits

```rust
fn main() {
    // Using TryFrom/TryInto for fallible conversions
    use std::convert::TryFrom;
    
    let x: i32 = 42;
    
    // TryFrom returns Result
    match u8::try_from(x) {
        Ok(val) => println!("Converted: {}", val),
        Err(_) => println!("Conversion failed!"),
    }
    
    // Large number fails
    let y: i32 = 300;
    match u8::try_from(y) {
        Ok(val) => println!("Converted: {}", val),
        Err(e) => println!("Error: {}", e),
    }
    
    // Using From/Into for infallible conversions
    let a: i16 = 42;
    let b: i32 = i32::from(a);  // or: let b: i32 = a.into();
    println!("i16 to i32: {}", b);
}
```

---

## 3.9 Type Aliases

```rust
// Create alternative names for types
type Meters = f64;
type Result<T> = std::result::Result<T, std::io::Error>;
type LargeTuple = (i32, f64, String, Vec<u8>);

fn main() {
    let height: Meters = 180.5;
    let width: Meters = 100.0;
    
    println!("Height: {}m, Width: {}m", height, width);
    
    // Type aliases don't create new types
    let x: Meters = 50.0;
    let y: f64 = x;  // OK: Meters is just f64
}
```

---

## 3.10 The Unit Type

```rust
fn main() {
    // The unit type () has exactly one value: ()
    let unit_value: () = ();
    
    // Functions without explicit return return ()
    fn do_nothing() {
        // Implicitly returns ()
    }
    
    let result = do_nothing();
    println!("Result: {:?}", result);  // ()
    
    // Blocks that end with semicolon return ()
    let block_result = {
        let x = 5;
        x + 1;  // Semicolon means this returns ()
    };
    println!("Block result: {:?}", block_result);  // ()
    
    // Useful for generic code
    let optional_unit: Option<()> = Some(());
}
```

---

## 3.11 The Never Type

```rust
// The never type (!) represents code that never returns
fn panic_forever() -> ! {
    panic!("This function never returns!");
}

fn infinite_loop() -> ! {
    loop {
        // Infinite loop
    }
}

fn main() {
    // The never type can coerce to any other type
    let x: i32 = match panic_forever() {
        // This arm is never reached, so it can be any type
    };
    
    // Common in error handling
    let result: Result<i32, &str> = Err("error");
    let value = result.unwrap_or_else(|e| {
        eprintln!("Fatal error: {}", e);
        std::process::exit(1);  // Never returns
    });
}
```

---

## 3.12 Choosing the Right Type

### Integer Type Selection

| Use Case | Recommended Type |
|----------|-----------------|
| General purpose | `i32` or `i64` |
| Array indexing | `usize` |
| Byte data | `u8` |
| Large numbers | `i128` or `u128` |
| Memory-constrained | `i16` or `u16` |
| Interop with C | Match C types (`c_int`, etc.) |

### Float Type Selection

| Use Case | Recommended Type |
|----------|-----------------|
| General purpose | `f64` |
| GPU/graphics | `f32` |
| Memory-constrained arrays | `f32` |
| Financial (use decimal crate!) | `Decimal` |

### Common Patterns

```rust
fn main() {
    // Counting/indices
    let index: usize = 0;
    let count: usize = 10;
    
    // Mathematical calculations
    let result: f64 = 3.14159 * 2.0;
    
    // Flags and conditions
    let is_valid: bool = true;
    
    // Single characters
    let initial: char = 'A';
    
    // Fixed-size collection
    let days: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    
    // Related values
    let point: (i32, i32) = (10, 20);
    
    // Raw bytes
    let byte: u8 = 0xFF;
}
```

---

## Chapter 3 Exercises

### Exercise 3.1: Type Exploration
```rust
// Create variables of each integer type
// Print their min and max values using std::TYPE::MIN/MAX
// Experiment with overflow behavior
```

### Exercise 3.2: Temperature Converter
```rust
// Write functions to convert between:
// - Celsius to Fahrenheit
// - Fahrenheit to Celsius
// - Celsius to Kelvin
// - Kelvin to Celsius
// Use appropriate types (f64 for precision)
```

### Exercise 3.3: Array Operations
```rust
// Create an array of 10 integers
// Calculate: sum, average, min, max
// Find the index of a specific value
// Reverse the array in place
```

### Exercise 3.4: Tuple Practice
```rust
// Create a function that returns multiple values as a tuple
// Example: (min, max, sum, average) of an array
// Practice destructuring and pattern matching
```

### Exercise 3.5: Type Conversion
```rust
// Practice safe and unsafe conversions
// Implement bounds checking before casting
// Use TryFrom/TryInto for fallible conversions
```

---

## Summary

In this chapter, you learned:

✅ All of Rust's scalar types (integers, floats, bool, char)
✅ Integer types and their ranges (i8 through i128, usize)
✅ Floating-point types (f32, f64) and precision issues
✅ Boolean operations and comparisons
✅ Unicode character type (char)
✅ Tuple types and destructuring
✅ Array types and operations
✅ Type conversion using `as` and traits
✅ Type aliases for cleaner code
✅ Unit type () and never type (!)
✅ How to choose the right type for each use case

---

## What's Next?

Now that you master Rust's type system, let's learn how to control program flow! In Chapter 4, we'll explore conditionals, loops, and pattern matching.

**Continue to [Chapter 4: Control Flow](./04_control_flow.md)**
