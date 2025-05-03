fn main() {
    // patch hl2sdk
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let patch_source_dir = format!("{}/../patches/hl2sdk/css", manifest_dir);
    let patch_target_dir = format!("{}/../vendor/hl2sdk/css", manifest_dir);

    std::fs::remove_dir_all(&patch_target_dir).expect("failed to remove vendor submodules");

    std::process::Command::new("git")
        .arg("submodule").arg("update").arg("--init").arg("--recursive")
        .output().expect("failed to update submodules");

    std::process::Command::new("./apply-patches.sh")
        .arg(patch_source_dir).arg(&patch_target_dir)
        .output().expect("failed to apply patches");

    // cc::Build::new()
    //     .file("src/srcds.cpp")
    //     .include("include")
    //     .include("vendor/hl2sdk/css/public")
    //     .include("vendor/hl2sdk/css/public/tier0")
    //     .include("vendor/hl2sdk/css/public/tier1")
    //     .flag("-Wno-unknown-pragmas")
    //     .flag("-L./vendor/hl2sdk/css/lib/public/linux")
    //     .flag("-ltier0_srv")
    //     .flag("-l:tier1_i486.a")
    //     .flag("-l:mathlib_i486.a")
    //     .flag("-l:libcurl.a")
    //     .flag("-ldl")
    //     .flag("-m32")
    //     .cpp(true)
    //     .cpp_link_stdlib("stdc++") // use libstdc++
    //     .define("GNU", None)
    //     .define("GNUC", None)
    //     .define("LINUX", None)
    //     .define("_LINUX", None)
    //     .define("POSIX", None)
    //     .std("gnu++17")
    //     .target("i686-unknown-linux-gnu")
    //     .compile("srcds");

    // Compile the C++ code
    cc::Build::new()
        .cpp(true)
        .file("src/srcds.cpp")
        .include("include")
        .compile("srcds");

    // Tell cargo to tell rustc to link the C++ library
    println!("cargo:rustc-link-lib=static=srcds");

    // Tell cargo where to find the library
    let out_dir = std::env::var("OUT_DIR").unwrap();
    println!("cargo:rustc-link-search={}", out_dir);

    // Tell cargo to invalidate the built crate whenever the header changes
    println!("cargo:rerun-if-changed=include/srcds.h");
    println!("cargo:rerun-if-changed=src/srcds.cpp");
    println!("cargo:rerun-if-changed={}/**/*", &patch_target_dir);
    // println!("cargo:rerun-if-changed=src/lib.rs");
    // println!("cargo:rerun-if-changed=src/srcds.cpp");
    // println!("cargo:rerun-if-changed=include/srcds.h");
}
