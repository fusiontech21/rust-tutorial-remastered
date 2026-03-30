# Chapter 5: Ownership & Borrowing

## 5.1 Understanding Ownership

Ownership is Rust's most unique feature. It's how Rust achieves memory safety without a garbage collector.

### The Stack and the Heap

```
Stack:                                Heap:
┌─────────────────┐                  
│ name = "Alice"  │     ┌──────────► ┌─────────────┐
│ age = 30        │     │            │ "Alice"     │
│ ptr ────────────┘     │            │ (7 bytes)   │
└─────────────────┘     │            └─────────────┘
                        │
Fast allocation         │            ┌─────────────┐
LIFO access             └──────────► │ [1,2,3...]  │
Fixed size              │            │ (dynamic)   │
                        │            └─────────────┘
Slower allocation
Random access
Dynamic size
```

### Ownership Rules

1. **Each value has a variable that's its owner**
2. **There can only be one owner at a time**
3. **When the owner goes out of scope, the value is dropped**

---

## 5.2 Move Semantics

### Basic Move

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // s1 is MOVED to s2
    
    // println!("{}", s1);  // ERROR: s1 is no longer valid!
    println!("{}", s2);  // OK
}
```

**What happens:**
```
Before: s1 → "hello"
After:  s2 → "hello"  (s1 is invalidated)
```

### Move with Functions

```rust
fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s);  // ERROR: s was moved
    
    let x = 5;
    makes_copy(x);
    println!("{}", x);  // OK: i32 implements Copy
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}  // some_string goes out of scope and is dropped

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}  // some_integer is copied, no drop
```

### Return Values and Moves

```rust
fn main() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    
    // s2 is moved, can't use it
    println!("s1 = {}", s1);
    println!("s3 = {}", s3);
}

fn gives_ownership() -> String {
    String::from("yours")
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string  // Return ownership
}
```

---

## 5.3 The Copy Trait

### Types that Implement Copy

```rust
fn main() {
    // All integer types
    let x = 5;
    let y = x;  // Copy, not move
    println!("x = {}, y = {}", x, y);  // Both work!
    
    // Boolean
    let b1 = true;
    let b2 = b1;
    
    // Float types
    let f1 = 3.14;
    let f2 = f1;
    
    // Character
    let c1 = 'a';
    let c2 = c1;
    
    // Tuples (if all elements implement Copy)
    let t1 = (1, 2, 3);
    let t2 = t1;
    
    // References (always Copy)
    let r1 = &x;
    let r2 = r1;
}
```

### Types that DON'T Implement Copy

```rust
fn main() {
    // String - does NOT implement Copy
    let s1 = String::from("hello");
    let s2 = s1;  // Move, not copy
    // println!("{}", s1);  // ERROR
    
    // Vec - does NOT implement Copy
    let v1 = vec![1, 2, 3];
    let v2 = v1;  // Move
    // println!("{:?}", v1);  // ERROR
    
    // Box - does NOT implement Copy
    let b1 = Box::new(5);
    let b2 = b1;  // Move
}
```

### Implementing Copy for Your Types

```rust
#[derive(Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1;  // Copy!
    
    println!("p1 = {:?}", p1);  // OK
    println!("p2 = {:?}", p2);  // OK
}

// Can't derive Copy for types with non-Copy fields
#[derive(Clone, Debug)]  // Can't add Copy here
struct NotCopy {
    data: String,  // String doesn't implement Copy
}
```

---

## 5.4 References and Borrowing

### Creating References

```rust
fn main() {
    let s1 = String::from("hello");
    
    // & creates a reference (borrow)
    let len = calculate_length(&s1);
    
    // s1 is still valid!
    println!("Length of '{}' is {}", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}  // s goes out of scope, but doesn't drop the String
```

**What happens:**
```
s1 → "hello"
      ↑
s ────┘  (s borrows from s1)
```

### Mutable References

```rust
fn main() {
    let mut s = String::from("hello");
    
    change(&mut s);
    
    println!("{}", s);  // "hello, world"
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

### Reference Rules

```rust
fn main() {
    let mut s = String::from("hello");
    
    // Rule 1: Multiple immutable references OK
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    
    // Rule 2: Only ONE mutable reference at a time
    // let m1 = &mut s;  // ERROR if r1, r2 still in use
    // let m2 = &mut s;  // ERROR: can't have two mutable refs
    
    // Rule 3: Mutable reference excludes immutable refs
    drop(r1);
    drop(r2);
    let m1 = &mut s;  // OK now
    m1.push_str(", world");
    
    // Can't create immutable ref while mutable exists
    // let r3 = &s;  // ERROR
}
```

### Reference Scope

```rust
fn main() {
    let mut s = String::from("hello");
    
    {
        let r1 = &s;
        let r2 = &s;
        println!("{} and {}", r1, r2);
    }  // r1 and r2 go out of scope
    
    let r3 = &mut s;  // OK: r1, r2 are gone
    r3.push_str(", world");
    
    let r4 = &s;  // OK: r3 is no longer used
    println!("{}", r4);
}
```

---

## 5.5 Dangling References

### What is a Dangling Reference?

```rust
// ❌ This won't compile!
fn dangle() -> &String {
    let s = String::from("hello");
    &s  // ERROR: s is dropped at end of function
}  // s goes out of scope, reference would be invalid

// ✅ Correct way - return ownership
fn no_dangle() -> String {
    String::from("hello")
}

fn main() {
    let s = no_dangle();
    println!("{}", s);
}
```

### Valid Reference Patterns

```rust
fn main() {
    // Reference to local variable (valid while in scope)
    let s = String::from("hello");
    let r = &s;
    println!("{}", r);  // OK
    
    // Reference as function parameter
    fn use_ref(s: &String) {
        println!("{}", s);
    }  // Reference is valid throughout function
    
    // Reference in struct (requires lifetimes - Chapter 11)
    struct RefStruct<'a> {
        data: &'a str,
    }
}
```

---

## 5.6 The Slice Type

### String Slices

```rust
fn main() {
    let s = String::from("hello world");
    
    // Create a slice
    let hello = &s[0..5];   // "hello"
    let world = &s[6..11];  // "world"
    
    // Shorthand syntax
    let hello = &s[..5];    // from start to 5
    let world = &s[6..];    // from 6 to end
    let full = &s[..];      // entire string
    
    println!("{} {}", hello, world);
    
    // Slice type is &str
    let slice: &str = &s[0..5];
}
```

### String Slice Functions

```rust
// ✅ Better: use &str instead of &String
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

fn main() {
    let my_string = String::from("hello world");
    let word = first_word(&my_string);  // Works with String
    println!("First word: {}", word);
    
    let my_str = "hello world";
    let word = first_word(my_str);  // Also works with &str!
    println!("First word: {}", word);
}
```

### Array Slices

```rust
fn main() {
    let numbers = [1, 2, 3, 4, 5];
    
    // Create a slice of an array
    let slice = &numbers[1..3];  // [2, 3]
    
    println!("{:?}", slice);
    
    // Slice type
    let slice: &[i32] = &numbers[1..4];
    
    // Pass slices to functions
    print_slice(&slice);
}

fn print_slice(slice: &[i32]) {
    println!("{:?}", slice);
}
```

---

## 5.7 Ownership in Data Structures

### Structs and Ownership

```rust
struct User {
    username: String,
    email: String,
    active: bool,
}

fn main() {
    let user1 = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        active: true,
    };
    
    // Move ownership
    let user2 = user1;
    // println!("{}", user1.username);  // ERROR
    
    // Clone for deep copy
    let user3 = user2.clone();
    println!("{}", user2.username);  // OK
    println!("{}", user3.username);  // OK
}
```

### Enums and Ownership

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let m1 = Message::Quit;  // No ownership
    let m2 = Message::Move { x: 1, y: 2 };  // Copy types
    let m3 = Message::Write(String::from("hello"));  // Owns String
    
    // Pattern matching moves ownership
    match m3 {
        Message::Write(text) => {
            println!("{}", text);
        }  // text is moved here
        _ => {}
    }
}
```

---

## 5.8 Advanced Ownership Patterns

### Clone vs Copy

```rust
fn main() {
    // Copy: implicit, bitwise copy
    let x = 5;
    let y = x;  // Copy happens automatically
    
    // Clone: explicit, deep copy
    let s1 = String::from("hello");
    let s2 = s1.clone();  // Explicit clone
    
    println!("s1 = {}, s2 = {}", s1, s2);
}
```

### Rc and Arc for Shared Ownership

```rust
use std::rc::Rc;

fn main() {
    // Rc: Reference Counted (single-threaded)
    let rc1 = Rc::new(String::from("hello"));
    let rc2 = Rc::clone(&rc1);
    let rc3 = Rc::clone(&rc1);
    
    println!("Reference count: {}", Rc::strong_count(&rc1));  // 3
    
    drop(rc2);
    println!("Reference count: {}", Rc::strong_count(&rc1));  // 2
}

// For multi-threaded, use Arc (Atomic Reference Counted)
use std::sync::Arc;

fn main() {
    let arc = Arc::new(5);
    let arc_clone = Arc::clone(&arc);
    
    // Can be shared across threads
}
```

### Interior Mutability

```rust
use std::cell::RefCell;

fn main() {
    // RefCell allows mutation through immutable reference
    let data = RefCell::new(5);
    
    *data.borrow_mut() = 10;  // Mutate through borrow
    
    println!("Value: {}", data.borrow());
}
```

---

## 5.9 Common Ownership Patterns

### Builder Pattern

```rust
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    age: Option<u32>,
}

struct UserBuilder {
    username: String,
    email: String,
    age: Option<u32>,
}

impl UserBuilder {
    fn new(username: String, email: String) -> Self {
        UserBuilder {
            username,
            email,
            age: None,
        }
    }
    
    fn age(mut self, age: u32) -> Self {
        self.age = Some(age);
        self
    }
    
    fn build(self) -> User {
        User {
            username: self.username,
            email: self.email,
            age: self.age,
        }
    }
}

fn main() {
    let user = UserBuilder::new(
        String::from("alice"),
        String::from("alice@example.com"),
    )
    .age(30)
    .build();
    
    println!("{:?}", user);
}
```

### Cow (Clone on Write)

```rust
use std::borrow::Cow;

fn process_text(text: &str) -> Cow<str> {
    if text.contains('!') {
        // Need to modify - clone
        Cow::Owned(text.replace('!', "."))
    } else {
        // No modification - borrow
        Cow::Borrowed(text)
    }
}

fn main() {
    let s1 = "Hello";
    let s2 = "Hello!";
    
    let result1 = process_text(s1);  // Borrows
    let result2 = process_text(s2);  // Clones
    
    println!("{}", result1);
    println!("{}", result2);
}
```

---

## Chapter 5 Exercises

### Exercise 5.1: Ownership Basics
```rust
// Create a String and demonstrate:
// 1. Moving to another variable
// 2. Moving to a function
// 3. Getting ownership back from function
```

### Exercise 5.2: References Practice
```rust
// Write a function that takes &String and returns length
// Write a function that takes &mut String and appends text
// Demonstrate multiple immutable refs vs single mutable ref
```

### Exercise 5.3: Slice Operations
```rust
// Create a String with multiple words
// Extract slices for each word
// Write a function that returns the second word as a slice
```

### Exercise 5.4: Ownership in Structs
```rust
// Create a struct with String fields
// Demonstrate move semantics with struct instances
// Implement Clone and demonstrate deep copy
```

### Exercise 5.5: Fix the Errors
```rust
// Given code with ownership errors, fix them:
fn main() {
    let s = String::from("hello");
    let len = get_length(s);
    println!("{} has length {}", s, len);  // ERROR
}

fn get_length(s: String) -> usize {
    s.len()
}
```

---

## Summary

In this chapter, you learned:

✅ Stack vs Heap memory allocation
✅ Ownership rules (one owner, drop when out of scope)
✅ Move semantics and how values are transferred
✅ The Copy trait for automatic copying
✅ References and borrowing (& and &mut)
✅ Mutable vs immutable borrowing rules
✅ Dangling references and how Rust prevents them
✅ Slice types (&str, &[T])
✅ Ownership in structs and enums
✅ Rc/Arc for shared ownership
✅ Common ownership patterns

---

## What's Next?

Now that you understand ownership, let's build more complex data structures! In Chapter 6, we'll explore **Structs and Methods**.

**Continue to [Chapter 6: Structs & Methods](./06_structs_methods.md)**
