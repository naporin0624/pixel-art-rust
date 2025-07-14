# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview
This is a Rust project called "pixel-art-rust" that is in early development stages. The project uses Rust edition 2024 and is managed with mise for consistent development environments.

## Common Development Commands

### Build Commands
```bash
# Build the project
cargo build

# Build in release mode
cargo build --release

# Run the project
cargo run

# Run with release optimizations
cargo run --release
```

### Development Commands
```bash
# Check code without building
cargo check

# Format code
cargo fmt

# Run linter
cargo clippy

# Run linter with all targets
cargo clippy --all-targets --all-features

# Run tests (when implemented)
cargo test

# Run a specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture

# Watch for changes and run tests
cargo watch -x test

# Generate and open documentation
cargo doc --open
```

## Architecture Notes
Currently, this is a minimal Rust project with only a main.rs file. As the project develops, consider:
- Using modules in src/ for different pixel art operations
- Implementing traits for different pixel art algorithms
- Using external crates for image processing (e.g., `image`, `imageproc`)
- Creating a library crate if pixel art functionality should be reusable

## Development Environment
- Rust version: 1.88.0 (managed by mise)
- Cargo version: 0.89.0 (managed by mise)
- The project uses mise for version management - ensure `mise install` is run before development

## MCP Server
The project includes an MCP (Model Context Protocol) server configuration for AI-assisted search capabilities, which can be helpful for exploring the codebase as it grows.