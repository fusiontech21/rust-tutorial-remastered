# Chapter 11: Lifetimes

## 11.1 Understanding Lifetimes

### What are Lifetimes?

Lifetimes are Rust's way of ensuring that references are valid for as long as they need to be.

```rust
fn main() {
    {
        let r;                // ---------+-- 'a
        {                     //          |
            let x = 5;        // -+-- 'b  |
            r = &x;           //  |       |
        }                     // -+       |
                              //          |
        // println!("{}", r); // ERROR: x was dropped
    }                         // ---------+
}
```

### Lifetime Annotations

```rust
// Lifetime annotation syntax
&'a i32        // Reference with lifetime 'a
&'a mut i32    // Mutable reference with lifetime 'a
```

---

## 11.2 Lifetime Annotation Syntax

### Basic Syntax

```rust
// Single lifetime
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let s1 = String::from("hello");
    let s2 = String::from("world");
    
    let result = longest(&s1, &s2);
    println!("Longest: {}", result);
}
```

### Multiple Lifetimes

```rust
fn multi_lifetime<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    // Return value tied to first parameter's lifetime
    x
}

fn main() {
    let s1 = String::from("short");
    let s2 = String::from("longer string");
    
    let result = multi_lifetime(&s1, &s2);
    println!("{}", result);
}
```

### Lifetime Elision

Rust has rules that allow omitting lifetime annotations in common cases:

```rust
// These are equivalent:

// With elision
fn first_word(s: &str) -> &str {
    &s.split_whitespace().next().unwrap()
}

// Explicit
fn first_word_explicit<'a>(s: &'a str) -> &'a str {
    &s.split_whitespace().next().unwrap()
}

// Elision Rules:
// 1. Each parameter gets its own lifetime
// 2. If exactly one input lifetime, it's assigned to all outputs
// 3. If &self or &mut self, self's lifetime is assigned to outputs
```

---

## 11.3 Lifetimes in Structs

### Structs with References

```rust
// Struct holding a reference needs lifetime
struct Excerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    
    let excerpt = Excerpt {
        part: first_sentence,
    };
    
    println!("Excerpt: {}", excerpt.part);
}

// Multiple lifetimes in struct
struct MultiRef<'a, 'b> {
    first: &'a str,
    second: &'b str,
}

fn main() {
    let s1 = String::from("hello");
    let s2 = String::from("world");
    
    let multi = MultiRef {
        first: &s1,
        second: &s2,
    };
}
```

### Struct Methods with Lifetimes

```rust
struct Excerpt<'a> {
    part: &'a str,
}

impl<'a> Excerpt<'a> {
    fn get_part(&self) -> &'a str {
        self.part
    }
    
    fn announce_and_return(&self, announcement: &'a str) -> &'a str {
        println!("Announcement: {}", announcement);
        self.part
    }
}

// Can omit lifetime in impl when using elision
impl<'a> Excerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Announcement: {}", announcement);
        self.part
    }
}

fn main() {
    let text = String::from("Hello, world!");
    let excerpt = Excerpt { part: &text };
    
    println!("Part: {}", excerpt.get_part());
}
```

---

## 11.4 Static Lifetime

```rust
// 'static lifetime means the reference lives for entire program
let s: &'static str = "I have a static lifetime";

// All string literals have 'static lifetime
const GREETING: &str = "Hello!";

fn main() {
    let x: &'static str = "Static string";
    println!("{}", x);
    
    // Can coerce 'static to shorter lifetimes
    let shorter: &str = x;
}
```

---

## 11.5 Combining Lifetimes with Traits

### Lifetime Bounds on Traits

```rust
use std::fmt::Display;

// Trait object with lifetime
fn print_with_lifetime<'a>(item: &'a dyn Display) {
    println!("{}", item);
}

// Bound on trait
fn longest_with_display<'a, T>(x: &'a str, y: &'a T) -> &'a str
where
    T: Display,
{
    println!("Y: {}", y);
    x
}

fn main() {
    let s1 = String::from("hello");
    let s2 = 42;
    
    let result = longest_with_display(&s1, &s2);
    println!("Result: {}", result);
}
```

### HRTB (Higher-Ranked Trait Bounds)

```rust
// For all lifetimes 'a
fn call_with_ref<F>(f: F)
where
    F: for<'a> Fn(&'a str),
{
    let s = String::from("hello");
    f(&s);
}

fn main() {
    call_with_ref(|s| println!("{}", s));
}
```

---

## 11.6 Advanced Lifetime Patterns

### Lifetime Subtyping

```rust
// 'b must outlive 'a
fn constrained<'a, 'b: 'a>(x: &'a i32, y: &'b i32) -> &'a i32 {
    x
}

// In structs
struct Ref<'a, 'b: 'a> {
    x: &'a i32,
    y: &'b i32,
}
```

### NLL (Non-Lexical Lifetimes)

```rust
// Rust 2018+ has NLL - more flexible lifetime checking

fn main() {
    let mut s = String::from("hello");
    
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // r1 and r2 no longer used after this point
    
    let r3 = &mut s;  // OK! NLL knows r1, r2 are dead
    println!("{}", r3);
}

// Without NLL, this would be an error
```

### Lifetime Coercion

```rust
fn main() {
    let s = String::from("hello");
    
    // Longer lifetime can be coerced to shorter
    let long: &str = &s;
    let short: &str = long;  // 'long coerced to 'short
    
    // Explicit lifetime bounds
    fn coerce<'a, 'b: 'a>(x: &'b str) -> &'a str {
        x  // 'b outlives 'a, so coercion is valid
    }
}
```

---

## 11.7 Common Lifetime Patterns

### Returning References from Functions

```rust
// ✅ Valid: returning reference to parameter
fn get_first<'a>(slice: &'a [i32]) -> Option<&'a i32> {
    slice.first()
}

// ❌ Invalid: returning reference to local
// fn get_local() -> &i32 {
//     let x = 5;
//     &x  // ERROR: x doesn't live long enough
// }

// ✅ Valid: returning owned value
fn get_owned() -> i32 {
    let x = 5;
    x
}
```

### Holding References in Structs

```rust
// Parser holding reference to input
struct Parser<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Parser { input, position: 0 }
    }
    
    fn current_char(&self) -> Option<char> {
        self.input[self.position..].chars().next()
    }
    
    fn advance(&mut self) {
        if self.position < self.input.len() {
            self.position += 1;
        }
    }
}

fn main() {
    let input = String::from("hello");
    let mut parser = Parser::new(&input);
    
    while let Some(c) = parser.current_char() {
        println!("Char: {}", c);
        parser.advance();
    }
}
```

### Iterator with Lifetimes

```rust
struct Window<'a, T> {
    slice: &'a [T],
    size: usize,
}

impl<'a, T> Window<'a, T> {
    fn new(slice: &'a [T], size: usize) -> Self {
        Window { slice, size }
    }
}

fn main() {
    let data = [1, 2, 3, 4, 5];
    let window = Window::new(&data, 3);
}
```

---

## 11.8 Lifetime Troubleshooting

### Common Errors

```rust
// ERROR: missing lifetime specifier
// fn longest(x: &str, y: &str) -> &str {

// FIX: add lifetime
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// ERROR: lifetime mismatch
// struct Excerpt {
//     part: &str,
// }

// FIX: add lifetime
struct Excerpt<'a> {
    part: &'a str,
}

// ERROR: borrowed value does not live long enough
// fn get_ref() -> &String {
//     let s = String::from("hello");
//     &s
// }

// FIX: return owned value
fn get_owned() -> String {
    String::from("hello")
}
```

### Debugging Lifetimes

```rust
// Use compiler errors as guidance
// Rust's lifetime errors are very informative

fn main() {
    let s1 = String::from("hello");
    let s2 = String::from("world");
    
    // Compiler will tell you what lifetime is needed
    let result = longest(&s1, &s2);
    println!("{}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

---

## 11.9 Lifetimes with Generics and Traits

```rust
use std::fmt::Display;

// Generic struct with lifetime
struct Container<'a, T> {
    value: &'a T,
    label: &'a str,
}

impl<'a, T: Display> Container<'a, T> {
    fn new(value: &'a T, label: &'a str) -> Self {
        Container { value, label }
    }
    
    fn print(&self) {
        println!("{}: {}", self.label, self.value);
    }
}

// Trait with lifetime
trait GetRef<'a> {
    fn get_ref(&'a self) -> &'a i32;
}

struct Wrapper {
    value: i32,
}

impl<'a> GetRef<'a> for Wrapper {
    fn get_ref(&'a self) -> &'a i32 {
        &self.value
    }
}

fn main() {
    let value = 42;
    let container = Container::new(&value, "Answer");
    container.print();
    
    let wrapper = Wrapper { value: 100 };
    println!("{}", wrapper.get_ref());
}
```

---

## Chapter 11 Exercises

### Exercise 11.1: Basic Lifetimes
```rust
// Add lifetime annotations to make these compile:
fn first_char(s: &str) -> &char;
fn longer(s1: &str, s2: &str) -> &str;
```

### Exercise 11.2: Struct Lifetimes
```rust
// Create a struct that holds:
// - Reference to a string
// - Reference to a slice of integers
// Implement methods that return references
```

### Exercise 11.3: Multiple Lifetimes
```rust
// Create a function with multiple lifetime parameters
// Practice lifetime subtyping ('b: 'a)
```

### Exercise 11.4: Lifetime Elision
```rust
// Identify which functions need explicit lifetimes
// Apply elision rules where possible
```

### Exercise 11.5: Complex Scenarios
```rust
// Create a parser struct with lifetimes
// Implement iterator that yields references
// Handle multiple reference fields
```

---

## Summary

In this chapter, you learned:

✅ What lifetimes are and why they matter
✅ Lifetime annotation syntax
✅ Lifetime elision rules
✅ Lifetimes in structs and impl blocks
✅ The 'static lifetime
✅ Combining lifetimes with traits
✅ Lifetime subtyping and coercion
✅ Non-lexical lifetimes (NLL)
✅ Common lifetime patterns and errors
✅ Debugging lifetime issues

---

## What's Next?

Lifetimes ensure reference validity, but Rust also has powerful functional programming features! In Chapter 12, we'll explore **Closures and Iterators**.

**Continue to [Chapter 12: Closures & Iterators](./12_closures_iterators.md)**
