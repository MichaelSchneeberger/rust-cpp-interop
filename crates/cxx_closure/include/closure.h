#pragma once

#include <memory>
#include <rust/cxx.h>
// #include "cxx-closure/src/main.rs.h"

struct CallbackContext;

void c_take_callback(rust::Fn<void(rust::Box<CallbackContext>)> callback, rust::Box<CallbackContext> ctx);
