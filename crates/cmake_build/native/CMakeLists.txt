cmake_minimum_required(VERSION 3.10)
project(libcats)

add_library(cats STATIC cats.cpp)
install(TARGETS cats DESTINATION .)
