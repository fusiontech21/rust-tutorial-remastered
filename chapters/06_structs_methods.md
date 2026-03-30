# Chapter 6: Structs & Methods

## 6.1 Defining Structs

### Struct Syntax

```rust
// Classic struct with named fields
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // Create an instance
    let user1 = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        sign_in_count: 1,
        active: true,
    };
    
    // Access fields
    println!("Username: {}", user1.username);
    println!("Email: {}", user1.email);
    
    // Modify mutable fields
    let mut user2 = User {
        username: String::from("bob"),
        email: String::from("bob@example.com"),
        sign_in_count: 0,
        active: true,
    };
    
    user2.sign_in_count = 1;
    user2.active = false;
}
```

### Field Init Shorthand

```rust
fn main() {
    let username = String::from("alice");
    let email = String::from("alice@example.com");
    
    // Verbose way
    let user1 = User {
        username: username,
        email: email,
        sign_in_count: 1,
        active: true,
    };
    
    // Shorthand (when parameter name matches field name)
    let user2 = User {
        username,  // Same as username: username
        email,
        sign_in_count: 1,
        active: true,
    };
}
```

### Struct Update Syntax

```rust
fn main() {
    let user1 = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        sign_in_count: 1,
        active: true,
    };
    
    // Create user2 with most fields from user1
    let user2 = User {
        username: String::from("bob"),
        email: String::from("bob@example.com"),
        ..user1  // Use remaining fields from user1
    };
    
    // Note: user1.username and user1.email are moved to user2
    // println!("{}", user1.username);  // ERROR
    println!("{}", user2.username);  // OK
}
```

---

## 6.2 Tuple Structs

```rust
// Tuple struct - fields don't have names
struct Color(u8, u8, u8);
struct Point(f64, f64, f64);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0.0, 0.0, 0.0);
    
    // Access fields by index
    println!("Red: {}", black.0);
    println!("X: {}", origin.0);
    
    // Destructure
    let Color(r, g, b) = black;
    println!("RGB: ({}, {}, {})", r, g, b);
    
    let Point(x, y, z) = origin;
    println!("Point: ({}, {}, {})", x, y, z);
    
    // Pattern matching
    match origin {
        Point(0.0, 0.0, 0.0) => println!("At origin"),
        Point(x, y, z) => println!("At ({}, {}, {})", x, y, z),
    }
}
```

---

## 6.3 Unit-Like Structs

```rust
// Struct with no fields
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
    
    // Useful for implementing traits
    // Or as a marker type
}

// Common use case: Marker trait implementation
struct Millimeters(u32);
struct Meters(u32);

fn main() {
    let length = Millimeters(1000);
    // Type system prevents mixing units
}
```

---

## 6.4 Printing Structs

### Debug Trait

```rust
// Derive Debug for printing
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    
    // Debug formatting
    println!("rect = {:?}", rect);
    
    // Pretty debug formatting
    println!("rect = {:#?}", rect);
    
    // dbg! macro for debugging
    let scale = 2;
    let rect = Rectangle {
        width: dbg!(30 * scale),  // Prints and returns value
        height: 50,
    };
}
```

### Display Trait

```rust
use std::fmt;

struct Point {
    x: f64,
    y: f64,
}

// Implement custom Display
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point({}, {})", self.x, self.y)
    }
}

fn main() {
    let p = Point { x: 3.0, y: 4.0 };
    println!("{}", p);  // Uses Display: "Point(3, 4)"
    println!("{:?}", p);  // ERROR: Debug not implemented
}

// Derive both
#[derive(Debug, Display)]  // Display can't be derived, must implement
struct Circle {
    radius: f64,
}
```

---

## 6.5 Methods

### Implementing Methods

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Method (takes &self)
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // Another method
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
    
    // Method with additional parameters
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }
    
    // Method that returns modified copy
    fn scaled(&self, factor: u32) -> Rectangle {
        Rectangle {
            width: self.width * factor,
            height: self.height * factor,
        }
    }
    
    // Associated function (no self)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
    
    // Multiple parameters
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let mut rect = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!("Area: {}", rect.area());
    println!("Perimeter: {}", rect.perimeter());
    
    rect.scale(2);
    println!("Scaled: {:?}", rect);
    
    let rect2 = rect.scaled(3);
    println!("Rect2: {:?}", rect2);
    
    // Using associated function
    let square = Rectangle::square(10);
    println!("Square: {:?}", square);
    
    let small = Rectangle {
        width: 5,
        height: 5,
    };
    
    println!("Can hold: {}", rect.can_hold(&small));
}
```

### Method Syntax vs Function Syntax

```rust
#[derive(Debug)]
struct Circle {
    radius: f64,
}

impl Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

fn main() {
    let c = Circle { radius: 5.0 };
    
    // Method syntax (sugar)
    let area1 = c.area();
    
    // Function syntax (what it compiles to)
    let area2 = Circle::area(&c);
    
    println!("Area1: {}, Area2: {}", area1, area2);
}
```

---

## 6.6 Multiple impl Blocks

```rust
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
}

// Methods
impl User {
    fn new(username: String, email: String) -> Self {
        User {
            username,
            email,
            active: true,
        }
    }
    
    fn is_active(&self) -> bool {
        self.active
    }
}

// Separate impl for organization
impl User {
    fn deactivate(&mut self) {
        self.active = false;
    }
    
    fn activate(&mut self) {
        self.active = true;
    }
}

fn main() {
    let mut user = User::new(
        String::from("alice"),
        String::from("alice@example.com"),
    );
    
    println!("Active: {}", user.is_active());
    user.deactivate();
    println!("Active: {}", user.is_active());
}
```

---

## 6.7 Self Keyword

```rust
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    // &self - immutable borrow
    fn get_name(&self) -> &str {
        &self.name
    }
    
    // &mut self - mutable borrow
    fn birthday(&mut self) {
        self.age += 1;
    }
    
    // self - takes ownership
    fn into_name(self) -> String {
        self.name
    }
    
    // Self as return type
    fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }
    
    // Self in return position
    fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
}

fn main() {
    let person = Person::new(String::from("Alice"), 30);
    println!("Name: {}", person.get_name());
    
    let name = person.into_name();  // person is moved
    println!("Name: {}", name);
}
```

---

## 6.8 Associated Functions (Constructors)

```rust
#[derive(Debug)]
struct Config {
    host: String,
    port: u16,
    timeout: u32,
}

impl Config {
    // Constructor
    fn new(host: String, port: u16) -> Self {
        Config {
            host,
            port,
            timeout: 30,  // Default
        }
    }
    
    // Builder pattern
    fn with_timeout(mut self, timeout: u32) -> Self {
        self.timeout = timeout;
        self
    }
    
    // Default implementation
    fn default() -> Self {
        Config {
            host: String::from("localhost"),
            port: 8080,
            timeout: 30,
        }
    }
    
    // Factory method
    fn from_env() -> Self {
        Config {
            host: std::env::var("HOST").unwrap_or_else(|_| "localhost".to_string()),
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .unwrap_or(8080),
            timeout: 30,
        }
    }
}

fn main() {
    let config1 = Config::new(String::from("example.com"), 443);
    let config2 = Config::default();
    let config3 = Config::from_env();
    
    // Builder pattern
    let config4 = Config::new(String::from("api.com"), 80)
        .with_timeout(60);
    
    println!("{:?}", config1);
    println!("{:?}", config2);
}
```

---

## 6.9 Structs with Generic Types

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
    
    fn x(&self) -> &T {
        &self.x
    }
    
    fn y(&self) -> &T {
        &self.y
    }
}

// Implement for specific type
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Implement with different generic
impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let integer = Point::new(5, 10);
    let float = Point::new(1.0, 4.0);
    
    println!("Integer x: {}", integer.x());
    println!("Float distance: {}", float.distance_from_origin());
    
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: "Hello", y: "World" };
    
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```

---

## 6.10 Const in Methods

```rust
struct Data {
    value: i32,
}

impl Data {
    // const fn can be used in const contexts
    const fn new(value: i32) -> Self {
        Data { value }
    }
    
    const fn get_value(&self) -> i32 {
        self.value
    }
    
    const fn double(&self) -> i32 {
        self.value * 2
    }
}

const DATA: Data = Data::new(42);
const DOUBLED: i32 = DATA.double();

fn main() {
    println!("Data: {}", DATA.get_value());
    println!("Doubled: {}", DOUBLED);
}
```

---

## 6.11 Visibility (Privacy)

```rust
// Public struct
pub struct Rectangle {
    // Public fields
    pub width: u32,
    pub height: u32,
    
    // Private fields
    internal_id: u64,
}

impl Rectangle {
    // Public method
    pub fn new(width: u32, height: u32) -> Self {
        Rectangle {
            width,
            height,
            internal_id: 0,
        }
    }
    
    // Private method
    fn calculate_id(&self) -> u64 {
        self.width as u64 * self.height as u64
    }
    
    // Public method using private method
    pub fn get_id(&self) -> u64 {
        self.calculate_id()
    }
}

// Module example
mod inner {
    pub struct PublicStruct;      // Accessible outside
    struct PrivateStruct;         // Only accessible in this module
    
    pub struct Mixed {
        pub field: i32,           // Public field
        private: i32,             // Private field
    }
}

fn main() {
    let rect = Rectangle::new(30, 50);
    println!("Width: {}", rect.width);
    println!("ID: {}", rect.get_id());
    
    // let id = rect.internal_id;  // ERROR: private field
    // rect.calculate_id();  // ERROR: private method
}
```

---

## 6.12 Common Patterns

### Newtype Pattern

```rust
// Wrapper type for type safety
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

impl Feet {
    fn new(value: f64) -> Self {
        Feet(value)
    }
}

fn main() {
    let meters = Meters::new(10.0);
    let feet = meters.to_feet();
    
    // Type system prevents mixing units
    // let wrong = meters.0 + feet.0;  // Type mismatch
}
```

### Builder Pattern

```rust
#[derive(Debug)]
struct Request {
    url: String,
    method: String,
    headers: Vec<(String, String)>,
    body: Option<String>,
    timeout: u32,
}

struct RequestBuilder {
    url: String,
    method: String,
    headers: Vec<(String, String)>,
    body: Option<String>,
    timeout: u32,
}

impl RequestBuilder {
    fn new(url: String) -> Self {
        RequestBuilder {
            url,
            method: "GET".to_string(),
            headers: Vec::new(),
            body: None,
            timeout: 30,
        }
    }
    
    fn method(mut self, method: &str) -> Self {
        self.method = method.to_string();
        self
    }
    
    fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.push((key.to_string(), value.to_string()));
        self
    }
    
    fn body(mut self, body: String) -> Self {
        self.body = Some(body);
        self
    }
    
    fn timeout(mut self, timeout: u32) -> Self {
        self.timeout = timeout;
        self
    }
    
    fn build(self) -> Request {
        Request {
            url: self.url,
            method: self.method,
            headers: self.headers,
            body: self.body,
            timeout: self.timeout,
        }
    }
}

fn main() {
    let request = RequestBuilder::new("https://api.example.com".to_string())
        .method("POST")
        .header("Content-Type", "application/json")
        .header("Authorization", "Bearer token")
        .body("{\"key\": \"value\"}".to_string())
        .timeout(60)
        .build();
    
    println!("{:?}", request);
}
```

---

## Chapter 6 Exercises

### Exercise 6.1: Basic Struct
```rust
// Create a Book struct with:
// - title: String
// - author: String
// - pages: u32
// - current_page: u32
// Implement methods: new, progress(), pages_remaining()
```

### Exercise 6.2: Tuple Struct
```rust
// Create a Color tuple struct with RGBA values (u8)
// Implement methods to:
// - Convert to grayscale
// - Adjust alpha (transparency)
// - Mix two colors
```

### Exercise 6.3: Builder Pattern
```rust
// Implement a ServerConfig builder with:
// - host, port, max_connections, timeout
// - Fluent interface (method chaining)
// - Validation before build
```

### Exercise 6.4: Generic Struct
```rust
// Create a generic Container<T> struct
// Implement methods: new, get, set, map
// Test with different types
```

### Exercise 6.5: Privacy Practice
```rust
// Create a module with public and private structs
// Implement public API with private helper methods
// Demonstrate encapsulation
```

---

## Summary

In this chapter, you learned:

✅ Defining structs with named fields
✅ Tuple structs and unit-like structs
✅ Field init shorthand and update syntax
✅ Deriving traits (Debug, Clone, etc.)
✅ Implementing methods with `impl`
✅ Associated functions (constructors)
✅ The `self` keyword variations
✅ Multiple `impl` blocks
✅ Generic structs
✅ Visibility and privacy rules
✅ Common patterns (Builder, Newtype)

---

## What's Next?

Structs let you create custom data types, but what about types that can be one of several variants? In Chapter 7, we'll explore **Enums and Pattern Matching**.

**Continue to [Chapter 7: Enums & Pattern Matching](./07_enums_pattern_matching.md)**
