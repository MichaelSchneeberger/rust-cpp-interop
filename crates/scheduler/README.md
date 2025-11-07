# Integrate a small *C++* project into *Rust*

For this crate, a *C++* `Scheduler` implementation is integrated into *Rust*.
The goal of this project is to use `cxx` in a realistic setup.

## Usage

Run the example using:

``` bash
cargo run --example basic
```

## Project structure

```
scheduler/
├── examples/                   // main() function in Rust
├── include/
│   ├── scheduler.h             // A simple C++ ReactiveX Scheduler implementation
│   └── schedulershim.h         // A shim implementing cxx compatible interface
├── src/
│   ├── lib.rs                  // FFI definition using cxx
│   └── main.cpp                // C++ main to test the C++ scheduler implementation
├── Cargo.toml
├── CMakeLists.txt              // CMake to test the C++ scheduler implementation
├── build.rs                    // defines the cxx build process
└── README.md
```
