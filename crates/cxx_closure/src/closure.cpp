#include "cxx-closure/src/lib.rs.h"
#include "cxx-closure/include/closure.h"

void c_take_callback(
  rust::Fn<void()> callback
) {
  callback();
}

void c_take_callback_and_ctx(
  rust::Fn<void(rust::Box<CallbackContext>)> callback,
  rust::Box<CallbackContext> ctx
) {
  callback(std::move(ctx));
}

void c_take_task(
  const DynFun &f
) {
  return execute_dyn_task(f);
}

