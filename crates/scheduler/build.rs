fn main() {
    cxx_build::bridge("src/lib.rs")
        .std("c++14")
        .compile("cxxbridge-demo");

    println!("cargo:rerun-if-changed=include/scheduler.h");
    println!("cargo:rerun-if-changed=include/schedulershim.h");
}
