use cmake;

fn main() {
    println!("cargo:rerun-if-changed=native/cats.hpp");
    println!("cargo:rerun-if-changed=native/cats.cpp");

    let dst = cmake::Config::new("native")
        .cxxflag("-std=c++20")
        .build();

    println!("cargo:rustc-link-search=native={}", dst.display());

    // 'cats' refers to 'libcat.a'
    println!("cargo:rustc-link-lib=static=cats");
    println!("cargo:rustc-link-lib=dylib=stdc++");
}
