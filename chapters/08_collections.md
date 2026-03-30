# Chapter 8: Collections

## 8.1 Vec - Growable Arrays

### Creating Vectors

```rust
fn main() {
    // Empty vector with type inference
    let v: Vec<i32> = Vec::new();
    
    // With initial values
    let v = vec![1, 2, 3, 4, 5];
    
    // With repeated values
    let v = vec![0; 10];  // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    
    // From array
    let arr = [1, 2, 3];
    let v = arr.to_vec();
    
    // From iterator
    let v: Vec<i32> = (1..=5).collect();
    
    // With capacity (optimization)
    let mut v = Vec::with_capacity(100);
    println!("Length: {}, Capacity: {}", v.len(), v.capacity());
}
```

### Modifying Vectors

```rust
fn main() {
    let mut v = Vec::new();
    
    // Push elements
    v.push(1);
    v.push(2);
    v.push(3);
    
    // Pop element (returns Option)
    let last = v.pop();
    println!("Popped: {:?}", last);  // Some(3)
    
    // Insert at index
    v.insert(1, 10);  // [1, 10, 2]
    
    // Remove at index
    let removed = v.remove(1);  // [1, 2]
    
    // Clear all
    v.clear();
    
    // Extend
    v.extend(vec![4, 5, 6]);
    v.extend(7..=10);
    
    println!("Vector: {:?}", v);
}
```

### Accessing Elements

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    
    // Indexing (panics if out of bounds)
    let third = &v[2];
    println!("Third element: {}", third);
    
    // Safe access with get
    match v.get(2) {
        Some(value) => println!("Third element: {}", value),
        None => println!("No third element"),
    }
    
    // Out of bounds
    // let out_of_bounds = &v[100];  // Panics!
    let safe = v.get(100);  // Returns None
    
    // First and last
    println!("First: {:?}", v.first());
    println!("Last: {:?}", v.last());
    
    // Slicing
    let slice = &v[1..4];  // [2, 3, 4]
    println!("Slice: {:?}", slice);
    
    // Mutable access
    let mut v = vec![1, 2, 3];
    v[1] = 10;
    println!("Modified: {:?}", v);
}
```

### Iterating Over Vectors

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    
    // Immutable iteration
    for i in &v {
        println!("{}", i);
    }
    
    // Mutable iteration
    let mut v = vec![1, 2, 3];
    for i in &mut v {
        *i *= 2;
    }
    println!("Doubled: {:?}", v);
    
    // With index
    for (index, value) in v.iter().enumerate() {
        println!("v[{}] = {}", index, value);
    }
    
    // Consume vector (takes ownership)
    for i in v {
        println!("Owned: {}", i);
    }
    // v is moved here
}
```

### Vector Operations

```rust
fn main() {
    let v = vec![3, 1, 4, 1, 5, 9, 2, 6];
    
    // Length
    println!("Length: {}", v.len());
    
    // Is empty
    println!("Is empty: {}", v.is_empty());
    
    // Contains
    println!("Contains 4: {}", v.contains(&4));
    
    // Sort
    let mut v = v;
    v.sort();
    println!("Sorted: {:?}", v);
    
    // Sort descending
    v.sort_by(|a, b| b.cmp(a));
    
    // Reverse
    v.reverse();
    
    // Deduplicate (requires sorted)
    v.sort();
    v.dedup();
    
    // Retain
    v.retain(|&x| x % 2 == 0);  // Keep only even numbers
    
    // Drain
    let drained: Vec<i32> = v.drain(1..3).collect();
    
    // Split off
    let mut v = vec![1, 2, 3, 4, 5];
    let v2 = v.split_off(3);  // v = [1, 2, 3], v2 = [4, 5]
}
```

---

## 8.2 String Types

### String vs &str

```rust
fn main() {
    // &str - string slice (borrowed)
    let s1: &str = "hello";
    
    // String - owned, growable
    let s2: String = String::from("hello");
    
    // Conversion
    let s3: String = s1.to_string();
    let s4: &str = &s2;
    
    // Memory
    println!("Size of &str: {}", std::mem::size_of::<&str>());  // 16 bytes (pointer + length)
    println!("Size of String: {}", std::mem::size_of::<String>());  // 24 bytes (pointer + length + capacity)
}
```

### Creating Strings

```rust
fn main() {
    // From literal
    let s1 = String::from("hello");
    
    // To string
    let s2 = "world".to_string();
    
    // Empty string
    let s3 = String::new();
    
    // With capacity
    let s4 = String::with_capacity(100);
    
    // From other types
    let num = 42;
    let s5 = num.to_string();
    
    let s6 = String::from(100);
    
    // From iterator
    let s7: String = ['h', 'e', 'l', 'l', 'o'].iter().collect();
    
    // Repeated
    let s8 = "ha".repeat(3);  // "hahaha"
}
```

### Modifying Strings

```rust
fn main() {
    let mut s = String::from("hello");
    
    // Push character
    s.push('!');
    
    // Push string
    s.push_str(" world");
    
    // Insert character
    s.insert(5, ',');  // "hello, world"
    
    // Insert string
    s.insert_str(0, "Say: ");
    
    // Remove last character
    let c = s.pop();
    
    // Remove at index
    let c = s.remove(0);
    
    // Clear
    s.clear();
    
    // Extend
    let mut s = String::from("hello");
    s.extend([' ', 'w', 'o', 'r', 'l', 'd']);
    
    println!("String: {}", s);
}
```

### String Concatenation

```rust
fn main() {
    let s1 = String::from("Hello");
    let s2 = String::from(" World");
    
    // Using +
    let s3 = s1 + &s2;  // s1 is moved
    
    // Using format!
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let s4 = format!("{} {}!", s1, s2);
    
    // Using push_str
    let mut s = String::from("Hello");
    s.push_str(" World");
    
    // Multiple concatenation
    let s = format!("{} + {} = {}", 2, 3, 2 + 3);
    
    // Join
    let words = vec!["hello", "world", "rust"];
    let s = words.join(" ");
    
    println!("{}", s);
}
```

### String Indexing and Slicing

```rust
fn main() {
    let s = String::from("hello");
    
    // Can't index!
    // let c = s[0];  // ERROR
    
    // Use chars instead
    let first = s.chars().next();
    
    // Slicing (be careful with Unicode!)
    let hello = "hello";
    let slice = &hello[0..2];  // "he"
    
    // Unicode example
    let s = "Hello, 世界";
    // &s[0..7] would panic! 世界 is 6 bytes
    
    // Safe slicing
    for c in s.chars() {
        println!("{}", c);
    }
    
    // Byte length vs char length
    println!("Bytes: {}", s.len());
    println!("Chars: {}", s.chars().count());
}
```

### String Methods

```rust
fn main() {
    let s = "  Hello, World!  ";
    
    // Trim whitespace
    println!("|{}|", s.trim());
    
    // Case conversion
    println!("{}", s.to_lowercase());
    println!("{}", s.to_uppercase());
    
    // Check prefix/suffix
    println!("{}", s.trim().starts_with("Hello"));
    println!("{}", s.trim().ends_with("!"));
    
    // Contains
    println!("{}", s.contains("World"));
    
    // Find
    println!("{:?}", s.find("World"));  // Some(8)
    
    // Replace
    println!("{}", s.replace("World", "Rust"));
    
    // Split
    for word in s.split_whitespace() {
        println!("Word: {}", word);
    }
    
    // Lines
    let text = "line1\nline2\nline3";
    for line in text.lines() {
        println!("Line: {}", line);
    }
    
    // Parse
    let num: i32 = "42".parse().unwrap();
}
```

---

## 8.3 HashMap

### Creating HashMaps

```rust
use std::collections::HashMap;

fn main() {
    // Empty HashMap
    let mut scores = HashMap::new();
    
    // Insert
    scores.insert("Alice", 90);
    scores.insert("Bob", 85);
    scores.insert("Charlie", 95);
    
    // From iterator
    let teams = vec![("Blue", 10), ("Yellow", 20)];
    let mut map: HashMap<_, _> = teams.into_iter().collect();
    
    // With capacity
    let mut map = HashMap::with_capacity(100);
    
    // Default
    let map = HashMap::new();
}
```

### Accessing HashMap

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Alice", 90);
    scores.insert("Bob", 85);
    
    // Get (returns Option)
    match scores.get("Alice") {
        Some(score) => println!("Alice scored: {}", score),
        None => println!("Alice not found"),
    }
    
    // Get with default
    let score = scores.get("Charlie").unwrap_or(&0);
    
    // Contains key
    println!("Has Alice: {}", scores.contains_key("Alice"));
    
    // Iterate
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    
    // Keys and values
    for key in scores.keys() {
        println!("Key: {}", key);
    }
    
    for value in scores.values() {
        println!("Value: {}", value);
    }
}
```

### Modifying HashMap

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    
    // Insert returns old value
    let old = scores.insert("Alice", 90);
    println!("Old value: {:?}", old);  // None
    
    // Update
    scores.insert("Alice", 95);
    
    // Entry API - insert if not exists
    scores.entry("Bob").or_insert(85);
    
    // Entry API - modify or insert
    let score = scores.entry("Charlie").or_insert(0);
    *score += 10;
    
    // Entry API - complex modification
    let text = "hello world";
    let mut char_count = HashMap::new();
    
    for c in text.chars() {
        *char_count.entry(c).or_insert(0) += 1;
    }
    
    println!("{:?}", char_count);
    
    // Remove
    scores.remove("Alice");
    
    // Clear
    scores.clear();
}
```

### HashMap with Custom Types

```rust
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Debug, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// Need to implement Hash for custom keys
impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

fn main() {
    let mut map = HashMap::new();
    map.insert(Point { x: 1, y: 2 }, "point1");
    map.insert(Point { x: 3, y: 4 }, "point2");
    
    let p = Point { x: 1, y: 2 };
    println!("{:?}", map.get(&p));
}
```

---

## 8.4 Common Collection Patterns

### Vec Patterns

```rust
fn main() {
    // Filter and collect
    let numbers = vec![1, 2, 3, 4, 5, 6];
    let evens: Vec<i32> = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)
        .copied()
        .collect();
    
    // Map and collect
    let doubled: Vec<i32> = numbers
        .iter()
        .map(|x| x * 2)
        .collect();
    
    // Zip
    let a = vec![1, 2, 3];
    let b = vec![4, 5, 6];
    let zipped: Vec<(i32, i32)> = a.iter().zip(b.iter()).copied().collect();
    
    // Flatten
    let nested = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    let flat: Vec<i32> = nested.into_iter().flatten().collect();
    
    // Unique values
    use std::collections::HashSet;
    let nums = vec![1, 2, 2, 3, 3, 3];
    let unique: HashSet<i32> = nums.into_iter().collect();
}
```

### String Patterns

```rust
fn main() {
    // Parse CSV
    let csv = "apple,banana,cherry";
    let fruits: Vec<&str> = csv.split(',').collect();
    
    // Build string efficiently
    let mut s = String::new();
    for i in 0..5 {
        s.push_str(&format!("{} ", i));
    }
    
    // Better: use format! with join
    let numbers: Vec<String> = (0..5).map(|i| i.to_string()).collect();
    let s = numbers.join(" ");
    
    // Parse key-value pairs
    let input = "name=Alice;age=30;city=NYC";
    let pairs: HashMap<&str, &str> = input
        .split(';')
        .filter_map(|s| s.split_once('='))
        .collect();
}
```

### HashMap Patterns

```rust
use std::collections::HashMap;

fn main() {
    // Group by
    let words = vec!["apple", "banana", "apricot", "blueberry"];
    let mut grouped: HashMap<char, Vec<&str>> = HashMap::new();
    
    for &word in &words {
        grouped
            .entry(word.chars().next().unwrap())
            .or_insert_with(Vec::new)
            .push(word);
    }
    
    // Count occurrences
    let items = vec!["apple", "banana", "apple", "cherry", "banana", "apple"];
    let mut counts = HashMap::new();
    
    for &item in &items {
        *counts.entry(item).or_insert(0) += 1;
    }
    
    // Merge HashMaps
    let mut map1 = HashMap::from([("a", 1), ("b", 2)]);
    let map2 = HashMap::from([("b", 3), ("c", 4)]);
    
    for (key, value) in map2 {
        map1.entry(key).or_insert(value);
    }
    
    // Retain entries
    let mut scores = HashMap::from([("Alice", 90), ("Bob", 50), ("Charlie", 80)]);
    scores.retain(|_, &mut score| score >= 60);
}
```

---

## 8.5 Performance Considerations

### Vec

```rust
fn main() {
    // Pre-allocate when size is known
    let mut v = Vec::with_capacity(1000);
    for i in 0..1000 {
        v.push(i);
    }
    
    // Avoid in hot loops
    let mut v = Vec::new();
    for i in 0..1000 {
        v.push(i);  // May reallocate
    }
    
    // Use slice for functions
    fn process(v: &[i32]) { }
    fn process_vec(v: &Vec<i32>) { }  // Less flexible
    
    let vec = vec![1, 2, 3];
    process(&vec);
    process(&vec[1..]);  // Can pass slice
}
```

### String

```rust
fn main() {
    // Efficient concatenation
    let mut s = String::with_capacity(100);
    s.push_str("hello");
    s.push_str(" world");
    
    // Inefficient (creates new String each time)
    let s = String::new();
    let s = s + "hello";
    let s = s + " world";
    
    // Use format! for complex strings
    let name = "Alice";
    let age = 30;
    let s = format!("{} is {} years old", name, age);
}
```

### HashMap

```rust
use std::collections::HashMap;

fn main() {
    // Set capacity for large maps
    let mut map = HashMap::with_capacity(1000);
    
    // Use entry API for modifications
    let mut map = HashMap::new();
    
    // Inefficient
    if !map.contains_key("key") {
        map.insert("key", 0);
    }
    let val = map.get_mut("key").unwrap();
    *val += 1;
    
    // Efficient
    *map.entry("key").or_insert(0) += 1;
    
    // Use get for read-only access
    if let Some(val) = map.get("key") {
        println!("{}", val);
    }
}
```

---

## Chapter 8 Exercises

### Exercise 8.1: Vector Operations
```rust
// Implement functions to:
// - Remove duplicates from Vec
// - Find second largest element
// - Rotate vector by n positions
// - Merge two sorted vectors
```

### Exercise 8.2: String Processing
```rust
// Implement functions to:
// - Reverse each word in a sentence
// - Check if string is palindrome
// - Compress string (aaa -> a3)
// - Validate email format
```

### Exercise 8.3: HashMap Applications
```rust
// Implement:
// - Word frequency counter
// - Two-sum problem solver
// - Anagram grouper
// - LRU cache (simplified)
```

### Exercise 8.4: Collection Conversion
```rust
// Practice converting between:
// - Vec to HashMap
// - HashMap to Vec
// - String to Vec<char>
// - Vec to String
```

---

## Summary

In this chapter, you learned:

✅ Vec - creating, modifying, and iterating
✅ String vs &str differences
✅ String manipulation methods
✅ HashMap operations and entry API
✅ Common collection patterns
✅ Performance considerations
✅ Collection conversions

---

## What's Next?

Collections are essential, but what happens when things go wrong? In Chapter 9, we'll master **Error Handling** in Rust.

**Continue to [Chapter 9: Error Handling](./09_error_handling.md)**
