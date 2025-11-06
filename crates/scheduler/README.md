# Integrate a small *C++* project into *Rust*

**The integration into *Rust* is not yet implemented**

For this crate, a *C++* `Scheduler` implementation is integrated into *Rust*.
The goal of this project is to use `cxx` in a realistic setup.


## Project structure

```
scheduler/
├── include/
│   ├── scheduler.h             // A simple C++ ReactiveX Scheduler implementation
│   └── schedulershim.h         // A shim implementing cxx compatible interface
├── src/
│   ├── lib.rs                  // FFI binding using cxx
│   └── main.cpp                // C++ main to test the C++ scheduler implementation
├── examples/                   // main() function in Rust
├── CMakeLists.txt              // CMake to test the C++ scheduler implementation
├── Cargo.toml
├── build.rs
└── README.md
```
