fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/srcds-rs/srcds_rs.cpp")
        .include("include")
        .std("c++17")
        .compile("cxxbridge-demo");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/srcds_rs.cpp");
    println!("cargo:rerun-if-changed=include/srcds_rs.h");
}
