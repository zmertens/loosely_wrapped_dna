fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/dawg.cpp")
        .flag_if_supported("-std=c++17")
        .compile("loosely_wrapped_dna");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/dawg.cpp");
    println!("cargo:rerun-if-changed=include/dawg_app.h");
    println!("cargo:rerun-if-changed=include/dawg.h");
    println!("cargo:rerun-if-changed=include/version.h");
    println!("cargo:rerun-if-changed=include/dawgarg.xmh");
    println!("cargo:rerun-if-changed=include/xm.h");
}