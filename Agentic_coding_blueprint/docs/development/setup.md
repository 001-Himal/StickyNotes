# Local Development Setup

## Prerequisites
1. **Rust Toolchain:** Install Rust 1.80+ via [rustup.rs](https://rustup.rs):
   ```bash
   rustup default stable
   ```
2. **C++ Build Tools (for Slint backend on Windows):**
   - Visual Studio 2022 C++ Build Tools or MSVC compiler.

## Repository Setup
```bash
# Clone or navigate to the project directory
cd StickyNotes

# Build the workspace
cargo build

# Run the main Sticky Note desktop manager
cargo run --bin sticky-note

# In a separate terminal, test the settings executable
cargo run --bin sticky-note-settings
```

## Running Tests & Checks
```bash
# Run unit & integration tests
cargo test

# Run strict clippy linting
cargo clippy -- -D warnings

# Format check
cargo fmt --check
```
