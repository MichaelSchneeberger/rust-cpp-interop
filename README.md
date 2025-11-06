# Integrate *C++* into *Rust*

This project investigates solutions to integrate and migrate *C++* code into *Rust*.

## Project structure

It includes Rust crates demonstrating various interop approaches and their challenges, as well as a markdown summarizing our findings:

```
rust-cpp-interop/
├── crates/
│   ├── bindgen_build/      // integration of C code into Rust using bindgen
│   ├── cc_build/           // integration of C code into Rust
│   ├── cmake_build/        // integration of C++ code into Rust using cmake
│   ├── cxx_closure/        // Investigates Rust closures passed accross the FFI boundary using cxx
│   └── scheduler/          // C++ implemented scheduler integrated into Rust
└── docs/
    └── part1_introduction.md 
```
