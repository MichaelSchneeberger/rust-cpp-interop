# Integrating a small *C++* Project into *Rust*

This crate demonstrates how to integrate a simple *C++* implementation of a *ReactiveX* *Scheduler* into *Rust*.
The goal is to showcase how the `cxx` crate can be used in a realistic setup.

## Usage

Run the example with:

``` bash
cargo run --example basic
```

## Problem Description

The *ReactiveX* *Scheduler* is an execution context built around a simple event loop that runs scheduled tasks.
It is implemented as a *C++* class `Scheduler` exposing the following methods:
* `schedule` - Schedules new tasks, either before or during loop execution
* `start_loop` - Starts executing the event loop
* `stop` - Stops the running loop.

A task is simply defined by a closure (function object) that takes no arguments.

The central question is:
* How can a `Scheduler` object be instantiated and its methods invoked from *Rust*?
* How can a task that contains captured values be defined and scheduled on the *Scheduler* using *Rust*?


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

## Proposed Solution

The proposed solution introduces a shim *schedulershim.h* that exposes an FFI-compatible interface for the *Scheduler*.
In particular, it defines a `schedule` method that accepts a trait object `DynFuncOnce` - a wrapper of a `FnOnce` closure.
It then calls the `schedule` function of the Scheduler, providing a C++ closure as an argument that executes a dedicated *Rust* callback function `execute_dyn_func_once`.
This allows to schedule a *Rust* closure - containing captured values - on the C++ *Scheduler*.

## References

* [ReactiveX Scheduler](https://reactivex.io/documentation/scheduler.html)

