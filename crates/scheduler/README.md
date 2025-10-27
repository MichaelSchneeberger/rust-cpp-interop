# Integrate a small *C++* project into *Rust*

**The integration into *Rust* is not yet implemented**

For this crate, a *C++* `Scheduler` implementation is integrated into *Rust*.
The goal of this project is to use `cxx` in a realistic setup.


## Project structure

```
scheduler/
├── include/
│   ├── scheduler.h             // a simple C++ ReactiveX Scheduler implementation
├── src/
│   └── main.cpp                // C++ main to test the C++ implementation (standalone)
├── CMakeLists.txt              // CMake to test the C++ implementation (standalone)
└── README.md
```
