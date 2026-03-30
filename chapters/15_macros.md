# Chapter 15: Macros

## 15.1 Macros vs Functions

### Why Macros?

```rust
// Functions: runtime abstraction
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Macros: compile-time code generation
macro_rules! add {
    ($a:expr, $b:expr) => {
        $a + $b
    };
}

fn main() {
    let x = add(5, 3);      // Function call
    let y = add!(5, 3);     // Macro invocation
}
```

### Differences

| Feature | Functions | Macros |
|---------|-----------|--------|
| Expansion | Runtime | Compile-time |
| Type system | Checked | After expansion |
| Variadic | No (without slices) | Yes |
| Code generation | No | Yes |

---

## 15.2 Declarative Macros (macro_rules!)

### Basic Syntax

```rust
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

fn main() {
    say_hello!();
}
```

### Macros with Parameters

```rust
macro_rules! add {
    ($a:expr, $b:expr) => {
        $a + $b
    };
}

macro_rules! print_value {
    ($value:expr) => {
        println!("Value: {}", $value);
    };
}

fn main() {
    let result = add!(5, 3);
    println!("Result: {}", result);
    
    print_value!(42);
    print_value!("hello");
}
```

### Multiple Rules

```rust
macro_rules! operation {
    (add, $a:expr, $b:expr) => {
        $a + $b
    };
    (sub, $a:expr, $b:expr) => {
        $a - $b
    };
    (mul, $a:expr, $b:expr) => {
        $a * $b
    };
    (div, $a:expr, $b:expr) => {
        $a / $b
    };
}

fn main() {
    println!("Add: {}", operation!(add, 10, 5));
    println!("Sub: {}", operation!(sub, 10, 5));
    println!("Mul: {}", operation!(mul, 10, 5));
    println!("Div: {}", operation!(div, 10, 5));
}
```

---

## 15.3 Fragment Specifiers

### Available Specifiers

```rust
macro_rules! demo {
    // Expression
    ($e:expr) => {
        println!("Expression: {}", $e);
    };
    
    // Block
    ($b:block) => {
        $b
    };
    
    // Statement
    ($s:stmt) => {
        $s
    };
    
    // Pattern
    ($p:pat) => {
        let $p = 5;
    };
    
    // Type
    ($t:ty) => {
        let x: $t = 5;
    };
    
    // Identifier
    ($i:ident) => {
        let $i = 5;
    };
    
    // Path
    ($path:path) => {
        let x: $path = 5;
    };
    
    // Lifetime
    ($l:lifetime) => {
        struct Ref<'$l> {
            r: &'$l str,
        }
    };
    
    // Literal
    ($lit:literal) => {
        println!("Literal: {}", $lit);
    };
    
    // Constant
    ($c:const) => {
        const C: i32 = $c;
    };
    
    // Item
    ($item:item) => {
        $item
    };
    
    // Meta (attribute content)
    ($m:meta) => {
        #[$m]
        struct S;
    };
}
```

---

## 15.4 Repetition

### Zero or More (*)

```rust
macro_rules! vec_of_strings {
    ($($x:expr),*) => {
        vec![$(String::from($x)),*]
    };
}

fn main() {
    let v = vec_of_strings!("a", "b", "c");
    println!("{:?}", v);
}
```

### One or More (+)

```rust
macro_rules! at_least_one {
    ($($x:expr),+) => {
        vec![$($x),*]
    };
}

fn main() {
    let v = at_least_one!(1, 2, 3);
    // let empty = at_least_one!();  // ERROR: needs at least one
}
```

### Zero or One (?)

```rust
macro_rules! maybe_semicolon {
    ($s:stmt $semi:tt?) => {
        $s $semi
    };
}
```

### Separator

```rust
macro_rules! sum {
    ($($x:expr),* $(,)?) => {
        {
            let mut sum = 0;
            $(sum += $x;)*
            sum
        }
    };
}

fn main() {
    println!("{}", sum!(1, 2, 3, 4, 5));
    println!("{}", sum!(1, 2, 3,));  // Trailing comma OK
}
```

---

## 15.5 Advanced Macro Patterns

### Recursive Macros

```rust
macro_rules! factorial {
    (0) => { 1 };
    ($n:expr) => {
        $n * factorial!($n - 1)
    };
}

fn main() {
    println!("5! = {}", factorial!(5));
}
```

### Internal Rules

```rust
macro_rules! calculator {
    // Internal rule
    (@calc $a:expr + $b:expr) => {
        $a + $b
    };
    (@calc $a:expr - $b:expr) => {
        $a - $b
    };
    
    // Public interface
    ($a:expr + $b:expr) => {
        calculator!(@calc $a + $b)
    };
    ($a:expr - $b:expr) => {
        calculator!(@calc $a - $b)
    };
}

fn main() {
    println!("{}", calculator!(5 + 3));
    println!("{}", calculator!(10 - 4));
}
```

### TT Muncher

```rust
macro_rules! print_types {
    () => {};
    ($name:ident: $ty:ty, $($rest:tt)*) => {
        println!("{}: {}", stringify!($name), stringify!($ty));
        print_types!($($rest)*);
    };
}

fn main() {
    print_types!(
        x: i32,
        y: String,
        z: Vec<u8>,
    );
}
```

---

## 15.6 Procedural Macros

### Types of Procedural Macros

```rust
// 1. Function-like macros: custom!()
// 2. Derive macros: #[derive(Custom)]
// 3. Attribute macros: #[custom_attribute]
// 4. Derive helper attributes: #[serde(rename = "name")]
```

### Function-like Procedural Macro

```rust
// In proc-macro crate (Cargo.toml: [lib] proc-macro = true)

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn hello(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    
    let expanded = quote! {
        println!("Hello, {}!", #input);
    };
    
    TokenStream::from(expanded)
}

// Usage:
// hello!("World");  // Expands to: println!("Hello, {}!", "World");
```

### Derive Macro

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Hello)]
pub fn hello_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    
    let expanded = quote! {
        impl Hello for #name {
            fn hello(&self) {
                println!("Hello from {}!", stringify!(#name));
            }
        }
    };
    
    TokenStream::from(expanded)
}

// Usage:
// #[derive(Hello)]
// struct MyStruct;
```

### Attribute Macro

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn logged(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let name = &input.sig.ident;
    
    let expanded = quote! {
        fn #name() {
            println!("Calling {}", stringify!(#name));
            #input
            println!("Finished {}", stringify!(#name));
        }
    };
    
    TokenStream::from(expanded)
}

// Usage:
// #[logged]
// fn my_function() { }
```

---

## 15.7 Debugging Macros

### cargo expand

```bash
# Install cargo-expand
cargo install cargo-expand

# View macro expansion
cargo expand
```

### trace_macros

```rust
#![feature(trace_macros)]

macro_rules! add {
    ($a:expr, $b:expr) => {
        $a + $b
    };
}

fn main() {
    trace_macros!(true);
    let x = add!(5, 3);
    trace_macros!(false);
}
```

---

## 15.8 Best Practices

### When to Use Macros

```rust
// ✅ Good uses:
// - Reducing boilerplate
// - Domain-specific languages
// - Compile-time computation
// - Variadic functionality

// ❌ Avoid when:
// - A function would work
// - Makes code hard to understand
// - Debugging becomes difficult
```

### Hygiene

```rust
// Macros are hygienic - variables don't leak
macro_rules! create_x {
    () => {
        let x = 5;
    };
}

fn main() {
    create_x!();
    // println!("{}", x);  // ERROR: x doesn't exist here
}
```

### Documentation

```rust
/// Creates a vector with the given elements
/// 
/// # Examples
/// 
/// ```
/// let v = my_vec![1, 2, 3];
/// assert_eq!(v, vec![1, 2, 3]);
/// ```
#[macro_export]
macro_rules! my_vec {
    ($($x:expr),*) => {
        vec![$($x),*]
    };
}
```

---

## 15.9 Common Macro Patterns

### Builder Pattern

```rust
macro_rules! builder {
    (
        $name:ident {
            $($field:ident: $ty:ty),* $(,)?
        }
    ) => {
        struct $name {
            $($field: $ty),*
        }
        
        impl $name {
            fn new() -> Self {
                $name {
                    $($field: Default::default()),*
                }
            }
            
            $(
                fn $field(mut self, $field: $ty) -> Self {
                    self.$field = $field;
                    self
                }
            )*
        }
    };
}

builder! {
    User {
        name: String,
        age: u32,
        email: String,
    }
}
```

### Match-like Macros

```rust
macro_rules! match_type {
    ($value:expr, i32 => $i32_rule:expr) => {
        $i32_rule
    };
    ($value:expr, String => $string_rule:expr) => {
        $string_rule
    };
    ($value:expr, _ => $default:expr) => {
        $default
    };
}
```

---

## Chapter 15 Exercises

### Exercise 15.1: Basic Macros
```rust
// Create macros for:
// - Swapping two variables
// - Creating a HashSet from elements
// - Timing code execution
```

### Exercise 15.2: Repetition
```rust
// Create variadic macros:
// - println! variant
// - SQL query builder
// - HTML element generator
```

### Exercise 15.3: Recursive Macros
```rust
// Implement:
// - Fibonacci at compile time
// - Nested tuple flattening
// - Expression evaluator
```

### Exercise 15.4: Procedural Macro
```rust
// Create a proc-macro crate:
// - Derive macro for ToString
// - Attribute macro for timing
// - Function-like macro for constants
```

### Exercise 15.5: DSL
```rust
// Create a domain-specific language:
// - Simple query language
// - Configuration DSL
// - State machine DSL
```

---

## Summary

In this chapter, you learned:

✅ Macros vs functions
✅ macro_rules! syntax
✅ Fragment specifiers
✅ Repetition patterns (*, +, ?)
✅ Recursive macros
✅ Procedural macros overview
✅ Function-like, derive, and attribute macros
✅ Debugging macros with cargo expand
✅ Best practices and hygiene
✅ Common macro patterns

---

## What's Next?

Macros give you metaprogramming power, but how do you ensure your code works correctly? In Chapter 16, we'll explore **Testing and Documentation**.

**Continue to [Chapter 16: Testing & Documentation](./16_testing_documentation.md)**
