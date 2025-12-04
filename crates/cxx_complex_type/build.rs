fn main() {
    cxx_build::bridge("src/lib.rs")
        .include("/usr/include/eigen3")
        .std("c++23")
        .compile("cxxclosure");

    println!("cargo:rerun-if-changed=include/matrixmult.hpp");
}

