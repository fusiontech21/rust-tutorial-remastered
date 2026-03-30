# Chapter 13: Smart Pointers

## 13.1 Introduction to Smart Pointers

### What are Smart Pointers?

Smart pointers are data structures that act like pointers but have additional metadata and capabilities. They own the data they point to and automatically manage memory.

```rust
// Regular reference (not a smart pointer)
let x = 5;
let r = &x;

// Smart pointer (Box<T>)
let b = Box::new(5);
```

### Common Smart Pointers

| Pointer | Description | Use Case |
|---------|-------------|----------|
| `Box<T>` | Heap allocation | Recursive types, large data |
| `Rc<T>` | Reference counting | Multiple owners (single-threaded) |
| `Arc<T>` | Atomic reference counting | Multiple owners (multi-threaded) |
| `Ref<T>`, `RefMut<T>` | Runtime borrow checking | Interior mutability |
| `Cell<T>` | Copy-based interior mutability | Simple interior mutability |

---

## 13.2 Box<T>

### Basic Usage

```rust
fn main() {
    // Allocate on heap
    let b = Box::new(5);
    println!("b = {}", b);
    
    // Box derefs automatically
    let x = 5;
    let y = Box::new(x);
    
    assert_eq!(5, *y);  // Dereference
    
    // Box can be moved
    let b1 = Box::new(String::from("hello"));
    let b2 = b1;  // Ownership transferred
    // println!("{}", b1);  // ERROR
    println!("{}", b2);  // OK
}
```

### Box for Recursive Types

```rust
// Can't have value of same type directly
// enum List {
//     Cons(i32, List),  // ERROR: infinite size
//     Nil,
// }

// Use Box for indirection
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));
    
    println!("{:?}", list);
}

// Print implementation
impl std::fmt::Debug for List {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Cons(val, next) => {
                write!(f, "{} -> {:?}", val, next)
            }
            Nil => write!(f, "Nil"),
        }
    }
}
```

### Box for Large Data

```rust
fn main() {
    // Large struct - allocate on heap to keep stack small
    struct LargeData {
        data: [u8; 10000],
    }
    
    // On stack (might cause stack overflow in tight spaces)
    // let data = LargeData { data: [0; 10000] };
    
    // On heap (preferred for large data)
    let data = Box::new(LargeData { data: [0; 10000] });
}
```

### Box for Trait Objects

```rust
trait Drawable {
    fn draw(&self);
}

struct Circle;
struct Rectangle;

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing circle");
    }
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!("Drawing rectangle");
    }
}

fn main() {
    // Vec of trait objects
    let shapes: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle),
        Box::new(Rectangle),
    ];
    
    for shape in shapes {
        shape.draw();
    }
}
```

---

## 13.3 Rc<T> - Reference Counted

### Basic Usage

```rust
use std::rc::Rc;

fn main() {
    // Create Rc
    let rc1 = Rc::new(String::from("hello"));
    
    // Clone increases reference count
    let rc2 = Rc::clone(&rc1);
    let rc3 = Rc::clone(&rc1);
    
    // All point to same data
    println!("rc1: {}", rc1);
    println!("rc2: {}", rc2);
    println!("rc3: {}", rc3);
    
    // Check reference count
    println!("Reference count: {}", Rc::strong_count(&rc1));  // 3
    
    // Drop decreases count
    drop(rc2);
    println!("After drop: {}", Rc::strong_count(&rc1));  // 2
    
    drop(rc3);
    println!("After drop: {}", Rc::strong_count(&rc1));  // 1
    
    // Last drop frees memory
    drop(rc1);
}
```

### Rc for Shared Ownership

```rust
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let shared = Rc::new(Cons(1,
        Box::new(Cons(2,
            Box::new(Nil)))));
    
    // Multiple owners of same list
    let a = shared;
    let b = Rc::clone(&shared);
    let c = Rc::clone(&shared);
    
    println!("Reference count: {}", Rc::strong_count(&a));
    
    // All share the same data
    println!("a: {:?}", a);
    println!("b: {:?}", b);
}
```

---

## 13.4 Arc<T> - Atomic Reference Counted

### Thread-Safe Reference Counting

```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let arc = Arc::new(5);
    
    let mut handles = vec![];
    
    for _ in 0..10 {
        let arc_clone = Arc::clone(&arc);
        
        let handle = thread::spawn(move || {
            println!("Value: {}", arc_clone);
        });
        
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Final count: {}", Arc::strong_count(&arc));
}
```

### Arc vs Rc

```rust
use std::rc::Rc;
use std::sync::Arc;

fn main() {
    // Rc - single-threaded, faster
    let rc = Rc::new(5);
    
    // Arc - multi-threaded, atomic operations
    let arc = Arc::new(5);
    
    // Arc can be sent between threads
    let arc_clone = Arc::clone(&arc);
    std::thread::spawn(move || {
        println!("{}", arc_clone);
    });
    
    // Rc cannot be sent between threads
    // let rc_clone = Rc::clone(&rc);
    // std::thread::spawn(move || {
    //     println!("{}", rc_clone);  // ERROR: Rc is not Send
    // });
}
```

---

## 13.5 Interior Mutability

### RefCell<T>

```rust
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(5);
    
    // Borrow mutably at runtime
    *data.borrow_mut() = 10;
    
    // Borrow immutably
    println!("Value: {}", data.borrow());
    
    // Runtime borrow checking
    let rc = RefCell::new(5);
    
    {
        let mut b1 = rc.borrow_mut();
        *b1 = 10;
        // b1 still borrowed here
    }  // b1 dropped, borrow ends
    
    // Now can borrow again
    let b2 = rc.borrow();
    println!("Value: {}", b2);
    
    // Panics if you violate borrowing rules at runtime
    // let b1 = rc.borrow_mut();
    // let b2 = rc.borrow();  // PANIC!
}
```

### Cell<T>

```rust
use std::cell::Cell;

fn main() {
    let cell = Cell::new(5);
    
    // Set and get (no borrowing)
    cell.set(10);
    println!("Value: {}", cell.get());
    
    // Useful for Copy types
    let counter = Cell::new(0);
    counter.set(counter.get() + 1);
    println!("Counter: {}", counter.get());
}
```

### Interior Mutability Pattern

```rust
use std::cell::RefCell;

#[derive(Debug)]
struct Messenger {
    sent: RefCell<usize>,
}

impl Messenger {
    fn new() -> Self {
        Messenger { sent: RefCell::new(0) }
    }
}

fn send_message(messenger: &Messenger, msg: &str) {
    println!("Sending: {}", msg);
    *messenger.sent.borrow_mut() += 1;
}

fn main() {
    let messenger = Messenger::new();
    
    send_message(&messenger, "Hello");
    send_message(&messenger, "World");
    
    println!("Messages sent: {}", *messenger.sent.borrow());
}
```

---

## 13.6 Combining Smart Pointers

### Rc<RefCell<T>>

```rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Data {
    value: i32,
}

fn main() {
    // Multiple owners + interior mutability
    let data = Rc::new(RefCell::new(Data { value: 5 }));
    
    let data2 = Rc::clone(&data);
    let data3 = Rc::clone(&data);
    
    // Mutate through any owner
    data2.borrow_mut().value = 10;
    
    println!("data: {:?}", data.borrow());
    println!("data2: {:?}", data2.borrow());
    println!("data3: {:?}", data3.borrow());
    
    println!("Reference count: {}", Rc::strong_count(&data));
}
```

### Arc<Mutex<T>>

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Counter: {}", *counter.lock().unwrap());
}
```

---

## 13.7 Weak References

### Breaking Reference Cycles

```rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    
    println!("leaf parent: {:?}", leaf.parent.borrow().upgrade());
    
    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    
    // Set parent using Weak reference
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    
    println!("leaf parent: {:?}", leaf.parent.borrow().upgrade());
    
    println!("Reference counts:");
    println!(" leaf strong: {}, weak: {}", 
             Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    println!(" branch strong: {}, weak: {}", 
             Rc::strong_count(&branch), Rc::weak_count(&branch));
}
```

---

## 13.8 Smart Pointer Patterns

### Deref Trait

```rust
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Implement deref for automatic dereferencing
impl<T> Deref for MyBox<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let x = MyBox::new(5);
    
    // Automatic deref
    println!("{}", x);  // Uses Deref
    println!("{}", *x);  // Explicit deref
}
```

### Deref Coercion

```rust
fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let m = Box::new(String::from("Rust"));
    
    // Box<String> -> &String -> &str
    hello(&m);
    
    let s = String::from("Rust");
    hello(&s);
}
```

### Drop Trait

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping {}", self.data);
    }
}

fn main() {
    let a = CustomSmartPointer { data: String::from("a") };
    let b = CustomSmartPointer { data: String::from("b") };
    
    println!("Created smart pointers");
    
    // Explicit drop (moves out of scope)
    drop(a);
    println!("Dropped a explicitly");
    
    // b dropped at end of scope
}
```

---

## Chapter 13 Exercises

### Exercise 13.1: Box Practice
```rust
// Create a recursive enum using Box
// Implement methods to:
// - Calculate sum
// - Find depth
// - Convert to Vec
```

### Exercise 13.2: Rc/Arc Practice
```rust
// Create a graph structure with shared nodes
// Use Rc for single-threaded version
// Convert to Arc for multi-threaded version
```

### Exercise 13.3: Interior Mutability
```rust
// Create a counter using RefCell
// Create a cache using Cell
// Implement observer pattern with interior mutability
```

### Exercise 13.4: Weak References
```rust
// Create parent-child relationships
// Use Weak to prevent cycles
// Implement tree with parent pointers
```

### Exercise 13.5: Smart Pointer Combinations
```rust
// Combine Rc<RefCell<T>> for shared mutable state
// Combine Arc<Mutex<T>> for thread-safe state
// Practice deref coercion
```

---

## Summary

In this chapter, you learned:

✅ Box<T> for heap allocation
✅ Rc<T> for reference counting
✅ Arc<T> for atomic reference counting
✅ RefCell<T> for runtime borrow checking
✅ Cell<T> for copy-based interior mutability
✅ Interior mutability pattern
✅ Combining smart pointers
✅ Weak references to break cycles
✅ Deref and Drop traits
✅ Deref coercion

---

## What's Next?

Smart pointers help manage memory, but Rust truly shines in concurrent programming! In Chapter 14, we'll explore **Concurrency and Async Programming**.

**Continue to [Chapter 14: Concurrency & Async](./14_concurrency_async.md)**
