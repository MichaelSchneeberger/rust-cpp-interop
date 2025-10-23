#include "cxx-closure/include/closure.h"
#include "cxx-closure/src/main.rs.h"


void c_take_callback(rust::Fn<size_t(rust::String)> callback) {
  callback("2020");
}

