# Chapter 17: Advanced Topics & Ecosystem

## 17.1 Unsafe Rust

### Unsafe Superpowers

```rust
// 1. Dereference raw pointers
// 2. Call unsafe functions
// 3. Access mutable static variables
// 4. Implement unsafe traits
// 5. Access union fields
```

### Raw Pointers

```rust
fn main() {
    let mut num = 5;
    
    // Create raw pointers
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    
    // Dereference in unsafe block
    unsafe {
        println!("r1 is: {}", *r1);
        *r2 = 10;
        println!("num is now: {}", num);
    }
    
    // Raw pointers can be null
    let null_ptr: *const i32 = std::ptr::null();
    
    // Raw pointers don't implement Send/Sync automatically
}
```

### Unsafe Functions

```rust
unsafe fn dangerous() {
    println!("This is dangerous!");
}

fn main() {
    // Must call in unsafe block
    unsafe {
        dangerous();
    }
}

// Safe wrapper pattern
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    assert!(mid <= len);
    
    unsafe {
        let ptr = slice.as_mut_ptr();
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
```

### Mutable Static

```rust
static mut COUNTER: i32 = 0;

fn add_to_count(inc: i32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_count(5);
    unsafe {
        println!("Counter: {}", COUNTER);
    }
}
```

### Unsafe Traits

```rust
unsafe trait Foo {
    fn method(&self);
}

unsafe impl Foo for i32 {
    fn method(&self) {
        println!("Unsafe implementation");
    }
}

fn main() {
    let x = 5;
    x.method();
}
```

---

## 17.2 Advanced Traits

### Trait Objects in Detail

```rust
trait Draw {
    fn draw(&self);
}

struct Button {
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing button: {}", self.label);
    }
}

struct TextField {
    content: String,
}

impl Draw for TextField {
    fn draw(&self) {
        println!("Drawing text field: {}", self.content);
    }
}

// Boxed trait objects
fn draw_all(components: Vec<Box<dyn Draw>>) {
    for component in components {
        component.draw();
    }
}

// Reference to trait objects
fn draw_all_ref(components: &[&dyn Draw]) {
    for component in components {
        component.draw();
    }
}
```

### Object Safety

```rust
// Object-safe traits can be trait objects

// ✅ Object-safe
trait Cloneable {
    fn clone(&self) -> Self;
}

// ❌ Not object-safe (returns Self)
trait NotObjectSafe {
    fn create() -> Self;
}

// ❌ Not object-safe (generic parameter)
trait AlsoNotObjectSafe {
    fn process<T>(&self, item: T);
}
```

---

## 17.3 Advanced Types

### Newtype Pattern

```rust
// Type-safe wrappers
struct Meters(f64);
struct Feet(f64);

impl Meters {
    fn new(value: f64) -> Self {
        Meters(value)
    }
    
    fn to_feet(&self) -> Feet {
        Feet(self.0 * 3.28084)
    }
}

// PhantomData for type markers
use std::marker::PhantomData;

struct Inches(i32);

struct Distance<Unit> {
    value: f64,
    _unit: PhantomData<Unit>,
}

impl Distance<Meters> {
    fn new(value: f64) -> Self {
        Distance {
            value,
            _unit: PhantomData,
        }
    }
}
```

### Type Aliases

```rust
// Simple alias
type Result<T> = std::result::Result<T, std::io::Error>;

// Generic alias
type Thunk = Box<dyn Fn() + Send + 'static>;

// In structs
type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}
```

### Never Type

```rust
// The never type (!)
fn bar() -> ! {
    panic!();
}

fn get_option() -> Option<i32> {
    let result: Result<i32, &str> = Ok(5);
    
    Some(result.unwrap_or_else(|_| {
        panic!("Should be Ok");
    }))
}

// In match
let x: Result<i32, &str> = Ok(5);
let value = match x {
    Ok(n) => n,
    Err(_) => return,  // ! coerces to any type
};
```

---

## 17.4 Advanced Functions

### Function Pointers

```rust
fn add_one(x: i32) -> i32 {
    x + 1
}

fn apply<F>(f: F, arg: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(arg)
}

fn apply_fn_pointer(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg)
}

fn main() {
    let f: fn(i32) -> i32 = add_one;
    let result = apply_fn_pointer(f, 5);
    println!("{}", result);
    
    // Array of function pointers
    let operations: [fn(i32, i32) -> i32; 4] = [
        |a, b| a + b,
        |a, b| a - b,
        |a, b| a * b,
        |a, b| a / b,
    ];
    
    for op in operations {
        println!("{}", op(10, 2));
    }
}
```

### Closures in Depth

```rust
// Closure types
fn make_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

// Closure return types
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

// Closure as struct field
struct Processor<F> {
    process: F,
}

impl<F: Fn(i32) -> i32> Processor<F> {
    fn new(process: F) -> Self {
        Processor { process }
    }
    
    fn execute(&self, input: i32) -> i32 {
        (self.process)(input)
    }
}
```

---

## 17.5 Advanced Lifetimes

### Lifetime Subtyping

```rust
// 'b must outlive 'a
fn process<'a, 'b: 'a>(x: &'a i32, y: &'b i32) -> &'a i32 {
    x
}

// In structs
struct Ref<'a, 'b: 'a> {
    x: &'a i32,
    y: &'b i32,
}
```

### HRTB (Higher-Ranked Trait Bounds)

```rust
// For all lifetimes
fn call_with_ref<F>(f: F)
where
    F: for<'a> Fn(&'a str),
{
    let s = String::from("hello");
    f(&s);
}

// In trait bounds
trait Processor {
    fn process(&self) -> for<'a> fn(&'a str) -> &'a str;
}
```

---

## 17.6 The Rust Ecosystem

### Essential Crates

```toml
[dependencies]
# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Error handling
anyhow = "1.0"
thiserror = "1.0"

# Async runtime
tokio = { version = "1", features = ["full"] }
async-std = "1.12"

# HTTP client
reqwest = { version = "0.11", features = ["json"] }

# Web framework
axum = "0.7"
actix-web = "4"

# CLI
clap = { version = "4", features = ["derive"] }
colored = "2"

# Logging
log = "0.4"
env_logger = "0.10"
tracing = "0.1"

# Testing
mockall = "0.12"
proptest = "1.4"
criterion = "0.5"

# Utilities
chrono = "0.4"
rand = "0.8"
regex = "1"
itertools = "0.12"
rayon = "1.8"
```

### Project Structure

```
my_project/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── LICENSE
├── .gitignore
├── src/
│   ├── main.rs          # Binary entry point
│   ├── lib.rs           # Library root
│   ├── bin/             # Additional binaries
│   │   └── other.rs
│   ├── modules/
│   │   ├── mod.rs
│   │   ├── submodule.rs
│   │   └── private.rs
│   └── utils/
│       └── helpers.rs
├── tests/               # Integration tests
│   ├── common/
│   │   └── mod.rs
│   └── integration_test.rs
├── benches/             # Benchmarks
│   └── benchmark.rs
├── examples/            # Examples
│   └── basic_usage.rs
└── build.rs             # Build script
```

---

## 17.7 Build Scripts

### build.rs

```rust
// build.rs
use std::env;

fn main() {
    // Set environment variables
    println!("cargo:rustc-env=BUILD_TIME={}", 
             std::time::SystemTime::now()
                 .duration_since(std::time::UNIX_EPOCH)
                 .unwrap()
                 .as_secs());
    
    // Re-run if file changes
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=data/config.json");
    
    // Link external libraries
    println!("cargo:rustc-link-lib=ssl");
    println!("cargo:rustc-link-search=native=/usr/lib");
    
    // Compile native code
    cc::Build::new()
        .file("src/native.c")
        .compile("native");
}
```

---

## 17.8 FFI (Foreign Function Interface)

### Calling C from Rust

```rust
use std::ffi::CStr;
use std::os::raw::c_char;

extern "C" {
    fn printf(format: *const c_char, ...) -> i32;
    fn strlen(s: *const c_char) -> usize;
}

fn main() {
    unsafe {
        let c_str = std::ffi::CString::new("Hello from Rust!\n").unwrap();
        printf(c_str.as_ptr());
    }
}
```

### Calling Rust from C

```rust
// src/lib.rs
use std::os::raw::c_char;
use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn rust_add(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub extern "C" fn rust_process_string(s: *const c_char) -> *const c_char {
    unsafe {
        if s.is_null() {
            return std::ptr::null();
        }
        // Process string
        s
    }
}
```

---

## 17.9 Performance Optimization

### Profiling

```bash
# Build with debug symbols
cargo build --release

# Use perf (Linux)
perf record target/release/myapp
perf report

# Use samply (cross-platform)
cargo install samply
samply record target/release/myapp
```

### Optimization Tips

```rust
// Use iterators (zero-cost)
let sum: i32 = (0..1000).map(|x| x * 2).sum();

// Pre-allocate collections
let mut vec = Vec::with_capacity(1000);

// Use slices instead of Vec when possible
fn process(slice: &[i32]) { }

// Avoid unnecessary clones
fn borrow(s: &str) { }
fn clone(s: String) { }  // Only if you need ownership

// Use Cow for clone-on-write
use std::borrow::Cow;
fn process(s: Cow<str>) { }

// Inline small functions
#[inline]
fn small_function() { }

#[inline(always)]
fn critical_function() { }
```

---

## 17.10 Common Patterns

### Builder Pattern

```rust
#[derive(Default)]
struct ServerConfig {
    host: String,
    port: u16,
    timeout: u32,
}

impl ServerConfig {
    fn host(mut self, host: impl Into<String>) -> Self {
        self.host = host.into();
        self
    }
    
    fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }
    
    fn timeout(mut self, timeout: u32) -> Self {
        self.timeout = timeout;
        self
    }
    
    fn build(self) -> Result<Server, ConfigError> {
        // Validate and build
        Ok(Server { config: self })
    }
}
```

### RAII Pattern

```rust
struct FileGuard {
    file: std::fs::File,
}

impl FileGuard {
    fn open(path: &str) -> std::io::Result<Self> {
        Ok(FileGuard {
            file: std::fs::File::open(path)?,
        })
    }
}

impl Drop for FileGuard {
    fn drop(&mut self) {
        // Cleanup automatically
        println!("File closed");
    }
}
```

### Typestate Pattern

```rust
struct Request<B> {
    body: B,
}

struct NoBody;
struct WithBody(String);

impl Request<NoBody> {
    fn new() -> Self {
        Request { body: NoBody }
    }
    
    fn with_body(self, body: String) -> Request<WithBody> {
        Request { body: WithBody(body) }
    }
}

impl Request<WithBody> {
    fn send(self) {
        println!("Sending with body: {}", self.body.0);
    }
}
```

---

## Chapter 17 Exercises

### Exercise 17.1: Unsafe Rust
```rust
// Practice raw pointers
// Create safe wrapper around unsafe code
// Implement custom allocator
```

### Exercise 17.2: Advanced Types
```rust
// Use PhantomData for type markers
// Implement newtype pattern
// Create type-safe units
```

### Exercise 17.3: FFI
```rust
// Call C library from Rust
// Export Rust functions to C
// Handle string conversions
```

### Exercise 17.4: Optimization
```rust
// Profile a slow program
// Apply optimization techniques
// Benchmark improvements
```

### Exercise 17.5: Project
```rust
// Create a complete Rust project
// Use essential crates
// Follow best practices
// Publish to crates.io
```

---

## Summary

In this chapter, you learned:

✅ Unsafe Rust and when to use it
✅ Raw pointers and unsafe functions
✅ Advanced trait patterns
✅ Advanced type system features
✅ Function pointers and closures
✅ Advanced lifetime patterns
✅ Essential crates in the ecosystem
✅ Project structure and organization
✅ Build scripts
✅ FFI with C
✅ Performance optimization
✅ Common Rust patterns (Builder, RAII, Typestate)

---

## Congratulations!

You've completed **The Ultimate Rust Tutorial**! 

You now have comprehensive knowledge of:
- Rust fundamentals (syntax, types, control flow)
- Ownership and borrowing
- Structs, enums, and pattern matching
- Collections and error handling
- Generics, traits, and lifetimes
- Functional features (closures, iterators)
- Smart pointers
- Concurrency and async programming
- Macros
- Testing and documentation
- Advanced topics and the ecosystem

## Next Steps

1. **Build Projects**: Apply your knowledge to real projects
2. **Read Rust Code**: Study popular open-source Rust projects
3. **Join the Community**: Participate in forums, Discord, local meetups
4. **Contribute**: Submit PRs to Rust projects
5. **Stay Updated**: Follow This Week in Rust, Rust blog

## Resources

- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Standard Library](https://doc.rust-lang.org/std/)
- [Crates.io](https://crates.io/)
- [This Week in Rust](https://this-week-in-rust.org/)

**Happy Rust coding! 🦀**
