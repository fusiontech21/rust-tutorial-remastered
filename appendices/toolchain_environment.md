# Appendix C: Rust Toolchain & Environment

## Rustup

### Installation

```bash
# Linux/macOS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows
# Download rustup-init.exe from https://rustup.rs
```

### Toolchain Management

```bash
# Show installed toolchains
rustup show
rustup toolchain list

# Install toolchain
rustup install 1.75.0
rustup install nightly
rustup install stable-x86_64-pc-windows-msvc

# Set default
rustup default stable
rustup default 1.75.0
rustup default nightly

# Update
rustup update
rustup update stable
rustup update nightly

# Remove
rustup toolchain uninstall 1.75.0
```

### Components

```bash
# List available components
rustup component list

# Add components
rustup component add rustfmt
rustup component add clippy
rustup component add rust-docs
rustup component add rust-src
rustup component add rust-analyzer
rustup component add llvm-tools-preview

# Remove components
rustup component remove rustfmt
```

### Targets

```bash
# List available targets
rustup target list

# Add target
rustup target add wasm32-unknown-unknown
rustup target add x86_64-unknown-linux-musl
rustup target add aarch64-apple-darwin

# Remove target
rustup target remove wasm32-unknown-unknown
```

### Overrides

```bash
# Set directory override
rustup override set stable
rustup override set 1.75.0

# Remove override
rustup override unset

# List overrides
rustup override list
```

## Cargo Environment

### Configuration Files

```toml
# ~/.cargo/config.toml (global)
# .cargo/config.toml (project-local)

[build]
target = "x86_64-unknown-linux-gnu"
jobs = 4

[target.x86_64-unknown-linux-gnu]
linker = "gcc"
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

[env]
CARGO_NET_GIT_FETCH_WITH_CLI = "true"
RUST_TEST_THREADS = "1"

[alias]
b = "build"
c = "check"
t = "test"
r = "run"
rr = "run --release"
l = "clippy"
lf = "clippy --fix"
```

### Environment Variables

```bash
# Paths
export CARGO_HOME="$HOME/.cargo"
export RUSTUP_HOME="$HOME/.rustup"

# Build
export CARGO_TARGET_DIR="./target"
export CARGO_INCREMENTAL=1

# Flags
export RUSTFLAGS="-C target-cpu=native"
export RUSTDOCFLAGS="--cfg docsrs"

# Network
export CARGO_NET_RETRY=5
export CARGO_NET_TIMEOUT=30

# Logging
export RUST_LOG=debug
export RUST_BACKTRACE=1
```

## Rust Versions

### Release Cycle

```
Stable: Every 6 weeks
Beta: Tracks next stable
Nightly: Daily builds with unstable features
```

### Checking Version

```bash
rustc --version
cargo --version
rustup --version
```

### Using Specific Versions

```bash
# Run with specific version
rustup run 1.75.0 cargo build
rustup run nightly cargo +nightly build

# Set for directory
rustup override set 1.75.0
```

## IDE Setup

### VS Code

```json
// settings.json
{
    "rust-analyzer.checkOnSave.command": "clippy",
    "rust-analyzer.inlayHints.parameterHints.enable": true,
    "rust-analyzer.inlayHints.typeHints.enable": true,
    "[rust]": {
        "editor.formatOnSave": true,
        "editor.defaultFormatter": "rust-lang.rust-analyzer"
    }
}
```

### Extensions

- rust-analyzer (required)
- Crates (dependency updates)
- Better TOML (Cargo.toml editing)
- Error Lens (inline errors)

### IntelliJ IDEA / RustRover

- Install Rust plugin (IDEA)
- Or use RustRover (dedicated IDE)
- Configure rust-analyzer

## Common Setups

### Development Setup (Linux)

```bash
# Install rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install components
rustup component add rustfmt clippy rust-docs rust-src

# Install useful tools
cargo install cargo-edit
cargo install cargo-watch
cargo install cargo-audit
cargo install cargo-outdated
cargo install cargo-flamegraph

# Add to shell
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### Cross-Compilation Setup

```bash
# Install cross-compilation targets
rustup target add x86_64-unknown-linux-musl
rustup target add armv7-unknown-linux-gnueabihf
rustup target add wasm32-unknown-unknown

# Install cross (Docker-based cross-compilation)
cargo install cross

# Use cross
cross build --target x86_64-unknown-linux-musl --release
```

### WebAssembly Setup

```bash
# Add WASM target
rustup target add wasm32-unknown-unknown

# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Build WASM
wasm-pack build --release
```

## Troubleshooting

### Common Issues

```bash
# Permission issues
sudo chown -R $(whoami) ~/.cargo
sudo chown -R $(whoami) ~/.rustup

# Clear cache
cargo clean
rm -rf ~/.cargo/registry

# Reinstall toolchain
rustup toolchain uninstall stable
rustup toolchain install stable

# Update rustup
rustup self update

# Check installation
rustup show
cargo --version
```

### Build Issues

```bash
# Verbose output
cargo build -vv

# Clean build
cargo clean && cargo build

# Check dependencies
cargo tree

# Update dependencies
cargo update
```

## Useful Tools

### Development

```bash
cargo install cargo-watch      # Auto-rebuild on changes
cargo install cargo-edit       # Add/rm dependencies
cargo install cargo-audit      # Security audit
cargo install cargo-outdated   # Check outdated deps
cargo install cargo-expand     # Expand macros
cargo install cargo-flamegraph # Profiling
```

### Code Quality

```bash
cargo install cargo-deny       # Dependency linting
cargo install cargo-geiger     # Unsafe code detection
cargo install cargo-udeps      # Unused dependency detection
```

### Documentation

```bash
cargo install cargo-docset     # Generate docsets
cargo install cargo-readme     # Generate README from docs
```
