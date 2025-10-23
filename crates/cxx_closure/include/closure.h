#pragma once
#include <rust/cxx.h>
#include <memory>

void c_take_callback(rust::Fn<size_t(rust::String)> callback);
