# *Rust* Closures with `cxx`

This crate explores how to pass *Rust* closure accross the `cxx` FFI boundary into *C++* code.

## Problem

The concrete problem is as follows:
Given a *Rust* closure, how can it be passed as an argument to a *C++* function?

```rust
// define Rust closure
let closure = |ctx| {
  println!("{}", ctx.msg)
};

// call C++ function
ffi::c_take_callback(closure);
```

In the example, the closure does not capture any external variables from its environment.
In *C++* terminology, `closure` has no captured values.

A natural follow-up question is therefore: How can we pass a closure that does capture values to *C++* through the FFI boundary?

## Project structure

The project is structured as follows:

```
cxx_closure/
├── include/
│   ├── closure.h           // function exposed to Rust
├── src/
│   ├── closure.cpp
│   └── main.rs             // FFI definition and C++ function call
├── Cargo.toml
├── README.md
└── build.rs                // cxx build bridge
```

## Proposed solution

The solution that is suggested by *Michael Bryan* (see References below), is to:

* summarize the captured values as an additional argument of the *Rust* closure, and
* add a *C++* shim including an additional argument `CallbackContext` to represent the captured values:

    ```cpp
    void c_take_callback(rust::Fn<void(rust::Box<CallbackContext>)> callback, rust::Box<CallbackContext> ctx) {
      callback(std::move(ctx));
    }

    ```

The disadvantage of this approach is that a shim function must be defined for each captured value type.


## References

* https://adventures.michaelfbryan.com/posts/rust-closures-in-ffi/
* https://github.com/Michael-F-Bryan/rust-closures-and-ffi/
* https://github.com/dtolnay/cxx/issues/114
* https://github.com/dtolnay/cxx/pull/85
* https://github.com/dtolnay/cxx/blob/6f132eee85461743fa048f1b79afc020d589f015/tests/ffi/tests.cc

