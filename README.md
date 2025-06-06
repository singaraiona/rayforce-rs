# Rayforce Rust Bindings

This crate provides Rust bindings for the Rayforce database library. It allows you to use Rayforce's functionality from Rust code in a safe and idiomatic way.

## System Requirements

Before using this crate, you need to have the Rayforce library installed on your system. The following components are required:

- Rayforce C library
- pkg-config
- A C compiler (gcc or clang)

## Installation

### 1. Install Rayforce Library

First, you need to install the Rayforce library on your system. The library should be installed in a standard location (e.g., `/usr/local/lib` and `/usr/local/include`).

### 2. Install pkg-config File

Copy the `rayforce.pc` file to your system's pkg-config directory:

```bash
# For system-wide installation (requires sudo)
sudo cp rayforce.pc /usr/lib/pkgconfig/
# OR for user-specific installation
mkdir -p ~/.local/lib/pkgconfig
cp rayforce.pc ~/.local/lib/pkgconfig/
```

### 3. Verify Installation

You can verify the installation by running:

```bash
pkg-config --libs --cflags rayforce
```

This should output the necessary compiler and linker flags.

## Features

- Safe Rust wrappers around the Rayforce C API
- Memory safety through Rust's ownership system
- Error handling through Rust's Result type
- Comprehensive type system mapping
- Thread-safe operations

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
rayforce-rs = "0.1.0"
```

## Example

The crate includes a comprehensive example that demonstrates:
- Runtime initialization and management
- Version checking
- Command line argument handling
- File descriptor mapping
- External runtime handling
- Runtime execution
- Proper cleanup

Run it with:
```bash
cargo run --example basic
```

The example shows how to:
1. Initialize the Rayforce runtime
2. Check the version
3. Handle command line arguments
4. Work with file descriptor mappings
5. Access external runtime
6. Execute the runtime
7. Clean up resources

## Safety

The bindings are designed to be safe to use from Rust code. All unsafe operations are wrapped in safe interfaces that maintain Rust's safety guarantees. However, you should still be careful when:

1. Managing object lifetimes
2. Handling raw pointers
3. Using unsafe functions directly

## Building

To build the crate, you need to have the Rayforce C library installed on your system. The build script will automatically generate the Rust bindings from the C header file.

```bash
cargo build
```

## Testing

Run the tests with:

```bash
cargo test
```

## Struct Alignment

The `option_t` struct is defined with `aligned(16)` to ensure it is returned via registers (a pair of registers) on modern architectures. This is important for performance and compatibility.

```c
typedef struct __attribute__((aligned(16))) {
    option_code_t code;  // 8 bytes
    raw_p value;         // 8 bytes
} option_t;
```

## License

This project is licensed under the MIT License - see the LICENSE file for details. 