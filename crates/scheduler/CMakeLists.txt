cmake_minimum_required(VERSION 3.10)

# Name of the project (and optionally specify language)
project(HelloWorld LANGUAGES CXX)

# Add executable target from source file(s)
add_executable(hello
  src/main.cpp
)

target_include_directories(hello PRIVATE
    ${CMAKE_SOURCE_DIR}/include
)

# Optionally set C++ standard
target_compile_features(hello PRIVATE cxx_std_23)
