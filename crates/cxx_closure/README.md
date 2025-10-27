# Rust Closures with cxx

This crate explores how to pass Rust closure accross the `cxx` FFI boundary into C++ code.

The concrete problem is as follows:
Given a Rust closure, how can it be passed as an argument to a C++ function?

```rust
// define Rust closure
let closure = |ctx| {
  println!("{}", ctx.msg)
};

// call C++ function
ffi::c_take_callback(closure);
```

In the example, the closure does not capture any external variables from its environment.
In C++ terminology, `closure` has no captured values.

A natural follow-up question is therefore: How can we pass a closure that does capture values to C++ through the FFI boundary?


## References

https://adventures.michaelfbryan.com/posts/rust-closures-in-ffi/
https://github.com/Michael-F-Bryan/rust-closures-and-ffi/

