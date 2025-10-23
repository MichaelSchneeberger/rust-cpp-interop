#include <print>

#include "cxx-closure/src/main.rs.h"
#include "cxx-closure/include/closure.h"


void c_take_callback(rust::Fn<void(rust::Box<CallbackContext>)> callback, rust::Box<CallbackContext> ctx) {
  // std::println("hello");
  callback(std::move(ctx));
}

