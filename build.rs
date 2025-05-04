// build.rs
fn main() {
    // Tell cargo to rebuild if any of these files change
    println!("cargo:rerun-if-changed=include/plugin.h");
    println!("cargo:rerun-if-changed=include/plugin_bridge.h");
    println!("cargo:rerun-if-changed=src/plugin.cpp");
    println!("cargo:rerun-if-changed=src/plugin_bridge.cpp");
    println!("cargo:rerun-if-changed=src/lib.rs");

    // Configure cxx build
    let mut builder = cxx_build::bridge("src/lib.rs");

    let hl2sdk_dir = "vendor/hl2sdk/css";
    builder.include("include");
    builder.include(format!("{}/public", hl2sdk_dir));
    builder.include(format!("{}/public/tier0", hl2sdk_dir));
    builder.include(format!("{}/public/tier1", hl2sdk_dir));

    // Add compiler flags
    builder
        .flag("-m32")
        .flag("-Wno-unknown-pragmas")
        .flag("-fPIC")
        .flag("-msse")
        .flag("-DGNU")
        .flag("-DGNUC")
        .flag("-DLINUX")
        .flag("-D_LINUX")
        .flag("-DPOSIX")
        .flag("-std=gnu++17")
        .flag(format!("-L{}/lib/public/linux", hl2sdk_dir));

    // Add the bridge.cpp file
    builder.file("src/plugin.cpp");
    builder.file("src/plugin_bridge.cpp");

    // Compile the C++ code
    builder.compile("plugin_bridge");
    println!("cargo:rustc-link-lib=static=plugin_bridge");

    // Link against required libraries
    println!("cargo:rustc-link-search=native={}/lib/public/linux", hl2sdk_dir);
    println!("cargo:rustc-link-lib=tier0_srv");
    println!("cargo:rustc-link-lib=vstdlib_srv");
    println!("cargo:rustc-link-lib=dl");
    println!("cargo:rustc-link-lib=static:+verbatim=tier1_i486.a");
    println!("cargo:rustc-link-lib=static:+verbatim=mathlib_i486.a");
    //println!("cargo:rustc-link-lib=static=curl");
}
