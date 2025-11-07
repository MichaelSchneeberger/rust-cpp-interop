#pragma once

#include <memory>
#include <rust/cxx.h>

struct DynFun;
struct CallbackContext;

void c_take_callback(
  rust::Fn<void()> callback
);

void c_take_callback_and_ctx(
  rust::Fn<void(rust::Box<CallbackContext>)> callback,
  rust::Box<CallbackContext> ctx
);

void c_take_task(
  const DynFun &f
);
