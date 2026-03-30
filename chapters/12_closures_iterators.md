# Chapter 12: Functional Features - Closures & Iterators

## 12.1 Closures

### Closure Syntax

```rust
fn main() {
    // Basic closure
    let add_one = |x| x + 1;
    let result = add_one(5);
    println!("{}", result);  // 6
    
    // Closure with multiple parameters
    let add = |x, y| x + y;
    println!("{}", add(3, 4));  // 7
    
    // Closure with no parameters
    let say_hello = || println!("Hello!");
    say_hello();
    
    // Closure with body
    let multiply = |x, y| {
        let result = x * y;
        result
    };
    println!("{}", multiply(3, 4));  // 12
}
```

### Type Annotations

```rust
fn main() {
    // Type inference
    let add_one = |x| x + 1;
    
    // Explicit types
    let add_one_explicit = |x: i32| -> i32 { x + 1 };
    
    // Multiple parameters with types
    let add = |x: i32, y: i32| -> i32 { x + y };
    
    // Mutable parameter
    let mut multiply = |mut x: i32| {
        x *= 2;
        x
    };
    
    // Closure as variable
    let closure: fn(i32) -> i32 = |x| x + 1;
}
```

### Capturing Environment

```rust
fn main() {
    let x = 42;
    
    // Closure captures x by reference
    let print_x = || println!("x = {}", x);
    print_x();
    
    // Closure captures and uses x
    let add_x = |y| y + x;
    println!("{}", add_x(10));  // 52
    
    // x is still accessible
    println!("x = {}", x);
}
```

### Closure Traits: Fn, FnMut, FnOnce

```rust
// Fn - captures by immutable reference
fn call_fn<F>(f: F)
where
    F: Fn(i32) -> i32,
{
    println!("Result: {}", f(5));
    println!("Result again: {}", f(10));  // Can call multiple times
}

// FnMut - captures by mutable reference
fn call_fn_mut<F>(mut f: F)
where
    F: FnMut(i32) -> i32,
{
    println!("Result: {}", f(5));
    println!("Result again: {}", f(10));
}

// FnOnce - captures by value (consumes closure)
fn call_fn_once<F>(f: F)
where
    F: FnOnce(i32) -> i32,
{
    println!("Result: {}", f(5));
    // f(10);  // ERROR: f was consumed
}

fn main() {
    let x = 10;
    
    // Implements Fn, FnMut, FnOnce
    let immutable = |y| y + x;
    call_fn(immutable);
    
    // Implements FnMut, FnOnce
    let mut counter = 0;
    let mut increment = |amount| {
        counter += amount;
        counter
    };
    call_fn_mut(increment);
    
    // Implements FnOnce only
    let s = String::from("hello");
    let consume = move |prefix: String| {
        format!("{} {}", prefix, s)
    };
    call_fn_once(consume);
}
```

### Move Keyword

```rust
fn main() {
    let s = String::from("hello");
    
    // Without move - borrows s
    let borrow = || println!("{}", s);
    borrow();
    println!("{}", s);  // Still accessible
    
    // With move - takes ownership
    let t = String::from("world");
    let owns = move || println!("{}", t);
    owns();
    // println!("{}", t);  // ERROR: t was moved
}
```

### Closures as Parameters

```rust
fn apply<F>(f: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(x)
}

fn apply_twice<F>(f: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(f(x))
}

fn main() {
    let result = apply(|x| x * 2, 5);
    println!("{}", result);  // 10
    
    let result = apply_twice(|x| x + 1, 5);
    println!("{}", result);  // 7
}
```

### Returning Closures

```rust
fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

fn make_multiplier(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x * y
}

fn main() {
    let add_five = make_adder(5);
    println!("{}", add_five(10));  // 15
    
    let multiply_by_three = make_multiplier(3);
    println!("{}", multiply_by_three(4));  // 12
}
```

---

## 12.2 Iterators

### Iterator Trait

```rust
// Iterator trait definition
trait Iterator {
    type Item;
    
    fn next(&mut self) -> Option<Self::Item>;
}
```

### Creating Iterators

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    
    // iter() - immutable references
    for x in v.iter() {
        println!("{}", x);
    }
    
    // iter_mut() - mutable references
    for x in v.iter_mut() {
        *x *= 2;
    }
    
    // into_iter() - consumes, yields ownership
    for x in v.into_iter() {
        println!("{}", x);
    }
    // v is moved here
    
    // Range iterators
    for i in 0..5 {
        println!("{}", i);
    }
    
    for i in (0..5).rev() {
        println!("{}", i);
    }
    
    for i in (0..10).step_by(2) {
        println!("{}", i);
    }
}
```

### Iterator Adapters (Lazy)

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    
    // map - transform elements
    let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
    
    // filter - keep elements matching predicate
    let evens: Vec<&i32> = v.iter().filter(|x| *x % 2 == 0).collect();
    
    // filter_map - filter and transform
    let strings: Vec<String> = v.iter()
        .filter_map(|x| {
            if *x % 2 == 0 {
                Some(format!("even: {}", x))
            } else {
                None
            }
        })
        .collect();
    
    // take - limit number of elements
    let first_three: Vec<&i32> = v.iter().take(3).collect();
    
    // skip - skip elements
    let skip_first_two: Vec<&i32> = v.iter().skip(2).collect();
    
    // enumerate - add index
    for (index, value) in v.iter().enumerate() {
        println!("{}: {}", index, value);
    }
    
    // zip - combine two iterators
    let v2 = vec!['a', 'b', 'c'];
    let zipped: Vec<(&i32, &char)> = v.iter().zip(v2.iter()).collect();
    
    // chain - concatenate iterators
    let v3 = vec![6, 7, 8];
    let chained: Vec<&i32> = v.iter().chain(v3.iter()).collect();
    
    // flatten - flatten nested iterators
    let nested = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    let flat: Vec<i32> = nested.into_iter().flatten().collect();
}
```

### Consuming Adapters

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    
    // collect - consume into collection
    let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
    
    // count - count elements
    let count = v.iter().count();
    
    // sum - sum elements
    let sum: i32 = v.iter().sum();
    
    // product - multiply elements
    let product: i32 = v.iter().product();
    
    // fold - accumulate with initial value
    let sum = v.iter().fold(0, |acc, x| acc + x);
    
    // reduce - accumulate without initial value
    let sum = v.iter().reduce(|acc, x| acc + x);
    
    // all - check if all match
    let all_positive = v.iter().all(|x| *x > 0);
    
    // any - check if any match
    let has_even = v.iter().any(|x| *x % 2 == 0);
    
    // find - find first matching element
    let first_even = v.iter().find(|x| *x % 2 == 0);
    
    // position - find index of first match
    let pos = v.iter().position(|x| *x == 3);
    
    // max, min
    let max = v.iter().max();
    let min = v.iter().min();
    
    // last - get last element
    let last = v.iter().last();
    
    // nth - get nth element
    let third = v.iter().nth(2);
}
```

### Custom Iterators

```rust
struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Counter {
        Counter { count: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let counter = Counter::new(5);
    
    for num in counter {
        println!("{}", num);
    }
    
    // Can use iterator methods
    let counter = Counter::new(5);
    let sum: u32 = counter.filter(|n| n % 2 == 0).sum();
    println!("Sum of evens: {}", sum);
}
```

---

## 12.3 Performance Patterns

### Zero-Cost Abstractions

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    
    // Iterator chain - zero cost!
    let result: i32 = v.iter()
        .filter(|&x| x % 2 == 0)
        .map(|x| x * 2)
        .sum();
    
    // Compiles to same code as manual loop
    let mut sum = 0;
    for &x in &v {
        if x % 2 == 0 {
            sum += x * 2;
        }
    }
}
```

### Avoiding Unnecessary Allocations

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    
    // Inefficient - collects intermediate results
    let result: Vec<i32> = v.iter()
        .map(|x| x * 2)
        .collect();
    let sum: i32 = result.iter().sum();
    
    // Efficient - no intermediate allocation
    let sum: i32 = v.iter()
        .map(|x| x * 2)
        .sum();
    
    // Use references when possible
    let strings = vec!["hello", "world"];
    let upper: Vec<&str> = strings.iter()
        .map(|s| *s)
        .collect();
}
```

---

## 12.4 Advanced Iterator Patterns

### Chain Operations

```rust
fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];
    
    // Chain multiple iterators
    let all: Vec<i32> = v1.iter()
        .chain(v2.iter())
        .copied()
        .collect();
    
    // Flat map for complex transformations
    let words = vec!["hello", "world"];
    let chars: Vec<char> = words.iter()
        .flat_map(|s| s.chars())
        .collect();
}
```

### Peekable and Fuse

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let mut iter = v.iter().peekable();
    
    // Peek without consuming
    if let Some(&&next) = iter.peek() {
        println!("Next: {}", next);
    }
    
    // Consume
    println!("Got: {}", iter.next().unwrap());
    
    // Fuse - stops after None
    let mut iter = v.iter().take(2).fuse();
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());  // None
    println!("{:?}", iter.next());  // Still None
}
```

### Scan and Inspect

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    
    // scan - stateful transformation
    let mut sum = 0;
    let running_sum: Vec<i32> = v.iter()
        .scan(0, |state, &x| {
            *state += x;
            Some(*state)
        })
        .collect();
    
    // inspect - debug iterator
    let result: Vec<i32> = v.iter()
        .inspect(|x| println!("Before: {}", x))
        .map(|x| x * 2)
        .inspect(|x| println!("After: {}", x))
        .collect();
}
```

---

## 12.5 Closures and Iterators Together

### Functional Patterns

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    // Map-reduce pattern
    let sum: i32 = numbers.iter()
        .map(|x| x * 2)
        .filter(|x| *x > 5)
        .sum();
    
    // Group by pattern
    use std::collections::HashMap;
    
    let words = vec!["apple", "banana", "apricot", "blueberry"];
    let mut grouped: HashMap<char, Vec<&str>> = HashMap::new();
    
    for &word in &words {
        let first = word.chars().next().unwrap();
        grouped.entry(first).or_insert_with(Vec::new).push(word);
    }
    
    // Partition pattern
    let (evens, odds): (Vec<i32>, Vec<i32>) = numbers
        .into_iter()
        .partition(|x| x % 2 == 0);
}
```

### Building Complex Pipelines

```rust
fn process_data(data: Vec<i32>) -> Vec<i32> {
    data.into_iter()
        .filter(|x| x > 0)           // Keep positive
        .map(|x| x * 2)              // Double
        .filter(|x| *x < 100)        // Keep under 100
        .collect()
}

fn analyze_text(text: &str) -> HashMap<String, usize> {
    text.chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .to_lowercase()
        .chars()
        .fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c.to_string()).or_insert(0) += 1;
            acc
        })
}
```

---

## Chapter 12 Exercises

### Exercise 12.1: Closure Basics
```rust
// Create closures that:
// - Capture by reference
// - Capture by value (move)
// - Implement Fn, FnMut, FnOnce
// Test each with different functions
```

### Exercise 12.2: Iterator Adapters
```rust
// Given a Vec<i32>, use iterators to:
// - Filter even numbers
// - Square them
// - Take first 5
// - Collect to Vec
```

### Exercise 12.3: Custom Iterator
```rust
// Implement Iterator for:
// - Fibonacci sequence
// - Range with step
// - Alternating signs
```

### Exercise 12.4: Iterator Methods
```rust
// Practice all consuming adapters:
// - fold, reduce
// - all, any
// - find, position
// - max, min
```

### Exercise 12.5: Functional Pipeline
```rust
// Process a list of transactions:
// - Filter valid transactions
// - Group by category
// - Calculate totals
// - Find top spenders
```

---

## Summary

In this chapter, you learned:

✅ Closure syntax and type inference
✅ Capturing environment (Fn, FnMut, FnOnce)
✅ Move keyword for closures
✅ Closures as parameters and return values
✅ Iterator trait and creation
✅ Iterator adapters (lazy operations)
✅ Consuming adapters
✅ Custom iterator implementation
✅ Zero-cost abstractions
✅ Advanced iterator patterns
✅ Functional programming patterns

---

## What's Next?

Closures and iterators give you functional power, but what about smart memory management? In Chapter 13, we'll explore **Smart Pointers**.

**Continue to [Chapter 13: Smart Pointers](./13_smart_pointers.md)**
