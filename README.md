# Integrate *C++* into *Rust*

This project investigates solutions to integrate *C++* code into *Rust*.

## Project structure

It includes Rust crates demonstrating various interop approaches and their challenges, as well as a markdown summarizing our findings:

```
rust-cpp-interop/
├── crates/
│   ├── bindgen_build/      // Small crate illustrating the integration of C code into Rust using bindgen
│   ├── cc_build/           // Small crate illustrating the integration of C code into Rust
│   ├── cmake_build/        // Small crate illustrating the integration of C++ code into Rust using cmake
│   ├── cxx_closure/        // Investigates Rust closures passed accross the FFI boundary using cxx
│   └── scheduler/          // C++ example project integrated into Rust
│                           //   The goal is to gain some practical experience with C++ Rust interop topic
└── docs/                   // Blog post series about the integration of C++ into Rust
    └── part1_introduction.md 
```
