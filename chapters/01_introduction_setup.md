# Chapter 1: Introduction & Setup

## 1.1 What is Rust?

Rust is a systems programming language that combines the performance and low-level control of C++ with modern language features and memory safety guarantees. Created by Graydon Hoare in 2006 and now primarily sponsored by Mozilla, Rust has gained massive adoption for its unique approach to memory management.

### Key Features of Rust

#### 🛡️ Memory Safety Without Garbage Collection

Unlike languages like Java or Python that use garbage collection (GC), Rust prevents memory errors at **compile time** through its ownership system:

```rust
// This code WON'T compile - Rust catches the error before runtime
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // s1 is MOVED to s2
    println!("{}", s1);  // ERROR: s1 is no longer valid!
}
```

#### ⚡ Zero-Cost Abstractions

Rust's high-level features compile down to the same machine code as low-level C:

```rust
// This iterator chain compiles to the same code as a manual for loop
let sum: i32 = (1..100)
    .filter(|x| x % 2 == 0)
    .map(|x| x * 2)
    .sum();
```

#### 🧵 Fearless Concurrency

Rust's type system prevents data races at compile time:

```rust
use std::thread;

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    
    // Rust ensures thread-safe access
    let handle = thread::spawn(move || {
        println!("Data in thread: {:?}", data);
    });
    
    handle.join().unwrap();
}
```

#### 📦 Excellent Tooling

- **Cargo**: Built-in package manager and build system
- **rustfmt**: Automatic code formatting
- **clippy**: Advanced linting for best practices
- **rustdoc**: Documentation generation
- **Excellent error messages**: Some of the best in the industry

### When to Use Rust

| Use Case | Why Rust? |
|----------|-----------|
| Systems Programming | Direct hardware access, no runtime overhead |
| Web Assembly | Compile to WASM for browser performance |
| CLI Tools | Fast, single-binary distribution |
| Embedded Systems | No GC, predictable performance |
| Network Services | High concurrency, memory safety |
| Blockchain | Performance-critical smart contracts |

### Companies Using Rust

- **Mozilla**: Servo browser engine
- **Microsoft**: Azure IoT, Windows components
- **Google**: Android, Fuchsia OS
- **Amazon**: Firecracker, Bottlerocket
- **Discord**: Backend services
- **Cloudflare**: Core infrastructure
- **Facebook**: Source control backend

---

## 1.2 Installing Rust

### Method 1: rustup (Recommended)

`rustup` is the official Rust toolchain installer and manages your Rust installation.

#### Linux/macOS

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### Windows

1. Download and run [`rustup-init.exe`](https://rustup.rs)
2. Follow the installation wizard
3. Install Visual Studio C++ Build Tools when prompted

### Post-Installation Setup

After installation, add Rust to your PATH:

```bash
# For bash/zsh, add to ~/.bashrc or ~/.zshrc
source $HOME/.cargo/env
```

### Verify Installation

```bash
# Check Rust compiler version
rustc --version
# Output: rustc 1.75.0 (82e1608df 2023-12-21)

# Check Cargo version
cargo --version
# Output: cargo 1.75.0 (1d209b9d1 2023-12-21)

# Check rustup version
rustup --version
# Output: rustup 1.26.0 (5af9b9484 2023-04-05)
```

---

## 1.3 Understanding the Toolchain

### rustup

The Rust toolchain installer and manager.

```bash
# Install a specific version
rustup install 1.70.0

# Set default version
rustup default 1.75.0

# Add components
rustup component add rustfmt clippy rust-docs

# Add targets for cross-compilation
rustup target add wasm32-unknown-unknown
rustup target add x86_64-unknown-linux-musl

# List installed toolchains
rustup toolchain list

# Update Rust
rustup update
```

### rustc

The Rust compiler.

```bash
# Compile a file
rustc main.rs

# Compile with optimizations
rustc -O main.rs

# Compile to specific output
rustc -o myprogram main.rs

# Show assembly output
rustc --emit=asm main.rs
```

### cargo

The Rust package manager and build system.

```bash
# Create new project
cargo new my_project
cargo new --lib my_library

# Build project
cargo build
cargo build --release  # Optimized build

# Run project
cargo run
cargo run --release

# Run tests
cargo test

# Check for errors without building
cargo check

# Add dependency
cargo add serde

# Format code
cargo fmt

# Lint code
cargo clippy

# Generate documentation
cargo doc --open
```

---

## 1.4 Your First Rust Program

### Creating a Project

```bash
cargo new hello_rust
cd hello_rust
```

### Project Structure

```
hello_rust/
├── Cargo.toml      # Project manifest (dependencies, metadata)
├── src/
│   └── main.rs     # Entry point
└── target/         # Build artifacts (generated)
```

### Understanding Cargo.toml

```toml
[package]
name = "hello_rust"
version = "0.1.0"
edition = "2021"  # Rust edition (2015, 2018, 2021)

[dependencies]
# Add dependencies here
serde = "1.0"
```

### Writing Hello World

Edit `src/main.rs`:

```rust
// This is a comment
// The main function is the entry point of every Rust program
fn main() {
    // println! is a macro (note the !)
    // It prints text followed by a newline
    println!("Hello, Rust!");
    
    // Use {} for formatting
    let name = "Developer";
    println!("Welcome, {}!", name);
    
    // Use {:?} for debug formatting
    let numbers = vec![1, 2, 3, 4, 5];
    println!("Numbers: {:?}", numbers);
}
```

### Running Your Program

```bash
cargo run
```

Output:
```
   Compiling hello_rust v0.1.0 (/path/to/hello_rust)
    Finished dev [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/hello_rust`
Hello, Rust!
Welcome, Developer!
Numbers: [1, 2, 3, 4, 5]
```

---

## 1.5 Understanding Compilation

### Debug vs Release Builds

```bash
# Debug build (fast compilation, includes debug info)
cargo build
# Output: target/debug/hello_rust

# Release build (slow compilation, optimized)
cargo build --release
# Output: target/release/hello_rust
```

| Feature | Debug | Release |
|---------|-------|---------|
| Optimization | None | Full (-O3) |
| Compile Time | Fast | Slow |
| Binary Size | Large | Small |
| Runtime Speed | Slower | Fastest |
| Debug Info | Yes | No |

### Understanding Error Messages

Rust has the best error messages in the industry:

```rust
fn main() {
    let x: i32 = "hello";  // Type mismatch
}
```

Error output:
```
error[E0308]: mismatched types
 --> src/main.rs:2:20
  |
2 |     let x: i32 = "hello";
  |            ---   ^^^^^^^ expected `i32`, found `&str`
  |            |
  |            expected due to this
  |
  = note: expected type `i32`
             found type `&'static str`

error: aborting due to previous error
```

The error tells you:
1. **Error code** (E0308) - searchable in documentation
2. **Location** - file, line, and column
3. **What's wrong** - type mismatch explanation
4. **Visual indicators** - arrows pointing to the problem
5. **Additional notes** - type details

---

## 1.6 IDE Setup

### Visual Studio Code (Recommended)

1. Install the **rust-analyzer** extension
2. Install optional extensions:
   - **Crates** - Shows available dependency updates
   - **Better TOML** - For Cargo.toml editing
   - **Error Lens** - Inline error display

### IntelliJ IDEA / RustRover

1. Install the **Rust** plugin (IDEA) or use **RustRover** (dedicated Rust IDE)
2. Install **rust-analyzer** (bundled with RustRover)

### Other Editors

- **Neovim/Vim**: rust.vim + coc.nvim with rust-analyzer
- **Emacs**: rust-mode + lsp-mode
- **Sublime Text**: RustEnhanced

---

## 1.7 Essential Cargo Commands

### Development Workflow

```bash
# Check for errors (faster than build)
cargo check

# Format all code
cargo fmt

# Run linter
cargo clippy

# Run tests
cargo test

# Run specific test
cargo test test_function_name

# Run benchmarks (requires nightly)
cargo +nightly bench
```

### Dependency Management

```bash
# Add dependency
cargo add serde
cargo add serde --dev  # Dev dependency
cargo add tokio --features full  # With features

# Remove dependency
cargo rm serde

# Update dependencies
cargo update

# Show dependency tree
cargo tree

# Show outdated dependencies
cargo outdated  # Requires cargo-outdated
```

### Project Management

```bash
# Create new binary project
cargo new myapp

# Create new library
cargo new mylib --lib

# Create in existing directory
cargo init

# Build documentation
cargo doc --open

# Clean build artifacts
cargo clean

# Verify package
cargo package
```

---

## 1.8 Rust Editions

Rust evolves through editions that introduce new features while maintaining backward compatibility.

| Edition | Release | Key Features |
|---------|---------|--------------|
| 2015 | 2015 | Original stable release |
| 2018 | 2018 | Modules, `?` operator, `dyn Trait` |
| 2021 | 2021 | Better pattern matching, IntoIterator |
| 2024 | 2024 | Upcoming |

Specify edition in `Cargo.toml`:
```toml
[package]
edition = "2021"
```

---

## 1.9 The Rust Community

### Resources

- **Official Documentation**: https://doc.rust-lang.org/
- **The Rust Book**: https://doc.rust-lang.org/book/
- **Rust by Example**: https://doc.rust-lang.org/rust-by-example/
- **Rust Playground**: https://play.rust-lang.org/
- **This Week in Rust**: https://this-week-in-rust.org/
- **Rust Users Forum**: https://users.rust-lang.org/
- **r/rust**: https://reddit.com/r/rust

### Getting Help

- **Discord**: https://discord.gg/rust-lang
- **Zulip**: https://rust-lang.zulipchat.com/
- **Stack Overflow**: Tag your questions with `rust`

---

## Chapter 1 Exercises

### Exercise 1.1: Installation Verification
1. Install Rust using rustup
2. Verify installation with `rustc --version` and `cargo --version`
3. Install rustfmt and clippy components

### Exercise 1.2: First Project
1. Create a new Cargo project called `greeting`
2. Modify `main.rs` to print your name and age
3. Run it with `cargo run`
4. Build a release version and compare binary sizes

### Exercise 1.3: Error Exploration
1. Intentionally create type errors in your code
2. Read and understand each error message
3. Fix the errors based on compiler suggestions

### Exercise 1.4: Tooling Practice
1. Run `cargo fmt` on your code
2. Run `cargo clippy` and fix any warnings
3. Generate documentation with `cargo doc --open`

---

## Summary

In this chapter, you learned:

✅ What Rust is and why it's unique (memory safety without GC)
✅ How to install Rust using rustup
✅ The Rust toolchain (rustup, rustc, cargo)
✅ How to create and run your first Rust project
✅ Understanding compilation (debug vs release)
✅ How to read Rust's excellent error messages
✅ IDE setup recommendations
✅ Essential Cargo commands
✅ Rust editions and the community

---

## What's Next?

Now that you have Rust installed and understand the basics, let's dive into the language itself! In Chapter 2, we'll explore Rust's syntax, variables, and basic constructs.

**Continue to [Chapter 2: Basic Syntax & Variables](./02_basic_syntax_variables.md)**
