# Installation

This guide covers all the ways to install Pixel Art Rust on your system.

## Prerequisites

Before installing Pixel Art Rust, make sure you have:

- **Rust 1.70 or higher** - Required for SIMD support and latest features
- **Cargo** - Comes bundled with Rust
- **Git** - For building from source (optional)

## Installing Rust

If you don't have Rust installed, get it from [rustup.rs](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Installation Methods

### From Crates.io (Recommended)

The easiest way to install is from the official Rust package registry:

```bash
cargo install pixel-art-rust
```

This will download, compile, and install the latest stable version.

### From GitHub Releases

Download pre-compiled binaries from the [releases page](https://github.com/naporin0624/pixel-art-rust/releases):

1. Go to the latest release
2. Download the binary for your platform
3. Extract and place in your PATH

### From Source

For the latest development version or to contribute:

```bash
# Clone the repository
git clone https://github.com/naporin0624/pixel-art-rust.git
cd pixel-art-rust

# Build in release mode
cargo build --release

# The binary will be in target/release/pixel-art-rust
```

## Verification

Verify your installation by running:

```bash
pixel-art-rust --version
```

You should see output similar to:

```
pixel-art-rust 0.1.0
```

## Updating

### From Crates.io

```bash
cargo install pixel-art-rust --force
```

### From Source

```bash
git pull
cargo build --release
```

## Troubleshooting

### Common Issues

**"command not found"**: Make sure `~/.cargo/bin` is in your PATH:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

**Compilation errors**: Ensure you have Rust 1.70+:

```bash
rustc --version
rustup update
```

**Permission errors**: On some systems, you might need to use `sudo` or adjust permissions.

## Next Steps

Now that you have Pixel Art Rust installed, check out the [Getting Started](/guide/getting-started) guide to create your first pixel art!
