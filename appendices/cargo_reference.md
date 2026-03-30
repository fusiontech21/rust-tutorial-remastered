# Appendix A: Cargo Commands Reference

## Project Management

```bash
# Create new project
cargo new <name>              # Create new binary project
cargo new <name> --lib        # Create new library project
cargo init                    # Initialize in existing directory
cargo init --lib              # Initialize as library

# Build
cargo build                   # Build in debug mode
cargo build --release         # Build in release mode (optimized)
cargo build --all-features    # Build with all features enabled
cargo build --no-default-features

# Run
cargo run                     # Run the binary
cargo run --release           # Run release build
cargo run -- <args>           # Pass arguments to program

# Check
cargo check                   # Check for errors (faster than build)
cargo check --all-targets     # Check all targets
```

## Testing

```bash
cargo test                    # Run all tests
cargo test <name>             # Run tests matching name
cargo test --lib              # Run library tests
cargo test --doc              # Run documentation tests
cargo test -- --nocapture     # Show test output
cargo test -- --ignored       # Run ignored tests
cargo test --release          # Run tests in release mode
```

## Formatting and Linting

```bash
cargo fmt                     # Format code
cargo fmt -- --check          # Check formatting
cargo clippy                  # Run linter
cargo clippy -- -D warnings   # Treat warnings as errors
cargo clippy --fix            # Auto-fix issues
```

## Dependencies

```bash
cargo add <crate>             # Add dependency
cargo add <crate> --dev       # Add dev dependency
cargo add <crate> --features <feat>  # Add with features
cargo rm <crate>              # Remove dependency
cargo update                  # Update dependencies
cargo update -p <crate>       # Update specific crate
cargo tree                    # Show dependency tree
cargo tree -i <crate>         # Show reverse dependencies
cargo tree -d                 # Show duplicate dependencies
```

## Documentation

```bash
cargo doc                     # Generate documentation
cargo doc --open              # Generate and open in browser
cargo doc --no-deps           # Exclude dependencies
cargo doc --private           # Include private items
```

## Publishing

```bash
cargo package                 # Package for publishing
cargo publish                 # Publish to crates.io
cargo publish --dry-run       # Test publish locally
cargo owner --add <user>      # Add owner
cargo owner --remove <user>   # Remove owner
```

## Cleaning

```bash
cargo clean                   # Remove target directory
cargo clean -p <package>      # Clean specific package
```

## Information

```bash
cargo --version               # Show cargo version
cargo --verbose               # Verbose output
cargo metadata                # Output machine-readable info
cargo locate-project          # Show path to Cargo.toml
cargo search <query>          # Search crates.io
cargo install <crate>         # Install binary crate
cargo uninstall <crate>       # Uninstall binary crate
```

## Workspaces

```bash
cargo build --workspace       # Build all workspace members
cargo build -p <package>      # Build specific package
cargo test --workspace        # Test all members
```

## Features

```bash
cargo build --features <feat>     # Enable features
cargo build --all-features        # Enable all features
cargo build --no-default-features # Disable default features
```

## Targets

```bash
cargo build --bin <name>          # Build specific binary
cargo build --lib                 # Build library
cargo build --example <name>      # Build example
cargo build --test <name>         # Build test
cargo build --bench <name>        # Build benchmark
```

## Cross-Compilation

```bash
rustup target add <target>        # Add compilation target
cargo build --target <target>     # Build for target
cargo run --target <target>       # Run for target
```

## Environment Variables

```bash
CARGO_HOME                    # Cargo directory
CARGO_TARGET_DIR              # Target directory
CARGO_INCREMENTAL             # Incremental compilation (0/1)
RUSTFLAGS                     # Pass flags to rustc
RUST_LOG                      # Logging level
RUST_BACKTRACE                # Backtrace (0/1/full)
```

## Configuration

```toml
# .cargo/config.toml
[build]
target = "x86_64-unknown-linux-gnu"

[target.x86_64-unknown-linux-gnu]
linker = "gcc"

[env]
MY_VAR = "value"
```

## Common Workflows

```bash
# Development loop
cargo check
cargo fmt
cargo clippy
cargo test

# Before commit
cargo fmt -- --check
cargo clippy -- -D warnings
cargo test --all-targets

# Release preparation
cargo build --release
cargo test --release
cargo bench

# Debug build issue
cargo clean
cargo build -vv
```
