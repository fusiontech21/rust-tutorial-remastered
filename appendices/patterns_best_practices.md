# Appendix B: Common Patterns & Best Practices

## Code Organization

### Module Structure

```rust
// src/lib.rs - Library root
pub mod utils;
pub mod models;
pub mod services;

pub use models::User;
pub use services::UserService;

// src/models/mod.rs - Module root
mod user;
mod post;

pub use user::User;
pub use post::Post;

// src/models/user.rs - Implementation
pub struct User {
    pub id: u32,
    pub name: String,
}
```

### Visibility

```rust
// Private by default
mod private {
    fn helper() {}  // Private
    pub fn public() {}  // Public within crate
}

// Public module
pub mod api {
    pub fn handle_request() {}  // Fully public
}

// Re-export
pub use internal::PublicType;

// crate:: for absolute paths
use crate::utils::helper;
```

## Error Handling Patterns

### Result Type Aliases

```rust
type AppResult<T> = Result<T, AppError>;
type IoResult<T> = std::io::Result<T>;
```

### Error Context

```rust
use anyhow::{Context, Result};

fn load_config() -> Result<Config> {
    let content = std::fs::read_to_string("config.toml")
        .context("Failed to read config file")?;
    
    let config: Config = toml::from_str(&content)
        .context("Failed to parse config")?;
    
    Ok(config)
}
```

### Early Return

```rust
fn process(data: Option<Data>) -> Result<(), Error> {
    let data = data.ok_or(Error::NoData)?;
    
    validate(&data)?;
    
    save(&data)?;
    
    Ok(())
}
```

## Ownership Patterns

### Borrowing

```rust
// Prefer borrowing
fn process(slice: &[i32]) { }
fn modify(slice: &mut [i32]) { }

// Return borrowed data
fn first_half(s: &str) -> &str {
    &s[..s.len() / 2]
}
```

### Cow (Clone on Write)

```rust
use std::borrow::Cow;

fn normalize(s: &str) -> Cow<str> {
    if s.trim() == s {
        Cow::Borrowed(s)
    } else {
        Cow::Owned(s.trim().to_string())
    }
}
```

## Type Patterns

### Newtype

```rust
struct Meters(f64);
struct Seconds(f64);

impl Meters {
    fn per_second(&self, seconds: Seconds) -> f64 {
        self.0 / seconds.0
    }
}
```

### Builder

```rust
#[derive(Default)]
struct Request {
    url: String,
    method: String,
    headers: HashMap<String, String>,
}

impl Request {
    fn builder(url: impl Into<String>) -> RequestBuilder {
        RequestBuilder {
            url: url.into(),
            method: "GET".to_string(),
            headers: HashMap::new(),
        }
    }
}

struct RequestBuilder {
    url: String,
    method: String,
    headers: HashMap<String, String>,
}

impl RequestBuilder {
    fn method(mut self, method: &str) -> Self {
        self.method = method.to_string();
        self
    }
    
    fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }
    
    fn build(self) -> Request {
        Request {
            url: self.url,
            method: self.method,
            headers: self.headers,
        }
    }
}
```

## Trait Patterns

### Marker Traits

```rust
trait Sendable {}
trait Serializable {}

impl Sendable for Data {}
impl Serializable for Data {}
```

### Trait Objects

```rust
trait Handler {
    fn handle(&self, request: &str) -> String;
}

fn process_handlers(handlers: &[Box<dyn Handler>], request: &str) {
    for handler in handlers {
        println!("{}", handler.handle(request));
    }
}
```

## Concurrency Patterns

### Channel Pattern

```rust
use std::sync::mpsc;

let (tx, rx) = mpsc::channel();

// Producer
thread::spawn(move || {
    tx.send(data).unwrap();
});

// Consumer
for item in rx {
    process(item);
}
```

### Arc<Mutex<T>> Pattern

```rust
use std::sync::{Arc, Mutex};

let shared = Arc::new(Mutex::new(Vec::new()));

for i in 0..10 {
    let shared = Arc::clone(&shared);
    thread::spawn(move || {
        let mut vec = shared.lock().unwrap();
        vec.push(i);
    });
}
```

## Performance Patterns

### Pre-allocation

```rust
// Bad: may reallocate multiple times
let mut vec = Vec::new();
for i in 0..1000 {
    vec.push(i);
}

// Good: allocate once
let mut vec = Vec::with_capacity(1000);
for i in 0..1000 {
    vec.push(i);
}
```

### Iterators

```rust
// Zero-cost abstraction
let sum: i32 = slice.iter()
    .filter(|&x| x > 0)
    .map(|x| x * 2)
    .sum();
```

### Slices

```rust
// Prefer slices over Vec
fn process(data: &[i32]) { }
fn process_vec(data: &Vec<i32>) { }  // Less flexible
```

## Testing Patterns

### Test Modules

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_feature() {
        assert_eq!(function(), expected);
    }
}
```

### Test Fixtures

```rust
fn create_test_user() -> User {
    User::new("Test", "test@example.com")
}

#[test]
fn test_user() {
    let user = create_test_user();
    assert!(user.is_valid());
}
```

## Documentation Patterns

### Doc Comments

```rust
/// Brief description
/// 
/// # Arguments
/// 
/// * `name` - Description
/// 
/// # Returns
/// 
/// Description of return value
/// 
/// # Examples
/// 
/// ```
/// let result = function(arg);
/// ```
/// 
/// # Panics
/// 
/// When this panics
/// 
/// # Errors
/// 
/// When this errors
pub fn function(name: &str) -> Result<(), Error> {
    Ok(())
}
```

## Naming Conventions

```rust
// Types: PascalCase
struct UserName;
enum Status { Active, Inactive }

// Functions/variables: snake_case
let user_name = String::new();
fn get_user() { }

// Constants: SCREAMING_SNAKE_CASE
const MAX_SIZE: usize = 100;

// Traits: PascalCase (often adjectives)
trait Clone;
trait IntoIterator;

// Modules: snake_case
mod user_handler;
mod db_utils;
```

## Common Anti-patterns to Avoid

```rust
// ❌ Don't clone unnecessarily
let s = expensive_string.clone();  // If you just need to borrow

// ✅ Do borrow
let s = &expensive_string;

// ❌ Don't use unwrap in library code
let value = result.unwrap();

// ✅ Do propagate errors
let value = result?;

// ❌ Don't use Vec when slice works
fn process(v: &Vec<i32>) { }

// ✅ Do use slices
fn process(v: &[i32]) { }

// ❌ Don't ignore Result
some_function_that_returns_result();

// ✅ Do handle it
some_function_that_returns_result()?;
let _ = some_function_that_returns_result();  // If intentionally ignoring
```
