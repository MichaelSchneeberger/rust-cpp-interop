fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/closure.cpp")
        .std("c++14")
        .compile("cxxclosure");

    println!("cargo:rerun-if-changed=src/closure.cpp");
    println!("cargo:rerun-if-changed=include/closure.h");
}
