#[cxx::bridge(namespace = "srcds::rs")]
mod ffi {
    // C++ types and signatures exposed to Rust.
    unsafe extern "C++" {
        include!("srcds-rs/srcds_rs.h");
    }
}

fn main() {
}
