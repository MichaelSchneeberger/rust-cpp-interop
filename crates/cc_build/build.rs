use cc::Build;
use std::path::Path;

fn main() {
    let project_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let native_src = project_root.join("native");

    Build::new()
        .flag_if_supported("-lcurl")
        .file(native_src.join("simple.c"))
        .compile("ccbuild");     // compiles to libccbuild.a
}
